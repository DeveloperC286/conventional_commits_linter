use std::collections::{HashMap, VecDeque};

use serde::Serialize;

use crate::commits::commit::Commit;
use crate::linting_error::LintingError;

#[derive(Serialize)]
struct LintingErrorsJSON {
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_hash: Option<String>,
    commit_message: String,
    linting_errors: Vec<LintingError>,
}

pub(crate) fn print_all(
    order: &VecDeque<Commit>,
    errors: &HashMap<Commit, Vec<LintingError>>,
) -> Result<String, serde_json::Error> {
    let mut linting_errors_json: Vec<LintingErrorsJSON> = vec![];

    for commit in order {
        if let Some(linting_errors) = errors.get(commit) {
            linting_errors_json.push(LintingErrorsJSON {
                commit_hash: commit.hash.clone(),
                commit_message: commit.message.clone(),
                linting_errors: linting_errors.clone(),
            });
        }
    }

    match serde_json::to_string(&linting_errors_json) {
        Ok(json) => Ok(json),
        Err(error) => {
            error!("{:?}", error);
            Err(error)
        }
    }
}

#[cfg(test)]
mod tests;