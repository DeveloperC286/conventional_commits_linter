use clap::ValueEnum;

#[derive(Clone, Debug, PartialEq, ValueEnum)]
pub enum CommitType {
    // Allow the Conventional Commits type to be any value.
    Any,
    // Allow the Conventional Commits type to only be (`build`, `chore`, `ci`, `docs`, `feat`, `fix`, `perf`, `refactor`, `style`, `test`, `revert`).
    Angular,
}
