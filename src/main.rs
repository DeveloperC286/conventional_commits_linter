#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate regex;

use std::process::exit;

use structopt::StructOpt;

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

    let commits =
        git::get_commit_messages_till_head_from(arguments.from_commit_hash, arguments.from_tag);

    match commits.len() {
        0 => {
            error!("No commit messages to lint.");
            exit(ERROR_EXIT_CODE);
        }
        _ => {
            let linting_errors = linter::lint_commits(&commits, arguments.allow_angular_type_only);

            if !linting_errors.is_empty() {
                if !arguments.quiet {
                    reporter::print_linting_errors(&commits, &linting_errors);
                    reporter::print_summary(&linting_errors);
                }
                exit(ERROR_EXIT_CODE);
            }
        }
    }
}
