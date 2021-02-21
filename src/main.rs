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

    let (commits, print_summary, print_commit_hash) = match arguments.from_stdin {
        true => {
            let mut input = String::new();
            stdin().read_to_string(&mut input).unwrap();

            (
                vec![Commit {
                    oid: git2::Oid::zero(),
                    message: input,
                }],
                false,
                false,
            )
        }
        false => (
            git::get_commit_messages_till_head_from(arguments.from_commit_hash, arguments.from_tag),
            true,
            true,
        ),
    };

    match commits.len() {
        0 => {
            error!("No commit messages to lint.");
            exit(ERROR_EXIT_CODE);
        }
        _ => {
            let linting_errors = linter::lint_commits(&commits, arguments.allow_angular_type_only);

            if !linting_errors.is_empty() {
                if !arguments.quiet {
                    reporter::print_linting_errors(&commits, &linting_errors, print_commit_hash);
                    if print_summary {
                        reporter::print_summary(&linting_errors);
                    }
                }
                exit(ERROR_EXIT_CODE);
            }
        }
    }
}
