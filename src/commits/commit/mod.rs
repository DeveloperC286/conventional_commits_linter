use std::sync::OnceLock;

use regex::Regex;

use crate::commit_type::CommitType;
use crate::linting_error::LintingError;

mod allow_angular_type_only;
mod constants;
mod conventional_commits_specification;

use self::constants::*;

#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) struct Commit {
    pub(crate) hash: Option<String>,
    pub(crate) message: String,
}

impl Commit {
    pub(crate) fn from_commit_message<T: Into<String>>(message: T) -> Commit {
        Commit {
            hash: None,
            message: message.into(),
        }
    }

    pub(crate) fn from_git(commit: &git2::Commit) -> Commit {
        let message = match commit.message().map(|m| m.to_string()) {
            Some(message) => {
                trace!(
                    "Found the commit message {message:?} for the commit with the hash '{}'.",
                    commit.id()
                );

                message
            }
            None => {
                warn!(
                    "Can not find commit message for the commit with the hash '{}'.",
                    commit.id()
                );

                String::new()
            }
        };

        Commit {
            hash: Some(commit.id().to_string()),
            message,
        }
    }

    pub(crate) fn lint(
        &self,
        commit_type: &CommitType,
        max_commit_title_length: usize,
        lowercase_scope: bool,
    ) -> Vec<LintingError> {
        info!("Linting the commit message {:?}.", self.message);
        let mut linting_errors = vec![];

        match conventional_commits_specification::lint(&self.message) {
            Ok(()) => {}
            Err(linting_error) => {
                linting_errors.push(linting_error);

                match conventional_commits_specification::preceding_whitespace::lint(&self.message)
                {
                    Ok(()) => {}
                    Err(linting_error) => {
                        linting_errors.push(linting_error);
                    }
                }

                match conventional_commits_specification::empty_scope::lint(&self.message) {
                    Ok(()) => {}
                    Err(linting_error) => {
                        linting_errors.push(linting_error);
                    }
                }

                match conventional_commits_specification::exclamation_mark_before_scope::lint(
                    &self.message,
                ) {
                    Ok(()) => {}
                    Err(linting_error) => {
                        linting_errors.push(linting_error);
                    }
                }

                match conventional_commits_specification::no_space_after_colon_preceding_type_and_scope::lint(&self.message) {
                Ok(()) => {}
                Err(linting_error) => {
                    linting_errors.push(linting_error);
                }
            }

                match conventional_commits_specification::no_description_after_type_and_scope::lint(
                    &self.message,
                ) {
                    Ok(()) => {}
                    Err(linting_error) => {
                        linting_errors.push(linting_error);
                    }
                }
            }
        }

        if *commit_type == CommitType::Angular {
            match allow_angular_type_only::lint(&self.message) {
                Ok(()) => {}
                Err(linting_error) => {
                    linting_errors.push(linting_error);
                }
            }
        }

        if lowercase_scope {
            match conventional_commits_specification::lowercase_scope::lint(&self.message) {
                Ok(()) => {}
                Err(linting_error) => {
                    linting_errors.push(linting_error);
                }
            }
        }

        // Check message length regardless of conventional commits compliance
        let max_length = if max_commit_title_length == 0 {
            None
        } else {
            Some(max_commit_title_length)
        };
        match conventional_commits_specification::message_length::lint(&self.message, max_length) {
            Ok(()) => {}
            Err(linting_error) => {
                linting_errors.push(linting_error);
            }
        }

        linting_errors
    }
}

#[cfg(test)]
mod tests;
