use structopt::{clap::ArgGroup, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "conventional_commits_linter",
    about = "A fast and sensible linter for the Conventional Commits v1.0.0 format. Which catches violations other linters do not combined with no dependencies on specific tools or interpreter languages.",
    group = ArgGroup::with_name("input").required(true)
)]
pub struct Arguments {
    #[structopt(
        group = "input",
        long,
        help = "The Git commit hash from where to take the range of commits from till HEAD to lint. The range is inclusive of HEAD and exclusive of the provided commit hash."
    )]
    pub from_commit_hash: Option<git2::Oid>,

    #[structopt(
        group = "input",
        long,
        help = "The Git tag from where to take the range of commits from till HEAD to lint. The range is inclusive of HEAD and exclusive of the provided tag."
    )]
    pub from_tag: Option<String>,

    #[structopt(
        group = "input",
        long,
        help = "Read the standard input and lint the input as a Git commit message."
    )]
    pub stdin: bool,

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
