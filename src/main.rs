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

    if arguments.from_commit_hash.is_some() && arguments.from_tag.is_some() {
        error!("Provide either the --from-tag or --from-commit-hash arguments not both.");
        std::process::exit(1);
    }

    if arguments.from_commit_hash.is_none() && arguments.from_tag.is_none() {
        error!("Provide either the --from-tag or --from-commit-hash argument.");
        std::process::exit(1);
    }

    let mut commit_messages: Vec<String> = vec![];
    if let Some(oid) = arguments.from_commit_hash {
        commit_messages = git::get_commit_messages_from(oid)
    }

    if let Some(tag_name) = arguments.from_tag {
        match git::get_tag_oid(&tag_name) {
            Some(oid) => {
                commit_messages = git::get_commit_messages_from(oid);
            }
            None => {
                error!("Could not find tag with the name '{}'.", tag_name);
                std::process::exit(1);
            }
        }
    }

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
