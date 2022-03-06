# Conventional Commits Linter
[![crates.io](https://img.shields.io/crates/v/conventional_commits_linter)](https://crates.io/crates/conventional_commits_linter)
[![Pipeline Status](https://gitlab.com/DeveloperC/conventional_commits_linter/badges/master/pipeline.svg)](https://gitlab.com/DeveloperC/conventional_commits_linter/commits/master)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org)
[![License](https://img.shields.io/badge/License-AGPLv3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)


## Pre 1.0.0 breaking changes _may_ be introduced without increasing the major version.


A tooling and language agnostic utility to lint Git commits against the Conventional Commits specification.


## Why use Conventional Commits Linter?
 * __No Dependencies__ - A binary download is provided, removing dependencies on downloading tools or interpreter languages.
 * __Correctness__ - The linting against the Conventional Commits specification is strict. Numerous violations missed by other linters are caught.
 * __Sensible Defaults__ - By default no subjective linting rules are applied, only compliance with the Conventional Commits specification is asserted.
 * __Configurable__ - While by default no subjective linting rules are applied, additional subjective linting rules can be enabled.
 * __Fast__ - Utilising Rust the performance is significantly better than other interpreted language linters.


## Upcoming
 * Optional configuration file over CLI arguments.
 * Add commit title length linting.
 * Add type as noun/non-noun linting.
 * Add scope as noun/non-noun linting.
 * Add exclamation before scope linting.
 * Adding description casing linting.
 * Adding type casing linting.
 * Adding scope casing CLI linting.


## Content
 * [Usage](#usage)
   + [Usage - Additional Flags](#usage-additional-flags)
   + [Usage - Git Environment Variables](#usage-git-environment-variables)
   + [Usage - Logging](#usage-logging)
 * [CICD Examples](#cicd-examples)
   + [GitLab CI Rust Project Example](#gitlab-ci-rust-project-example)
     + [Via Cargo](#via-cargo)
     + [Via Binary Download](#via-binary-download)
   + [Git Hooks Rust Project Example](#git-hooks-rust-project-example)
 * [Downloading Binary](#downloading-binary)
 * [Compiling via Local Repository](#compiling-via-local-repository)
 * [Compiling via Cargo](#compiling-via-cargo)
 * [Unit Testing](#unit-testing)
 * [End-to-End Testing](#end-to-end-testing)
 * [Issues/Feature Requests](#issuesfeature-requests)


## Usage
Conventional Commits Linter can either operate upon a range of Git commits in the repositories' history or on a commit message from standard in.
To provide a commit message by standard in simple add the flag `--from-stdin` and standard in will be read.
Otherwise to specify the range of commits you can add either the `--from-commit-hash <commit-hash>` or `--from-reference <reference>` arguments.
The range of commits starts exclusively from the commit specified till inclusively of `HEAD`.

All commit messages provided or within the range are linted against the Conventional Commits v1.0.0 specification.
If any commits messages fail linting then an error message explaining why is logged and Conventional Commits Linter exits with a non zero exit code.

The only required arguments are any of the `--from-stdin`, `--from-commit-hash <commit-hash>` or `--from-reference <reference>` arguments.


### Usage - Additional Flags

| Flag                      | |
|---------------------------|-|
| --allow-angular-type-only | Allow the Conventional Commits type to only be (`build`, `ci`, `docs`, `feat`, `fix`, `perf`, `refactor`, `style`, `test`, `revert`), otherwise linting for the commit will fail. |
| --output | How to output the linting results if their are any, the options are (`Quiet`, `Pretty`, `JSON`) `Pretty` is the default. |


### Usage - Git Environment Variables
When looking for a repository the Git environment variables are respected.
When `${GIT_DIR}` is set, it takes precedence and Conventional Commits Linter begins searching for a repository in the directory specified in `${GIT_DIR}`.
When `${GIT_DIR}` is not set, Conventional Commits Linter searches for a repository beginning in the current directory.


### Usage - Logging
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
        # Lint all the commits in the branch.
        - /usr/local/cargo/bin/conventional_commits_linter --from-reference "origin/${CI_MERGE_REQUEST_TARGET_BRANCH_NAME}" --allow-angular-type-only
    rules:
        - if: $CI_MERGE_REQUEST_ID
```


#### Via Binary Download
See [Downloading Binary](#downloading-binary) for more details about Binary downloads.

__Note - This example downloads version `0.12.0`.__

```
conventional-commits-linting:
    stage: conventional-commits-linting
    image: rust
    before_script:
        - wget -q -O tmp.zip "https://gitlab.com/DeveloperC/conventional_commits_linter/-/jobs/artifacts/bin-0.12.0/download?job=release-binary-compiling-x86_64-linux-musl" && unzip tmp.zip && rm tmp.zip
    script:
        # Lint all the commits in the branch.
        - ./conventional_commits_linter --from-commit-hash "origin/${CI_MERGE_REQUEST_TARGET_BRANCH_NAME}" --allow-angular-type-only
    rules:
        - if: $CI_MERGE_REQUEST_ID
```

### Git Hooks Rust Project Example

An example `commit-msg` Git hook to check if a Rust projects semantic version needs increased because of the commit message.

```
#!/usr/bin/env bash

set -o errexit
set -o pipefail

# Lint new commit's message.
cat "${1}" | "${HOME}/.cargo/bin/conventional_commits_linter" --from-stdin --allow-angular-type-only
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
`cargo install` places the produced binary at `${HOME}/.cargo/bin/conventional_commits_linter`.

```
cargo install conventional_commits_linter
```

By default it installs the latest version at the time of execution.
You can specify a specific version to install using the `--version` argument.
For certain environments such as CICD etc you may want to pin the version.

e.g.

```
cargo install conventional_commits_linter --version 0.12.0
```

Rather than pinning to a specific version you can specify the major or minor version.

e.g.

```
cargo install conventional_commits_linter --version ^0
```

Will download the latest `0.*` release whether that is `0.12.0` or `0.23.0`.


## Unit Testing
The unit test suite has a number parameterised tests testing the Conventional Commits v1.0.0 linting, cargo can be used to setup and run all the unit tests.

```
cargo test
```


## End-to-End Testing
To ensure correctness as there are a variety of out of process dependencies the project has an End-to-End behaviour driven test suite using the behave framework (https://github.com/behave/behave).

To run the test suite you need to
 - Compile the Convention Commits Linter binary.
 - Install Python3.
 - Install Behave.
 - Execute Behave.

__Note - You can't use --release as the test suite uses `target/debug/conventional_commits_linter`.__

```
cargo build
cd conventional_commits_linter/end-to-end-tests/
virtualenv -p python3 .venv
source .venv/bin/activate
pip3 install -r requirements.txt
behave
```


## Issues/Feature Requests
To report a bug/issue or request a new feature use [https://gitlab.com/DeveloperC/conventional_commits_linter/-/issues](https://gitlab.com/DeveloperC/conventional_commits_linter/-/issues).
