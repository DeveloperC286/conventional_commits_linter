use std::collections::HashMap;

use ansi_term::Colour::Red;
use git2::Oid;

use serde::Serialize;

use crate::model::{Commit, LintingError};

pub fn pretty_print_linting_error(
    oid: Option<git2::Oid>,
    message: &str,
    linting_errors: &[LintingError],
) -> String {
    let mut pretty_print = String::new();
    let red = Red.bold();

    if let Some(oid) = oid {
        pretty_print.push_str(&format!("{} - {}\n", red.paint("Commit Hash"), oid));
    }
    pretty_print.push_str(&format!("{} - {:?}\n", red.paint("Message"), message));

    if linting_errors.contains(&LintingError::NonConventionalCommitsSpecification) {
        pretty_print.push_str(&format!(
           "\t{} - Commit title does not comply with the Conventional Commits V1.0.0 specification.\n",
           red.paint("X")
       ));
    }

    if linting_errors.contains(&LintingError::PrecedingWhitespace) {
        pretty_print.push_str(&format!(
            "\t{} - Commit title has preceding whitespace characters.\n",
            red.paint("X")
        ));
    }

    if linting_errors.contains(&LintingError::NonAngularType) {
        pretty_print.push_str(&format!(
            "\t{} - Commit title does not use an Angular type.\n",
            red.paint("X")
        ));
    }

    if linting_errors.contains(&LintingError::EmptyScope) {
        pretty_print.push_str(&format!(
            "\t{} - Commit title has a scope which is empty.\n",
            red.paint("X")
        ));
    }

    if linting_errors.contains(&LintingError::NoSpaceAfterColonPrecedingTypeAndScope) {
        pretty_print.push_str(&format!(
            "\t{} - Commit title has no space after the colon preceding the Conventional Commits type and scope.\n",
            red.paint("X")
        ));
    }

    if linting_errors.contains(&LintingError::NoDescriptionAfterTypeAndScope) {
        pretty_print.push_str(&format!(
            "\t{} - Commit title has no description after the Conventional Commits type and scope.\n",
            red.paint("X")
        ));
    }

    pretty_print.push('\n');
    pretty_print
}

pub fn pretty_print_linting_errors(
    commits: &[Commit],
    linting_errors: &HashMap<Oid, Vec<LintingError>>,
) -> String {
    let mut pretty_print = String::new();

    for commit in commits {
        if linting_errors.contains_key(&commit.oid) {
            pretty_print.push_str(&pretty_print_linting_error(
                Some(commit.oid),
                &commit.message,
                &linting_errors.get(&commit.oid).unwrap(),
            ));
        }
    }

    let red = Red.bold();
    let number_of_issues: usize = linting_errors.values().map(|x| x.len()).sum();

    pretty_print.push_str(&format!(
        "{} - Found {} separate linting issues across {} commits.",
        red.paint("X"),
        number_of_issues,
        linting_errors.len()
    ));
    pretty_print
}

pub fn json_print_linting_error(message: &str, linting_errors: &[LintingError]) -> String {
    serde_json::to_string(&[LintingErrorJSON {
        commit_hash: None,
        commit_message: message.to_string(),
        linting_errors: linting_errors.to_vec(),
    }])
    .unwrap()
}

pub fn json_print_linting_errors(
    commits: &[Commit],
    linting_errors: &HashMap<Oid, Vec<LintingError>>,
) -> String {
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
