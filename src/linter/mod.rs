use std::collections::HashMap;

use git2::Oid;

use crate::model::{Commit, LintingError};

mod allow_angular_type_only;
mod conventional_commits_specification;
mod regex;

pub fn lint_commits(
    commits: &[Commit],
    allow_angular_type_only: bool,
) -> HashMap<Oid, Vec<LintingError>> {
    let mut oid_to_linting_errors = HashMap::new();

    for commit in commits {
        let linting_errors = lint_commit(commit, allow_angular_type_only);

        if !linting_errors.is_empty() {
            oid_to_linting_errors.insert(commit.oid, linting_errors);
        }
    }

    oid_to_linting_errors
}

pub fn lint_commit(commit: &Commit, allow_angular_type_only: bool) -> Vec<LintingError> {
    let mut linting_errors = vec![];

    match conventional_commits_specification::lint(&commit.message) {
        Ok(()) => {}
        Err(linting_error) => {
            linting_errors.push(linting_error);

            match conventional_commits_specification::empty_scope::lint(&commit.message) {
                Ok(()) => {}
                Err(linting_error) => {
                    linting_errors.push(linting_error);
                }
            }

            match conventional_commits_specification::preceding_whitespace::lint(&commit.message) {
                Ok(()) => {}
                Err(linting_error) => {
                    linting_errors.push(linting_error);
                }
            }

            match conventional_commits_specification::no_description::lint(&commit.message) {
                Ok(()) => {}
                Err(linting_error) => {
                    linting_errors.push(linting_error);
                }
            }

            match conventional_commits_specification::no_space_after_type::lint(&commit.message) {
                Ok(()) => {}
                Err(linting_error) => {
                    linting_errors.push(linting_error);
                }
            }
        }
    }

    if allow_angular_type_only {
        match allow_angular_type_only::lint(&commit.message) {
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
