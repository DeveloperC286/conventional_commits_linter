#![allow(clippy::single_component_path_imports)]

#[cfg(test)]
use rstest_reuse;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::io::{stdin, Read};

use clap::Parser;
use git2::Repository;

use crate::cli::Arguments;
use crate::commits::Commits;
use crate::output::Output;

mod cli;
mod commits;
mod git_history_mode;
mod linting_error;
mod linting_errors;
mod output;
mod source;

const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    pretty_env_logger::init();
    trace!("Version {}.", env!("CARGO_PKG_VERSION"));
    let arguments = Arguments::parse();
    trace!("The command line arguments provided are {arguments:?}.");

    if run(arguments).is_err() {
        std::process::exit(ERROR_EXIT_CODE);
    }
}

fn run(arguments: Arguments) -> Result<(), git2::Error> {
    let commits = match (
        arguments.from_stdin,
        arguments.from_commit_hash,
        arguments.from_reference,
    ) {
        (true, None, None) => {
            let mut commit_message = String::new();
            stdin().read_to_string(&mut commit_message).unwrap();

            Ok(Commits::from_commit_message(commit_message))
        }
        (false, Some(from_commit_hash), None) => {
            let repository = Repository::open_from_env()?;
            Commits::from_commit_hash(&repository, from_commit_hash, arguments.git_history_mode)
        }
        (false, None, Some(from_reference)) => {
            let repository = Repository::open_from_env()?;
            Commits::from_reference(&repository, from_reference, arguments.git_history_mode)
        }
        (_, _, _) => {
            unreachable!("Invalid combination of arguments.");
        }
    }?;

    if let Some(linting_results) = commits.lint(arguments.allow_angular_type_only) {
        match arguments.output {
            Output::Quiet => {}
            Output::Pretty => {
                println!("{}", linting_results.pretty());
            }
            Output::JSON => {
                // TODO handle
                println!("{}", linting_results.json().unwrap());
            }
        }

        return Err(git2::Error::from_str(""));
    }

    Ok(())
}
