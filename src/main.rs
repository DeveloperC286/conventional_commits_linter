#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate regex;

use structopt::StructOpt;

mod git;
mod linter;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "conventional_commits_linter",
    about = "A utility to lint Git commit messages against the Convectional Commits v1.0.0 format."
)]
struct Args {
    #[structopt(
        long = "from-commit-hash",
        help = "The Git commit hash from where to take the range of commits from till HEAD to lint. The range is inclusive of HEAD and exclusive of the provided commit hash."
    )]
    from_commit_hash: String,
}

fn main() {
    pretty_env_logger::init();
    let args = Args::from_args();
    debug!("The command line arguments provided are {:?}.", args);

    let commit_messages = git::get_commit_messages_from(&args.from_commit_hash);

    let number_of_linting_errors = linter::lint_commits(commit_messages);
    if number_of_linting_errors > 0 {
        error!(
            "{} commits failed Conventional Commits v1.0.0 linting.",
            number_of_linting_errors
        );
        std::process::exit(1);
    }
}
