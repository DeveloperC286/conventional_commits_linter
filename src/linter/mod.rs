mod allow_angular_type_only;
mod conventional_commits_specification;

use crate::model::{Commit, LintingError};
use git2::Oid;
use std::collections::HashMap;

pub fn lint_commits(
    commits: Vec<Commit>,
    allow_angular_type_only: bool,
) -> HashMap<Oid, Vec<LintingError>> {
    let mut linting_errors = HashMap::new();

    for commit in commits {
        match conventional_commits_specification::lint(&commit.message) {
            Ok(()) => {}
            Err(linting_error) => {
                linting_errors = add_to_linting_errors(linting_errors, commit.oid, linting_error);
            }
        }

        if allow_angular_type_only {
            match allow_angular_type_only::lint(&commit.message) {
                Ok(()) => {}
                Err(linting_error) => {
                    linting_errors =
                        add_to_linting_errors(linting_errors, commit.oid, linting_error);
                }
            }
        }
    }

    linting_errors
}

fn add_to_linting_errors(
    mut linting_errors: HashMap<Oid, Vec<LintingError>>,
    oid: Oid,
    linting_error: LintingError,
) -> HashMap<Oid, Vec<LintingError>> {
    linting_errors
        .entry(oid)
        .or_insert(vec![])
        .push(linting_error);

    linting_errors
}
