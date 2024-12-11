use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[command(version, about)]
#[command(group(
            ArgGroup::new("from")
                .required(true)
        ))]
pub(crate) struct Arguments {
    #[arg(
        group = "from",
        long,
        help = "Read the standard input and lint the input as a Git commit message."
    )]
    pub(crate) from_stdin: bool,

    #[arg(
        group = "from",
        long,
        help = "The Git reference from where to start taking the range of commits from till HEAD to lint. The range is inclusive of HEAD and exclusive of the provided reference."
    )]
    pub(crate) from_reference: Option<String>,

    #[arg(
        group = "from",
        long,
        help = "The Git commit hash from where to take the range of commits from till HEAD to lint. The range is inclusive of HEAD and exclusive of the provided commit hash."
    )]
    pub(crate) from_commit_hash: Option<String>,

    #[arg(
        long,
        default_value = "first",
        help = "Specifies how commits are parsed, acceptable values are 'first' to parse only the first parent of merge commits, or 'all' to parse all parents."
    )]
    pub(crate) history_mode: crate::history_mode::HistoryMode,

    #[arg(
        long,
        help = "Allow the Conventional Commits type to only be (`build`, `ci`, `docs`, `feat`, `fix`, `perf`, `refactor`, `style`, `test`, `revert`), otherwise linting for the commit will fail."
    )]
    pub(crate) allow_angular_type_only: bool,

    #[arg(
        long,
        default_value = "pretty",
        help = "Specifies the format for outputting results, acceptable values are quiet, pretty, and json."
    )]
    pub(crate) output: crate::output::Output,
}
