use std::collections::HashMap;

use git2::Oid;

use serde::Serialize;

use crate::model::{Commit, LintingError};

pub fn print(message: &str, linting_errors: &[LintingError]) -> String {
    serde_json::to_string(&[LintingErrorJSON {
        commit_hash: None,
        commit_message: message.to_string(),
        linting_errors: linting_errors.to_vec(),
    }])
    .unwrap()
}

pub fn print_all(commits: &[Commit], linting_errors: &HashMap<Oid, Vec<LintingError>>) -> String {
    let mut linting_errors_json: Vec<LintingErrorJSON> = vec![];

    for commit in commits {
        if let Some(linting_errors) = linting_errors.get(&commit.oid) {
            linting_errors_json.push(LintingErrorJSON {
                commit_hash: Some(commit.oid.to_string()),
                commit_message: commit.message.clone(),
                linting_errors: linting_errors.clone(),
            });
        }
    }

    serde_json::to_string(&linting_errors_json).unwrap()
}

#[derive(Serialize)]
struct LintingErrorJSON {
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_hash: Option<String>,
    commit_message: String,
    linting_errors: Vec<LintingError>,
}

#[cfg(test)]
mod tests;
