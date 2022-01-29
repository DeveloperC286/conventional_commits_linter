use crate::LintingError;

mod allow_angular_type_only;
mod conventional_commits_specification;
mod regex;

pub(crate) fn lint_commit_message<T: AsRef<str>>(
    commit_message: T,
    allow_angular_type_only: bool,
) -> Vec<LintingError> {
    let mut linting_errors = vec![];

    match conventional_commits_specification::lint(commit_message.as_ref()) {
        Ok(()) => {}
        Err(linting_error) => {
            linting_errors.push(linting_error);

            match conventional_commits_specification::preceding_whitespace::lint(
                commit_message.as_ref(),
            ) {
                Ok(()) => {}
                Err(linting_error) => {
                    linting_errors.push(linting_error);
                }
            }

            match conventional_commits_specification::empty_scope::lint(commit_message.as_ref()) {
                Ok(()) => {}
                Err(linting_error) => {
                    linting_errors.push(linting_error);
                }
            }

            match conventional_commits_specification::no_space_after_colon_preceding_type_and_scope::lint(commit_message.as_ref()) {
                Ok(()) => {}
                Err(linting_error) => {
                    linting_errors.push(linting_error);
                }
            }

            match conventional_commits_specification::no_description_after_type_and_scope::lint(
                commit_message.as_ref(),
            ) {
                Ok(()) => {}
                Err(linting_error) => {
                    linting_errors.push(linting_error);
                }
            }
        }
    }

    if allow_angular_type_only {
        match allow_angular_type_only::lint(commit_message.as_ref()) {
            Ok(()) => {}
            Err(linting_error) => {
                linting_errors.push(linting_error);
            }
        }
    }

    linting_errors
}

#[cfg(test)]
mod tests;
