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
mod model;
mod reporter;

fn main() {
    pretty_env_logger::init();
    let arguments = cli::Arguments::from_args();
    debug!("The command line arguments provided are {:?}.", arguments);

    let commits =
        git::get_commit_messages_till_head_from(arguments.from_commit_hash, arguments.from_tag);

    match commits.len() {
        0 => {
            error!("No commit messages to lint.");
            std::process::exit(1);
        }
        _ => {
            let linting_errors = linter::lint_commits(commits, arguments.allow_angular_type_only);

            reporter::print_summary(&linting_errors);

            if linting_errors.len() > 0 {
                std::process::exit(1);
            }
        }
    }
}
