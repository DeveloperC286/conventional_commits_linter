use std::collections::{HashMap, VecDeque};

use git2::{Oid, Repository, Revwalk};

use crate::commits::commit::Commit;
use crate::source::Source;
use crate::LintingError;
use crate::LintingErrors;

pub mod commit;

/// A representation of a range of commits within a Git repository.
#[cfg(not(test))]
pub struct Commits {
    commits: VecDeque<Commit>,
    source: Source,
}

#[cfg(test)]
pub struct Commits {
    pub(crate) commits: VecDeque<Commit>,
    pub(crate) source: Source,
}

impl Commits {
    /// Create a new range of commits containing a singular commit created from the commit_message.
    ///
    /// This functionality is intended to allow you to lint a commit message before creating the
    /// commit, e.g. in a Git hook etc.
    ///
    ///```
    ///use conventional_commits_linter_lib::Commits;
    ///
    ///let commits = Commits::from_commit_message("feat: adding stdin support");
    ///```
    pub fn from_commit_message<T: Into<String>>(commit_message: T) -> Commits {
        Commits {
            commits: VecDeque::from(vec![Commit::from_commit_message(commit_message)]),
            source: Source::CommitMessage,
        }
    }

    /// Create a new range of commits from a reference exclusively from the commit specified by the reference till inclusively of `HEAD`.
    ///
    /// Supports providing either the full or short name of the reference.
    ///
    /// E.g. short name.
    ///
    /// ```
    /// use conventional_commits_linter_lib::Commits;
    /// use git2::Repository;
    ///
    /// let repository = Repository::open_from_env().unwrap();
    /// let commits = Commits::from_reference(&repository, "0.9.0").unwrap();
    /// ```
    ///
    /// E.g. full name.
    ///
    /// ```
    /// use conventional_commits_linter_lib::Commits;
    /// use git2::Repository;
    ///
    /// let repository = Repository::open_from_env().unwrap();
    /// let commits = Commits::from_reference(&repository, "refs/tags/0.9.0").unwrap();
    /// ```
    pub fn from_reference<T: AsRef<str>>(
        repository: &Repository,
        reference: T,
    ) -> Result<Commits, git2::Error> {
        let reference_oid = get_reference_oid(repository, reference.as_ref())?;
        let commits = get_commits_till_head_from_oid(repository, reference_oid)?;
        Ok(Commits {
            commits,
            source: Source::Git,
        })
    }

    /// Create a new range of commits from a commit hash exclusively from the commit specified till inclusively of `HEAD`.
    ///
    /// Supports providing either the full commit hash or a shortened commit hash.
    ///
    /// E.g. shortened commit hash.
    ///
    /// ```
    /// use conventional_commits_linter_lib::Commits;
    /// use git2::Repository;
    ///
    /// let repository = Repository::open_from_env().unwrap();
    /// let commits = Commits::from_commit_hash(&repository, "0d12672").unwrap();
    /// ```
    ///
    /// E.g. full commit hash.
    ///
    /// ```
    /// use conventional_commits_linter_lib::Commits;
    /// use git2::Repository;
    ///
    /// let repository = Repository::open_from_env().unwrap();
    /// let commits = Commits::from_commit_hash(&repository, "0d126721af479dfd9ed2a5960c4202a87cfd4932").unwrap();
    /// ```
    pub fn from_commit_hash<T: AsRef<str>>(
        repository: &Repository,
        commit_hash: T,
    ) -> Result<Commits, git2::Error> {
        let commit_oid = parse_to_oid(repository, commit_hash.as_ref())?;
        let commits = get_commits_till_head_from_oid(repository, commit_oid)?;
        Ok(Commits {
            commits,
            source: Source::Git,
        })
    }

    /// Lint the range of commits using the provided settings.
    ///
    /// If any linting errors are found then an error containing the linting errors is returned.
    ///
    /// ```
    ///use conventional_commits_linter_lib::Commits;
    ///
    ///let commits = Commits::from_commit_message("feat: adding stdin support");
    ///let linting_result = commits.lint(true);
    ///```
    pub fn lint(self, allow_angular_type_only: bool) -> Result<(), LintingErrors> {
        let mut errors: HashMap<Commit, Vec<LintingError>> = HashMap::new();

        for commit in self.commits.iter().cloned() {
            let commit_errors = commit.lint(allow_angular_type_only);

            if !commit_errors.is_empty() {
                errors.insert(commit, commit_errors);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(LintingErrors::from(self.source, self.commits, errors))
        }
    }
}

fn get_commits_till_head_from_oid(
    repository: &Repository,
    from_commit_hash: Oid,
) -> Result<VecDeque<Commit>, git2::Error> {
    fn get_revwalker(
        repository: &Repository,
        from_commit_hash: Oid,
    ) -> Result<Revwalk, git2::Error> {
        let mut commits = repository.revwalk()?;
        commits.push_head()?;

        match commits.hide(from_commit_hash) {
            Ok(_) => Ok(commits),
            Err(error) => {
                error!("Can not find a commit with the hash '{from_commit_hash}'.");
                Err(error)
            }
        }
    }

    let revwalker = get_revwalker(repository, from_commit_hash)?;
    let mut commits = VecDeque::new();

    for oid in revwalker {
        let oid = oid?;
        let commit = repository.find_commit(oid)?;
        let commit = Commit::from_git(&commit);
        commits.push_front(commit);
    }

    if commits.is_empty() {
        let error_message = "No Git commits within the provided range.".to_string();
        error!("{error_message}");
        Err(git2::Error::from_str(&error_message))
    } else {
        debug!("Operating upon {} commits.", commits.len());
        Ok(commits)
    }
}

fn get_reference_oid(repository: &Repository, matching: &str) -> Result<Oid, git2::Error> {
    match repository.resolve_reference_from_short_name(matching) {
        Ok(reference) => {
            trace!(
                "Matched {matching:?} to the reference {:?}.",
                reference.name().unwrap()
            );
            let commit = reference.peel_to_commit()?;
            Ok(commit.id())
        }
        Err(error) => {
            error!("Could not find a reference with the name {matching:?}.");
            Err(error)
        }
    }
}

fn parse_to_oid(repository: &Repository, oid: &str) -> Result<Oid, git2::Error> {
    match oid.len() {
        1..=39 => {
            trace!("Attempting to find a match for the short commit hash {oid:?}.");
            let matching_oid_lowercase = oid.to_lowercase();

            let mut revwalker = repository.revwalk()?;
            revwalker.push_head()?;

            let matched_commit_hashes: Vec<Oid> = revwalker
                .into_iter()
                .map(|result| match result {
                    Ok(oid) => {
                        let oid_lowercase = oid.to_string().to_lowercase();

                        if oid_lowercase.starts_with(&matching_oid_lowercase) {
                            return Some(oid);
                        }

                        None
                    }
                    Err(error) => {
                        error!("{error:?}");
                        None
                    }
                })
                .flatten()
                .collect();

            match matched_commit_hashes.len() {
                0 => {
                    let error_message = format!(
                        "No commit hashes start with the provided short commit hash {matching_oid_lowercase:?}."
                    );
                    error!("{error_message}");
                    Err(git2::Error::from_str(&error_message))
                }
                1 => Ok(*matched_commit_hashes.first().unwrap()),
                _ => {
                    let error_message = format!("Ambiguous short commit hash, the commit hashes {matched_commit_hashes:?} all start with the provided short commit hash {matching_oid_lowercase:?}.");
                    error!("{error_message}");
                    Err(git2::Error::from_str(&error_message))
                }
            }
        }
        _ => match git2::Oid::from_str(oid) {
            Ok(oid) => Ok(oid),
            Err(error) => {
                error!("{oid:?} is not a valid commit hash.");
                Err(error)
            }
        },
    }
}
