# Conventional Commits Linter
[![crates.io](https://img.shields.io/crates/v/conventional_commits_linter)](https://crates.io/crates/conventional_commits_linter) [![pipeline status](https://gitlab.com/DeveloperC/conventional_commits_linter/badges/master/pipeline.svg)](https://gitlab.com/DeveloperC/conventional_commits_linter/commits/master) [![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

conventional_commits_linter is a utility to lint Git commit history against the Conventional Commits v1.0.0 format (https://www.conventionalcommits.org/en/v1.0.0/).


## Why?
conventional_commits_linter was created due to a lack of easy to use intuitive linters that comply with the Conventional Commits v1.0.0 format.
The Unix philosophy of 'Make each program do one thing well.' combined with the principle of convention over configuration creates an easy to use and versatile tool not tied to specific tooling or language.


## Content
 * [Usage](#usage)
   + [Usage - Logging](#usage-logging)
 * [Issues/Feature Requests](#issuesfeature-requests)
 * [Compiling via Local Repository](#compiling-via-local-repository)
 * [Compiling via Cargo](#compiling-via-cargo)
 * [Unit Testing](#unit-testing)


## Usage
conventional_commits_linter with the enviroment variable `GIT_DIR` unset, will search for a Git repository starting in the current directory.
Through the non-optional argument `--from-commit-hash` a range of commits will be parsed from the Git repository till HEAD, The range is inclusive of HEAD and exclusive of the provided commit hash.

All commit messages are then linted against the Conventional Commits v1.0.0 specfication.
If any commits do not meet the specfication then an error message is logged and conventional_commits_linter exits with a non zero exit code.
Otherwise conventional_commits_linter exits with a zero exit code.


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


## Issues/Feature Requests
To report a bug/issue or request a new feature use [https://gitlab.com/DeveloperC/conventional_commits_linter/-/issues](https://gitlab.com/DeveloperC/conventional_commits_linter/-/issues).
