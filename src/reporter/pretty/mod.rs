use std::collections::HashMap;

use ansi_term::Colour::Red;
use git2::Oid;

use crate::model::{Commit, LintingError};

pub(crate) fn print(
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

pub(crate) fn print_all(
    commits: &[Commit],
    linting_errors: &HashMap<Oid, Vec<LintingError>>,
) -> String {
    let mut pretty_print = String::new();

    for commit in commits {
        if linting_errors.contains_key(&commit.oid) {
            pretty_print.push_str(&print(
                Some(commit.oid),
                &commit.message,
                linting_errors.get(&commit.oid).unwrap(),
            ));
        }
    }

    let red = Red.bold();
    let total_linting_errors: usize = linting_errors.values().map(|x| x.len()).sum();

    pretty_print.push_str(&format!(
        "{} - Found {} separate linting errors across {} commits.",
        red.paint("X"),
        total_linting_errors,
        linting_errors.len()
    ));
    pretty_print
}

#[cfg(test)]
mod tests;
