# Conventional Commits Linter
[![crates.io](https://img.shields.io/crates/v/conventional_commits_linter)](https://crates.io/crates/conventional_commits_linter) [![pipeline status](https://gitlab.com/DeveloperC/conventional_commits_linter/badges/master/pipeline.svg)](https://gitlab.com/DeveloperC/conventional_commits_linter/commits/master) [![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

Conventional Commits Linter is a utility to lint Git commit messages against the Conventional Commits v1.0.0 format.


## Why use Conventional Commits Linter?
 * __Correct__ - Numerous violations against the Conventional Commits v1.0.0 format are caught unlike other linters.
 * __Sensible Defaults__ - By default no subjective linting rules are applied, however you can adjust the linting rules.
 * __No Dependencies__ - A binary download is provided, removing dependencies on downloading tools or interpreter languages.
 * __Fast__ - Utilising Regex and Rust the performance is significantly better than some other interpreted language linters.


## Upcoming
 * Allowing standard input over reading from Git directly.
 * Optional configuration file over CLI arguments/flags.
 * Add commit title length arguments/flags.
 * Add type as noun linting warning/CLI arguments/flags.
 * Add scope as noun linting warning/CLI arguments/flags.
 * Add exclamation before scope linting warning.
 * Adding description casing CLI arguments/flags.
 * Adding type casing CLI arguments/flags.
 * Adding scope casing CLI arguments/flags.


## Content
 * [Usage](#usage)
   + [Usage - Logging](#usage-logging)
   + [Usage - Additional Flags](#usage-additional-flags)
 * [Compiling via Local Repository](#compiling-via-local-repository)
 * [Compiling via Cargo](#compiling-via-cargo)
 * [Unit Testing](#unit-testing)
 * [End-to-End Testing](#end-to-end-testing)
 * [Issues/Feature Requests](#issuesfeature-requests)


## Usage
Conventional Commits Linter operates directly on a Git repository, the Git environment variables are respected.
With the environment variable `GIT_DIR` unset, Conventional Commits Linter will search for a Git repository starting in the current directory.


Using either the argument `--from-commit-hash` or `--from-tag` will note the start of the range of commits till HEAD to lint. The range is inclusive of HEAD and exclusive of the initial commit.


All commit messages in the range are linted against the Conventional Commits v1.0.0 specification.
If any commits messages fail linting then an error message explaining why is logged and Conventional Commits Linter exits with a non zero exit code.


## Usage - Additional Flags

| Flag                      | |
|---------------------------|-|
| --allow-angular-type-only | Allow the Conventional Commits type to only be (`build`, `ci`, `docs`, `feat`, `fix`, `perf`, `refactor`, `style`, `test`, `revert`), otherwise linting for the commit will fail. |


## Usage - Logging
The crates `pretty_env_logger` and `log` are used to provide logging.
The environment variable `RUST_LOG` can be used to set the logging level.
See [https://crates.io/crates/pretty_env_logger](https://crates.io/crates/pretty_env_logger) for more detailed documentation.


## Compiling via Local Repository
Checkout the code repository locally, change into the repository's directory and then build via cargo.
Using the `--release` flag produces an optimised binary but takes longer to compile.

```
git clone git@gitlab.com:DeveloperC/conventional_commits_linter.git
cd conventional_commits_linter/
cargo build --release
```

The compiled binary is present in `target/release/conventional_commits_linter`.


## Compiling via Cargo
Cargo is the Rust package manager, using the `install` sub-command it pulls the crate from `crates.io` and then compiles the binary locally.
`cargo install` places the produced binary at `$HOME/.cargo/bin/conventional_commits_linter`.

```
cargo install conventional_commits_linter
```


## Unit Testing
The unit test suite has a number parameterised tests testing the Conventional Commits v1.0.0 linting, cargo can be used to setup and run all the unit tests.

```
cargo test
```


## End-to-End Testing
To ensure correctness as there are a variety of out of process dependencies the project has an End-to-End test suite.
The End-to-End suite uses the behave framework (https://github.com/behave/behave).
To run the test suite you need to first build a binary, install behave and then execute behave.

#### Note - You can't use --release as the End-to-End test suite uses `target/debug/conventional_commits_linter`.

```
cargo build
cd end-to-end-tests/
virtualenv -p python3 .venv
source .venv/bin/activate
pip install -r requirements.txt
behave
```


## Issues/Feature Requests
To report a bug/issue or request a new feature use [https://gitlab.com/DeveloperC/conventional_commits_linter/-/issues](https://gitlab.com/DeveloperC/conventional_commits_linter/-/issues).
