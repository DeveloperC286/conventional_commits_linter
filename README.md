# Conventional Commits Linter
[![crates.io](https://img.shields.io/crates/v/conventional_commits_linter)](https://crates.io/crates/conventional_commits_linter) [![pipeline status](https://gitlab.com/DeveloperC/conventional_commits_linter/badges/master/pipeline.svg)](https://gitlab.com/DeveloperC/conventional_commits_linter/commits/master) [![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org) [![License: AGPL v3](https://img.shields.io/badge/License-AGPLv3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)


## Pre 1.0.0 breaking changes _may_ be introduced without increasing the major version.


A tooling and language agnostic Git commit linter for the Conventional Commits specification.


## Why use Conventional Commits Linter?
 * __No Dependencies__ - A binary download is provided, removing dependencies on downloading tools or interpreter languages.
 * __Correctness__ - The linting against the Conventional Commits specification is strict. Numerous violations missed by other linters are caught.
 * __Sensible Defaults__ - By default no subjective linting rules are applied, only compliance with the Conventional Commits specification is asserted.
 * __Configurable__ - While by default no subjective linting rules are applied, additional subjective linting rules can be enabled.
 * __Fast__ - Utilising Rust the performance is significantly better than other interpreted language linters.


## Upcoming
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
   + [Usage - Additional Flags](#usage-additional-flags)
   + [Usage - Logging](#usage-logging)
 * [CICD Examples](#cicd-examples)
   + [GitLab CI Rust Project Example](#gitlab-ci-rust-project-example)
     + [Via Cargo](#via-cargo)
     + [Via Binary Download](#via-binary-download)
 * [Downloading Binary](#downloading-binary)
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


## CICD Examples
### GitLab CI Rust Project Example
#### Via Cargo
See [Compiling via Cargo](#compiling-via-cargo) for more details about installing via Cargo.

__Note - This example downloads the latest `0.*` version.__

```
conventional-commits-linting:
    stage: conventional-commits-linting
    image: rust
    before_script:
        - cargo install conventional_commits_linter --version ^0
    script:
        - COMMON_ANCESTOR_COMMIT=`git merge-base origin/$CI_MERGE_REQUEST_SOURCE_BRANCH_NAME origin/$CI_MERGE_REQUEST_TARGET_BRANCH_NAME`
        # Lint all the commits in the branch.
        - /usr/local/cargo/bin/conventional_commits_linter --from-commit-hash $COMMON_ANCESTOR_COMMIT --allow-angular-type-only
    rules:
        - if: $CI_MERGE_REQUEST_ID
```


#### Via Binary Download
See [Downloading Binary](#downloading-binary) for more details about Binary downloads.

__Note - This example downloads version `0.7.1`.__

```
conventional-commits-linting:
    stage: conventional-commits-linting
    image: rust
    before_script:
        - wget -q -O tmp.zip "https://gitlab.com/DeveloperC/conventional_commits_linter/-/jobs/artifacts/0.7.1/download?job=release-binary-compiling-x86_64-linux-musl" && unzip tmp.zip && rm tmp.zip
    script:
        - COMMON_ANCESTOR_COMMIT=`git merge-base origin/$CI_MERGE_REQUEST_SOURCE_BRANCH_NAME origin/$CI_MERGE_REQUEST_TARGET_BRANCH_NAME`
        # Lint all the commits in the branch.
        - ./conventional_commits_linter --from-commit-hash $COMMON_ANCESTOR_COMMIT --allow-angular-type-only
    rules:
        - if: $CI_MERGE_REQUEST_ID
```


## Downloading Binary
Statically linked compiled binaries are available for download.
Visit the releases page at [https://gitlab.com/DeveloperC/conventional_commits_linter/-/releases](https://gitlab.com/DeveloperC/conventional_commits_linter/-/releases) to see all the releases, the release notes contains links to binary downloads for various architectures.

If you do not trust the provided binaries another option is to compile your own and then make it available for remote download, so your CICD etc can then download it.


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
Cargo is the Rust package manager, using the `install` sub-command it pulls the Conventional Commits Linter from `crates.io` and then compiles the binary locally.
`cargo install` places the produced binary at `$HOME/.cargo/bin/conventional_commits_linter`.

```
cargo install conventional_commits_linter
```

By default it installs the latest version at the time of execution.
You can specify a specific version to install using the `--version` argument.
For certain environments such as CICD etc you may want to pin the version.

e.g.

```
cargo install conventional_commits_linter --version 0.7.0
```

Rather than pinning to a specific version you can specify the major or minor version.

e.g.

```
cargo install conventional_commits_linter --version ^0
```

Will download the latest `0.*` release whether that is `0.6.7` or `0.11.0`.


## Unit Testing
The unit test suite has a number parameterised tests testing the Conventional Commits v1.0.0 linting, cargo can be used to setup and run all the unit tests.

```
cargo test
```


## End-to-End Testing
To ensure correctness as there are a variety of out of process dependencies the project has an End-to-End behaviour driven test suite using the behave framework (https://github.com/behave/behave).
To run the test suite you need to first build a binary, install Python3, install behave and then execute behave to run the behaviour driven test suite.

__Note - You can't use --release as the test suite uses `target/debug/conventional_commits_linter`.__

```
cargo build
cd end-to-end-tests/
virtualenv -p python3 .venv
source .venv/bin/activate
pip3 install -r requirements.txt
behave
```


## Issues/Feature Requests
To report a bug/issue or request a new feature use [https://gitlab.com/DeveloperC/conventional_commits_linter/-/issues](https://gitlab.com/DeveloperC/conventional_commits_linter/-/issues).
