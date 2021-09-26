use structopt::{clap::ArgGroup, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "conventional_commits_linter",
    about = "A tooling and language agnostic Git commit linter for the Conventional Commits specification.",
    group = ArgGroup::with_name("from").required(true),
    group = ArgGroup::with_name("reporter")
)]
pub(crate) struct Arguments {
    #[structopt(
        group = "from",
        long,
        help = "The Git commit hash from where to take the range of commits from till HEAD to lint. The range is inclusive of HEAD and exclusive of the provided commit hash."
    )]
    pub(crate) from_commit_hash: Option<git2::Oid>,

    #[structopt(
        long,
        group = "from",
        help = "The Git reference from where to start taking the range of commits from till HEAD to lint. The range is inclusive of HEAD and exclusive of the provided reference."
    )]
    pub(crate) from_reference: Option<String>,

    #[structopt(
        group = "from",
        long,
        help = "Read the standard input and lint the input as a Git commit message."
    )]
    pub(crate) from_stdin: bool,

    #[structopt(
        long,
        help = "Allow the Conventional Commits type to only be (`build`, `ci`, `docs`, `feat`, `fix`, `perf`, `refactor`, `style`, `test`, `revert`), otherwise linting for the commit will fail."
    )]
    pub(crate) allow_angular_type_only: bool,

    #[structopt(
        group = "reporter",
        long,
        help = "With JSON enabled when linting errors are encountered they are printed out in a JSON format and not the default pretty more human readable console format."
    )]
    pub(crate) json: bool,

    #[structopt(
        group = "reporter",
        long,
        short,
        help = "With quiet enabled when linting errors are encountered no output is printed."
    )]
    pub(crate) quiet: bool,
}
