#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate regex;

use structopt::StructOpt;

mod cli;
mod git;
mod linter;

fn main() {
    pretty_env_logger::init();
    let arguments = cli::Arguments::from_args();
    debug!("The command line arguments provided are {:?}.", arguments);

    let commit_messages =
        git::get_commit_messages_till_head_from(arguments.from_commit_hash, arguments.from_tag);

    match commit_messages.len() {
        0 => {
            error!("No commit messages to lint.");
            std::process::exit(1);
        }
        _ => {
            let number_of_linting_errors =
                linter::lint_commits(commit_messages, arguments.allow_angular_type_only);
            if number_of_linting_errors > 0 {
                error!(
                    "{} commits failed Conventional Commits v1.0.0 linting.",
                    number_of_linting_errors
                );
                std::process::exit(1);
            }
        }
    }
}
