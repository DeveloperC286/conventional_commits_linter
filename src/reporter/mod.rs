use std::collections::HashMap;

use console::Style;
use git2::Oid;

use crate::model::{Commit, LintingError};

pub fn print_summary(linting_errors: &HashMap<Oid, Vec<LintingError>>) {
    let number_of_issues: usize = linting_errors.values().map(|x| x.len()).sum();
    let red = Style::new().red().bold();

    println!(
        "{} - Found {} separate linting issues across {} commits.",
        red.apply_to("X"),
        number_of_issues,
        linting_errors.len()
    );
}

pub fn print_linting_errors(commits: &[Commit], linting_errors: &HashMap<Oid, Vec<LintingError>>) {
    let red = Style::new().red().bold();

    for commit in commits {
        if linting_errors.contains_key(&commit.oid) {
            println!("{} - {}", red.apply_to("Commit Hash"), commit.oid);
            println!("{} - {:?}", red.apply_to("Message"), commit.message);

            for linting_error in linting_errors.get(&commit.oid).unwrap() {
                match linting_error {
                    LintingError::NON_ANGULAR_TYPE => {
                        println!(
                            "\t{} - Commit title does not use an Angular type.",
                            red.apply_to("X")
                        );
                    }
                    LintingError::NON_CONVENTIONAL_COMMITS_SPECIFICATION => {
                        println!(
                            "\t{} - Commit title does not comply with the Conventional Commits V1.0.0 specification.",
                            red.apply_to("X")
                        );
                    }
                    LintingError::EMPTY_SCOPE => {
                        println!(
                            "\t{} - Commit title has a scope which is empty.",
                            red.apply_to("X")
                        );
                    }
                    LintingError::PRECEDING_WHITESPACE => {
                        println!(
                            "\t{} - Commit title has preceding whitespace characters.",
                            red.apply_to("X")
                        );
                    }
                    LintingError::NO_DESCRIPTION => {
                        println!(
                            "\t{} - Commit title has no description after the Conventional Commits type and scope.",
                            red.apply_to("X")
                        );
                    }
                    LintingError::NO_SPACE_AFTER_TYPE => {
                        println!(
                            "\t{} - Commit title has no space after the colon preceding the Conventional Commits type and scope.",
                            red.apply_to("X")
                        );
                    }
                }
            }

            println!();
        }
    }
}
