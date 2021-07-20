use std::collections::HashMap;

use console::Style;
use git2::Oid;

use crate::model::{Commit, LintingError};

pub fn pretty_print_linting_error(
    oid: Option<git2::Oid>,
    message: &str,
    linting_errors: &[LintingError],
) {
    let red = Style::new().red().bold();

    if let Some(oid) = oid {
        println!("{} - {}", red.apply_to("Commit Hash"), oid);
    }
    println!("{} - {:?}", red.apply_to("Message"), message);

    if linting_errors.contains(&LintingError::NonConventionalCommitsSpecification) {
        println!(
           "\t{} - Commit title does not comply with the Conventional Commits V1.0.0 specification.",
           red.apply_to("X")
       );
    }

    if linting_errors.contains(&LintingError::PrecedingWhitespace) {
        println!(
            "\t{} - Commit title has preceding whitespace characters.",
            red.apply_to("X")
        );
    }

    if linting_errors.contains(&LintingError::NonAngularType) {
        println!(
            "\t{} - Commit title does not use an Angular type.",
            red.apply_to("X")
        );
    }

    if linting_errors.contains(&LintingError::EmptyScope) {
        println!(
            "\t{} - Commit title has a scope which is empty.",
            red.apply_to("X")
        );
    }

    if linting_errors.contains(&LintingError::NoSpaceAfterColonPrecedingTypeAndScope) {
        println!(
            "\t{} - Commit title has no space after the colon preceding the Conventional Commits type and scope.",
            red.apply_to("X")
        );
    }

    if linting_errors.contains(&LintingError::NoDescriptionAfterTypeAndScope) {
        println!(
            "\t{} - Commit title has no description after the Conventional Commits type and scope.",
            red.apply_to("X")
        );
    }

    println!();
}

pub fn pretty_print_linting_errors(
    commits: &[Commit],
    linting_errors: &HashMap<Oid, Vec<LintingError>>,
) {
    let red = Style::new().red().bold();

    for commit in commits {
        if linting_errors.contains_key(&commit.oid) {
            pretty_print_linting_error(
                Some(commit.oid),
                &commit.message,
                &linting_errors.get(&commit.oid).unwrap(),
            );
        }
    }

    let number_of_issues: usize = linting_errors.values().map(|x| x.len()).sum();

    println!(
        "{} - Found {} separate linting issues across {} commits.",
        red.apply_to("X"),
        number_of_issues,
        linting_errors.len()
    );
}
