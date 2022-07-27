use std::collections::{HashMap, VecDeque};
use std::fmt::Write;

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
            let _ = writeln!(pretty_print, "{} - {commit_hash}", red.paint("Commit Hash"));
        }

        let _ = writeln!(
            pretty_print,
            "{} - {:?}",
            red.paint("Message"),
            commit.message
        );

        if linting_errors.contains(&LintingError::NonConventionalCommitsSpecification) {
            let _ = writeln!(pretty_print,
            "\t{} - Commit title does not comply with the Conventional Commits V1.0.0 specification.",
            red.paint("X")
        );
        }

        if linting_errors.contains(&LintingError::PrecedingWhitespace) {
            let _ = writeln!(
                pretty_print,
                "\t{} - Commit title has preceding whitespace characters.",
                red.paint("X")
            );
        }

        if linting_errors.contains(&LintingError::NonAngularType) {
            let _ = writeln!(
                pretty_print,
                "\t{} - Commit title does not use an Angular type.",
                red.paint("X")
            );
        }

        if linting_errors.contains(&LintingError::ExclamationMarkBeforeScope) {
            let _ = writeln!(
                pretty_print,
                "\t{} - Commit title has a exclamation mark before the scope.",
                red.paint("X")
            );
        }

        if linting_errors.contains(&LintingError::EmptyScope) {
            let _ = writeln!(
                pretty_print,
                "\t{} - Commit title has a scope which is empty.",
                red.paint("X")
            );
        }

        if linting_errors.contains(&LintingError::NoSpaceAfterColonPrecedingTypeAndScope) {
            let _ = writeln!(
                pretty_print,
                "\t{} - Commit title has no space after the colon preceding the type and scope.",
                red.paint("X")
            );
        }

        if linting_errors.contains(&LintingError::NoDescriptionAfterTypeAndScope) {
            let _ = writeln!(
                pretty_print,
                "\t{} - Commit title has no description after the type and scope.",
                red.paint("X")
            );
        }

        pretty_print
    }

    let mut pretty_print = String::new();

    for commit in order.iter() {
        if let Some(errors) = errors.get(commit) {
            let _ = writeln!(pretty_print, "{}", print(commit, errors));
        }
    }

    if source == Source::Git {
        let red = Red.bold();
        let total_linting_errors: usize = errors.values().map(|x| x.len()).sum();

        let _ = write!(
            pretty_print,
            "{} - Found {total_linting_errors} separate linting errors across {} commits.",
            red.paint("X"),
            errors.len()
        );
    }

    pretty_print
}

#[cfg(test)]
mod tests;
