# Conventional Commits Linter
[![crates.io](https://img.shields.io/crates/v/conventional_commits_linter)](https://crates.io/crates/conventional_commits_linter)
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


- [Usage](#usage)
- [Examples](#examples)
  - [GitHub Actions](#github-actions)
  - [GitLab CI](#gitlab-ci)
- [Installation](#installation)
  - [Binary](#binary)
  - [Cargo](#cargo)
  - [Docker](#docker)

## Usage
Conventional Commits Linter can either operate upon a range of Git commits in the repositories' history or on a commit message from standard in.
To provide a commit message by standard in use `-` as the positional argument and standard in will be read.
Otherwise to specify the range of commits you can provide a commit hash or reference as a positional argument.
The range of commits starts exclusively from the commit specified till inclusively of `HEAD`.

All commit messages provided or within the range are linted against the Conventional Commits v1.0.0 specification.
If any commits messages fail linting then an error message explaining why is logged and Conventional Commits Linter exits with a non zero exit code.

The only required argument is a commit hash/reference as a positional argument, or `-` to read from standard input.

## Examples
### GitHub Actions
<!-- x-release-please-start-version -->
```yaml
name: Conventional Commits

on: pull_request

permissions:
  contents: read

jobs:
  linting:
    name: Linting
    runs-on: ubuntu-latest
    container:
      image: ghcr.io/developerc286/conventional_commits_linter:0.17.2
    steps:
      - name: Checkout code.
        uses: actions/checkout@v5
        with:
          ref: ${{ github.event.pull_request.head.sha }}
          fetch-depth: 0
      - name: Check Conventional Commits linting.
        run: conventional_commits_linter --type angular "origin/${{ github.base_ref }}"
```
<!-- x-release-please-end -->

### GitLab CI
<!-- x-release-please-start-version -->
```yaml
conventional-commits-linting:
    stage: conventional-commits-linting
    image: rust
    before_script:
        - version="v0.17.2" && wget -O - "https://github.com/DeveloperC286/conventional_commits_linter/releases/download/${version}/x86_64-unknown-linux-musl.tar.gz" | tar xz --directory "/usr/bin/"
    script:
        # Lint all the commits in the branch.
        - conventional_commits_linter --type angular "origin/${CI_MERGE_REQUEST_SOURCE_BRANCH_NAME}"
    rules:
        - if: $CI_MERGE_REQUEST_ID
```
<!-- x-release-please-end -->

## Installation
### Binary
Statically linked compiled binaries are available for download.
Visit the releases page at [https://github.com/DeveloperC286/conventional_commits_linter/releases](https://github.com/DeveloperC286/conventional_commits_linter/releases) to see all the releases, the release notes contains links to binary downloads for various architectures.

If you do not trust the provided binaries another option is to compile your own and then make it available for remote download, so your CICD etc can then download it.

<!-- x-release-please-start-version -->
```sh
version="v0.17.2" && wget -O - "https://github.com/DeveloperC286/conventional_commits_linter/releases/download/${version}/x86_64-unknown-linux-musl.tar.gz" | tar xz --directory "/usr/bin/"
```
<!-- x-release-please-end -->

### Cargo
Cargo is the Rust package manager, the `install` sub-command pulls from [crates.io](https://crates.io/crates/conventional_commits_linter) and then compiles the binary locally, placing the compiled binary at `${HOME}/.cargo/bin/conventional_commits_linter`.

```sh
cargo install conventional_commits_linter
```

By default it installs the latest version at the time of execution.
You can specify a specific version to install using the `--version` argument.
For certain environments such as CICD etc you may want to pin the version.

e.g.

<!-- x-release-please-start-version -->
```sh
cargo install conventional_commits_linter --version 0.17.2
```
<!-- x-release-please-end -->

See [https://doc.rust-lang.org/cargo/commands/cargo-install.html#install-options](https://doc.rust-lang.org/cargo/commands/cargo-install.html#install-options) for more detailed documentation.

### Docker
You can use the Docker image published to [ghcr.io/developerc286/conventional_commits_linter](https://github.com/DeveloperC286/conventional_commits_linter/pkgs/container/conventional_commits_linter).

```sh
docker run --rm -v $(pwd):/workspace -w /workspace ghcr.io/developerc286/conventional_commits_linter:latest origin/HEAD
```

## Issues/Feature Requests
Report issues or request features on our [GitHub Issues](https://github.com/DeveloperC286/conventional_commits_linter/issues).
