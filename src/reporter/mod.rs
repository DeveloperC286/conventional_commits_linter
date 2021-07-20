use std::collections::HashMap;

use console::Style;
use git2::Oid;

use crate::model::{Commit, LintingError};

pub fn pretty_print_linting_error(
    oid: Option<git2::Oid>,
    message: &str,
    linting_errors: &[LintingError],
) -> String {
    let mut pretty_print = String::new();
    let red = Style::new().red().bold();

    if let Some(oid) = oid {
        pretty_print.push_str(&format!("{} - {}\n", red.apply_to("Commit Hash"), oid));
    }
    pretty_print.push_str(&format!("{} - {:?}\n", red.apply_to("Message"), message));

    if linting_errors.contains(&LintingError::NonConventionalCommitsSpecification) {
        pretty_print.push_str(&format!(
           "\t{} - Commit title does not comply with the Conventional Commits V1.0.0 specification.\n",
           red.apply_to("X")
       ));
    }

    if linting_errors.contains(&LintingError::PrecedingWhitespace) {
        pretty_print.push_str(&format!(
            "\t{} - Commit title has preceding whitespace characters.\n",
            red.apply_to("X")
        ));
    }

    if linting_errors.contains(&LintingError::NonAngularType) {
        pretty_print.push_str(&format!(
            "\t{} - Commit title does not use an Angular type.\n",
            red.apply_to("X")
        ));
    }

    if linting_errors.contains(&LintingError::EmptyScope) {
        pretty_print.push_str(&format!(
            "\t{} - Commit title has a scope which is empty.\n",
            red.apply_to("X")
        ));
    }

    if linting_errors.contains(&LintingError::NoSpaceAfterColonPrecedingTypeAndScope) {
        pretty_print.push_str(&format!(
            "\t{} - Commit title has no space after the colon preceding the Conventional Commits type and scope.\n",
            red.apply_to("X")
        ));
    }

    if linting_errors.contains(&LintingError::NoDescriptionAfterTypeAndScope) {
        pretty_print.push_str(&format!(
            "\t{} - Commit title has no description after the Conventional Commits type and scope.\n",
            red.apply_to("X")
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

    let red = Style::new().red().bold();
    let number_of_issues: usize = linting_errors.values().map(|x| x.len()).sum();

    pretty_print.push_str(&format!(
        "{} - Found {} separate linting issues across {} commits.",
        red.apply_to("X"),
        number_of_issues,
        linting_errors.len()
    ));
    pretty_print
}
