use structopt::{clap::ArgGroup, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "conventional_commits_linter",
    about = "A tooling and language agnostic Git commit linter for the Conventional Commits specification.",
    group = ArgGroup::with_name("from").required(true)
)]
pub struct Arguments {
    #[structopt(
        group = "from",
        long,
        help = "The Git commit hash from where to take the range of commits from till HEAD to lint. The range is inclusive of HEAD and exclusive of the provided commit hash."
    )]
    pub from_commit_hash: Option<git2::Oid>,

    #[structopt(
        group = "from",
        long,
        help = "The Git tag from where to take the range of commits from till HEAD to lint. The range is inclusive of HEAD and exclusive of the provided tag."
    )]
    pub from_tag: Option<String>,

    #[structopt(
        group = "from",
        long,
        help = "Read the standard input and lint the input as a Git commit message."
    )]
    pub from_stdin: bool,

    #[structopt(
        long,
        help = "Allow the Conventional Commits type to only be (`build`, `ci`, `docs`, `feat`, `fix`, `perf`, `refactor`, `style`, `test`, `revert`), otherwise linting for the commit will fail."
    )]
    pub allow_angular_type_only: bool,

    #[structopt(
        long,
        short,
        help = "Do not print any linting warnings/errors or summaries out."
    )]
    pub quiet: bool,
}
