use std::collections::{HashMap, VecDeque};

use git2::{Oid, Repository, Revwalk};

use crate::commits::commit::Commit;
use crate::git_history_mode::GitHistoryMode;
use crate::linting_error::LintingError;
use crate::linting_errors::LintingErrors;
use crate::source::Source;

pub mod commit;

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
    pub fn from_commit_message<T: Into<String>>(commit_message: T) -> Commits {
        Commits {
            commits: VecDeque::from(vec![Commit::from_commit_message(commit_message)]),
            source: Source::CommitMessage,
        }
    }

    pub fn from_reference<T: AsRef<str>>(
        repository: &Repository,
        reference: T,
        git_history_mode: GitHistoryMode,
    ) -> Result<Commits, git2::Error> {
        let reference_oid = get_reference_oid(repository, reference.as_ref())?;
        let commits = get_commits_till_head_from_oid(repository, reference_oid, git_history_mode)?;
        Ok(Commits {
            commits,
            source: Source::Git,
        })
    }

    pub fn from_commit_hash<T: AsRef<str>>(
        repository: &Repository,
        commit_hash: T,
        git_history_mode: GitHistoryMode,
    ) -> Result<Commits, git2::Error> {
        let commit_oid = parse_to_oid(repository, commit_hash.as_ref())?;
        let commits = get_commits_till_head_from_oid(repository, commit_oid, git_history_mode)?;
        Ok(Commits {
            commits,
            source: Source::Git,
        })
    }

    pub fn lint(self, allow_angular_type_only: bool) -> Option<LintingErrors> {
        let mut errors: HashMap<Commit, Vec<LintingError>> = HashMap::new();

        for commit in self.commits.iter().cloned() {
            let commit_errors = commit.lint(allow_angular_type_only);

            if !commit_errors.is_empty() {
                errors.insert(commit, commit_errors);
            }
        }

        if errors.is_empty() {
            None
        } else {
            Some(LintingErrors::from(self.source, self.commits, errors))
        }
    }
}

fn get_commits_till_head_from_oid(
    repository: &Repository,
    from_commit_hash: Oid,
    git_history_mode: GitHistoryMode,
) -> Result<VecDeque<Commit>, git2::Error> {
    fn get_revwalker(
        repository: &Repository,
        from_commit_hash: Oid,
        git_history_mode: GitHistoryMode,
    ) -> Result<Revwalk, git2::Error> {
        let mut commits = repository.revwalk()?;
        if git_history_mode == GitHistoryMode::FirstParent {
            commits.simplify_first_parent()?;
        }
        commits.push_head()?;

        match commits.hide(from_commit_hash) {
            Ok(_) => Ok(commits),
            Err(error) => {
                error!("Can not find a commit with the hash '{from_commit_hash}'.");
                Err(error)
            }
        }
    }

    let revwalker = get_revwalker(repository, from_commit_hash, git_history_mode)?;
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
                .filter_map(|result| match result {
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
