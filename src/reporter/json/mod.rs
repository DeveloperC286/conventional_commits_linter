use std::collections::HashMap;

use git2::Oid;
use serde::Serialize;
use std::process::exit;

use crate::model::{Commit, LintingError};

#[derive(Serialize)]
struct CommitLintingErrorsJSON {
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_hash: Option<String>,
    commit_message: String,
    linting_errors: Vec<LintingError>,
}

pub(crate) fn print(message: &str, linting_errors: &[LintingError]) -> String {
    match serde_json::to_string(&[CommitLintingErrorsJSON {
        commit_hash: None,
        commit_message: message.to_string(),
        linting_errors: linting_errors.to_vec(),
    }]) {
        Ok(json) => json,
        Err(error) => {
            error!("{:?}", error);
            error!("Unable to serialize a Commit's Linting Errors to JSON.");
            exit(crate::ERROR_EXIT_CODE);
        }
    }
}

pub(crate) fn print_all(
    commits: &[Commit],
    linting_errors: &HashMap<Oid, Vec<LintingError>>,
) -> String {
    let mut linting_errors_json: Vec<CommitLintingErrorsJSON> = vec![];

    for commit in commits {
        if let Some(linting_errors) = linting_errors.get(&commit.oid) {
            linting_errors_json.push(CommitLintingErrorsJSON {
                commit_hash: Some(commit.oid.to_string()),
                commit_message: commit.message.clone(),
                linting_errors: linting_errors.clone(),
            });
        }
    }

    match serde_json::to_string(&linting_errors_json) {
        Ok(json) => json,
        Err(error) => {
            error!("{:?}", error);
            error!("Unable to serialize every Commit's Linting Errors to a JSON array.");
            exit(crate::ERROR_EXIT_CODE);
        }
    }
}

#[cfg(test)]
mod tests;
