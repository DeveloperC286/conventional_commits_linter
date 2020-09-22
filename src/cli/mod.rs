use structopt::{clap::ArgGroup, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "conventional_commits_linter",
    about = "A fast and sensible linter for the Conventional Commits v1.0.0 format. Which catches violations other linters do not combined with no dependencies on specific tools or interpreter languages.",
    group = ArgGroup::with_name("from").required(true)
)]
pub struct Arguments {
    #[structopt(
        group = "from",
        long = "from-commit-hash",
        help = "The Git commit hash from where to take the range of commits from till HEAD to lint. The range is inclusive of HEAD and exclusive of the provided commit hash."
    )]
    pub from_commit_hash: Option<git2::Oid>,
    #[structopt(
        group = "from",
        long = "from-tag",
        help = "The Git tag from where to take the range of commits from till HEAD to lint. The range is inclusive of HEAD and exclusive of the provided tag."
    )]
    pub from_tag: Option<String>,
    #[structopt(
        long = "allow-angular-type-only",
        help = "Allow the Conventional Commits type to only be (`build`, `ci`, `docs`, `feat`, `fix`, `perf`, `refactor`, `style`, `test`, `revert`), otherwise linting for the commit will fail."
    )]
    pub allow_angular_type_only: bool,
    #[structopt(
        long = "quiet",
        help = "Do not print any linting warnings/errors or summaries out."
    )]
    pub quiet: bool,
}
