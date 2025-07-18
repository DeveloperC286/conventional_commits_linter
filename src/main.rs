#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::io::{stdin, Read};

use anyhow::{Context, Result};
use clap::Parser;
use git2::Repository;

use crate::cli::Arguments;
use crate::commits::Commits;
use crate::output::Output;

mod cli;
mod commit_type;
mod commits;
mod history_mode;
mod linting_error;
mod linting_errors;
mod output;
mod source;

const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    let arguments = Arguments::parse();

    // Set up logging: if verbose is true and RUST_LOG is not set, default to info level
    if arguments.verbose && std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::init();

    info!("Version {}.", env!("CARGO_PKG_VERSION"));
    debug!("The command line arguments provided are {arguments:?}.");

    match run(arguments) {
        Ok(exit_code) => {
            std::process::exit(exit_code);
        }
        Err(err) => {
            error!("{err:?}");
            std::process::exit(ERROR_EXIT_CODE);
        }
    }
}

fn run(arguments: Arguments) -> Result<i32> {
    let commits = if arguments.from == "-" {
        let mut commit_message = String::new();
        stdin().read_to_string(&mut commit_message).unwrap();
        Ok(Commits::from_commit_message(commit_message))
    } else {
        let repository =
            Repository::open_from_env().context("Unable to open the Git repository.")?;
        Commits::from_git(&repository, arguments.from, arguments.history_mode)
    }?;

    if let Some(linting_results) =
        commits.lint(&arguments.commit_type, arguments.max_commit_title_length)
    {
        match arguments.output {
            Output::Quiet => {}
            Output::Pretty => {
                println!("{}", linting_results.pretty());
            }
            Output::JSON => {
                println!("{}", linting_results.json()?);
            }
        }

        // As we don't want an error printed but linting failed so want to exit unsuccesffuly.
        return Ok(1);
    }

    info!("Successfully linted all commits.");
    Ok(0)
}
