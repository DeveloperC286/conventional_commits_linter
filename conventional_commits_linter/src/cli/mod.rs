use structopt::{clap::ArgGroup, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "conventional_commits_linter",
    about = "A tooling and language agnostic Git commit linter for the Conventional Commits specification.",
    group = ArgGroup::with_name("from").required(true),
)]
pub(crate) struct Arguments {
    #[structopt(
        group = "from",
        long,
        help = "Read the standard input and lint the input as a Git commit message."
    )]
    pub(crate) from_stdin: bool,

    #[structopt(
        group = "from",
        long,
        help = "The Git reference from where to start taking the range of commits from till HEAD to lint. The range is inclusive of HEAD and exclusive of the provided reference."
    )]
    pub(crate) from_reference: Option<String>,

    #[structopt(
        group = "from",
        long,
        help = "The Git commit hash from where to take the range of commits from till HEAD to lint. The range is inclusive of HEAD and exclusive of the provided commit hash."
    )]
    pub(crate) from_commit_hash: Option<String>,

    #[structopt(
        long,
        default_value = "FirstParent",
        help = "The mode to use when transversing the Git commit history of the Git commit range, to collect the Git commit messages to use in calculating the next semantic version."
    )]
    pub(crate) git_history_mode: conventional_commits_linter_lib::GitHistoryMode,

    #[structopt(
        long,
        help = "Allow the Conventional Commits type to only be (`build`, `ci`, `docs`, `feat`, `fix`, `perf`, `refactor`, `style`, `test`, `revert`), otherwise linting for the commit will fail."
    )]
    pub(crate) allow_angular_type_only: bool,

    #[structopt(
        long,
        default_value = "Pretty",
        help = "How to output the linting results if their are any, the options are (`Quiet`, `Pretty`, `JSON`) `Pretty` is the default."
    )]
    pub(crate) output: crate::output::Output,
}
