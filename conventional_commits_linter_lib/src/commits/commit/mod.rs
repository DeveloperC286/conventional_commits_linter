use crate::LintingError;

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
        crate::linter::lint_commit_message(&self.message, allow_angular_type_only)
    }
}
