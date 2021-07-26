#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate regex;

use std::{
    io::{stdin, Read},
    process::exit,
};

use structopt::StructOpt;

use crate::model::Commit;

mod cli;
mod git;
mod linter;
mod model;
mod reporter;

const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    pretty_env_logger::init();
    let arguments = cli::Arguments::from_args();
    debug!("The command line arguments provided are {:?}.", arguments);

    if arguments.from_stdin {
        let mut input = String::new();
        stdin().read_to_string(&mut input).unwrap();

        let commit = Commit {
            oid: git2::Oid::zero(),
            message: input,
        };
        let linting_errors = crate::linter::lint_commit(&commit, arguments.allow_angular_type_only);

        if !linting_errors.is_empty() {
            if !arguments.quiet {
                if arguments.json {
                    println!(
                        "{}",
                        crate::reporter::json::print(&commit.message, &linting_errors)
                    );
                } else {
                    println!(
                        "{}",
                        crate::reporter::pretty::print(None, &commit.message, &linting_errors)
                    );
                }
            }
            exit(ERROR_EXIT_CODE);
        }
    } else {
        let commits = git::get_commit_messages_till_head_from(
            arguments.from_commit_hash,
            arguments.from_reference,
        );

        if commits.is_empty() {
            error!("No commit messages to lint.");
            exit(ERROR_EXIT_CODE);
        }

        let linting_errors =
            crate::linter::lint_commits(&commits, arguments.allow_angular_type_only);

        if !linting_errors.is_empty() {
            if !arguments.quiet {
                if arguments.json {
                    println!(
                        "{}",
                        crate::reporter::json::print_all(&commits, &linting_errors)
                    );
                } else {
                    println!(
                        "{}",
                        crate::reporter::pretty::print_all(&commits, &linting_errors)
                    );
                }
            }
            exit(ERROR_EXIT_CODE);
        }
    }
}
