mod allow_angular_type_only;
mod conventional_commits_specification;

use crate::model::{Commit, LintingError};
use git2::Oid;
use std::collections::HashMap;

pub fn lint_commits(
    commits: &Vec<Commit>,
    allow_angular_type_only: bool,
) -> HashMap<Oid, Vec<LintingError>> {
    let mut oid_to_linting_errors = HashMap::new();

    for commit in commits {
        let mut linting_errors = vec![];

        match conventional_commits_specification::lint(&commit.message) {
            Ok(()) => {}
            Err(linting_error) => {
                linting_errors.push(linting_error);
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

        if !linting_errors.is_empty() {
            oid_to_linting_errors.insert(commit.oid, linting_errors);
        }
    }

    oid_to_linting_errors
}
