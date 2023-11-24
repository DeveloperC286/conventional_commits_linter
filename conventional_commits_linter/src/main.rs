#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::io::{stdin, Read};

use anyhow::{bail, Context, Result};
use clap::Parser;
use conventional_commits_linter_lib::Commits;
use git2::Repository;

use crate::cli::Arguments;
use crate::output::Output;

mod cli;
mod output;

const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    pretty_env_logger::init();
    trace!("Version {}.", env!("CARGO_PKG_VERSION"));
    let arguments = Arguments::parse();
    trace!("The command line arguments provided are {arguments:?}.");

    if let Err(err) = run(arguments) {
        error!("{:?}", err);
        std::process::exit(ERROR_EXIT_CODE);
    }
}

fn run(arguments: Arguments) -> Result<()> {
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
            let repository = Repository::open_from_env()
                .context(format!("Unable to open the Git repository."))?;
            Commits::from_commit_hash(&repository, from_commit_hash, arguments.git_history_mode)
        }
        (false, None, Some(from_reference)) => {
            let repository = Repository::open_from_env()
                .context(format!("Unable to open the Git repository."))?;
            Commits::from_reference(&repository, from_reference, arguments.git_history_mode)
        }
        (_, _, _) => {
            bail!("Invalid combination of from arguments.");
        }
    }?;

    if let Some(linting_results) = commits.lint(arguments.allow_angular_type_only) {
        match arguments.output {
            Output::Quiet => {}
            Output::Pretty => {
                println!("{}", linting_results.pretty());
            }
            Output::JSON => {
                // TODO handle unwrap.
                println!("{}", linting_results.json().unwrap());
            }
        }

        bail!("Found linting errors within the commit messages.");
    }

    Ok(())
}
