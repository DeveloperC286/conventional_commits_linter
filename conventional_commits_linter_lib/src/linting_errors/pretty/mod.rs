use std::collections::{HashMap, VecDeque};

use ansi_term::Colour::Red;

use crate::commits::commit::Commit;
use crate::source::Source;
use crate::LintingError;

pub(crate) fn print_all(
    source: Source,
    order: &VecDeque<Commit>,
    errors: &HashMap<Commit, Vec<LintingError>>,
) -> String {
    fn print(commit: &Commit, linting_errors: &[LintingError]) -> String {
        let mut pretty_print = String::new();
        let red = Red.bold();

        if let Some(commit_hash) = &commit.hash {
            pretty_print.push_str(&format!("{} - {}\n", red.paint("Commit Hash"), commit_hash));
        }

        pretty_print.push_str(&format!(
            "{} - {:?}\n",
            red.paint("Message"),
            commit.message
        ));

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

    let mut pretty_print = String::new();

    for commit in order.iter() {
        if let Some(errors) = errors.get(commit) {
            pretty_print.push_str(&print(commit, errors));
        }
    }

    if source == Source::Git {
        let red = Red.bold();
        let total_linting_errors: usize = errors.values().map(|x| x.len()).sum();

        pretty_print.push_str(&format!(
            "{} - Found {} separate linting errors across {} commits.",
            red.paint("X"),
            total_linting_errors,
            errors.len()
        ));
    }

    pretty_print
}

#[cfg(test)]
mod tests;
