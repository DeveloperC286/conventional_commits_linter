use crate::LintingError;

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

    pub(crate) fn lint(&self, allow_angular_type_only: bool) -> Vec<LintingError> {
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

        if allow_angular_type_only {
            match allow_angular_type_only::lint(&self.message) {
                Ok(()) => {}
                Err(linting_error) => {
                    linting_errors.push(linting_error);
                }
            }
        }

        linting_errors
    }
}

#[cfg(test)]
mod tests;
