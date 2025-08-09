use std::collections::{HashMap, VecDeque};

use anyhow::{bail, Context, Result};
use git2::{Oid, Repository, Revwalk};

use crate::commit_type::CommitType;
use crate::commits::commit::Commit;
use crate::history_mode::HistoryMode;
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

    pub fn from_git<T: AsRef<str>>(
        repository: &Repository,
        git: T,
        history_mode: HistoryMode,
    ) -> Result<Commits> {
        let oid = parse_to_oid(repository, git.as_ref()).or_else(|error| {
            get_reference_oid(repository, git.as_ref()).map_err(|e| error.context(e))
        })?;

        let commits = get_commits_till_head_from_oid(repository, oid, history_mode)?;
        Ok(Commits {
            commits,
            source: Source::Git,
        })
    }

    pub fn lint(
        self,
        commit_type: &CommitType,
        max_commit_title_length: usize,
    ) -> Option<LintingErrors> {
        let mut errors: HashMap<Commit, Vec<LintingError>> = HashMap::new();

        for commit in self.commits.iter().cloned() {
            let commit_errors = commit.lint(commit_type, max_commit_title_length);

            if !commit_errors.is_empty() {
                warn!(
                    "Found {} linting errors for the commit {:?}.",
                    commit_errors.len(),
                    commit.hash
                );
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
    history_mode: HistoryMode,
) -> Result<VecDeque<Commit>> {
    fn get_revwalker(
        repository: &Repository,
        from_commit_hash: Oid,
        history_mode: HistoryMode,
    ) -> Result<Revwalk<'_>> {
        let mut commits = repository.revwalk()?;
        if history_mode == HistoryMode::First {
            commits.simplify_first_parent()?;
        }
        commits.push_head()?;

        commits.hide(from_commit_hash).context(format!(
            "Can not find a commit with the hash '{from_commit_hash}'."
        ))?;
        Ok(commits)
    }

    let revwalker = get_revwalker(repository, from_commit_hash, history_mode)?;
    let mut commits = VecDeque::new();

    for oid in revwalker {
        let oid = oid?;
        let commit = repository.find_commit(oid)?;
        let commit = Commit::from_git(&commit);
        commits.push_front(commit);
    }

    if commits.is_empty() {
        bail!("No Git commits within the provided range.");
    }

    info!("Found {} commits within the provided range.", commits.len());
    Ok(commits)
}

fn get_reference_oid(repository: &Repository, matching: &str) -> Result<Oid> {
    let reference = repository
        .resolve_reference_from_short_name(matching)
        .context(format!(
            "Could not find a reference with the name {matching:?}."
        ))?;
    debug!(
        "Matched {:?} to the reference {:?}.",
        matching,
        reference.name().unwrap()
    );
    let commit = reference.peel_to_commit()?;
    Ok(commit.id())
}

fn parse_to_oid(repository: &Repository, oid: &str) -> Result<Oid> {
    match oid.len() {
        1..=39 => {
            debug!("Attempting to find a match for the short commit hash {oid:?}.");
            let matching_oid_lowercase = oid.to_lowercase();

            let mut revwalker = repository.revwalk()?;
            revwalker.push_head()?;

            let matched_commit_hashes: Vec<Oid> = revwalker
                .filter_map(|result| match result {
                    Ok(oid) => {
                        let oid_lowercase = oid.to_string().to_lowercase();

                        if oid_lowercase.starts_with(&matching_oid_lowercase) {
                            debug!("Found a match for the short commit hash {oid:?}.");
                            return Some(oid);
                        }

                        None
                    }
                    Err(_) => None,
                })
                .collect();

            match matched_commit_hashes.len() {
                0 => {
                    bail!(
                        "No commit hashes start with the provided short commit hash {:?}.",
                        matching_oid_lowercase
                    );
                }
                1 => Ok(*matched_commit_hashes.first().unwrap()),
                _ => {
                    bail!("Ambiguous short commit hash, the commit hashes {:?} all start with the provided short commit hash {:?}.", matched_commit_hashes, matching_oid_lowercase);
                }
            }
        }
        _ => git2::Oid::from_str(oid).context(format!("{oid:?} is not a valid commit hash.")),
    }
}
