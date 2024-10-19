VERSION 0.7


COPY_CI_DATA:
    COMMAND
    COPY --dir "ci/" ".github/" "./"


COPY_METADATA:
    COMMAND
    DO +COPY_CI_DATA
    COPY --dir ".git/" "./"


rust-base:
    FROM rust:1.74.0-alpine3.18
    RUN apk add --no-cache musl-dev bash
    WORKDIR "/conventional_commits_linter"


check-clean-git-history:
    FROM +rust-base
    RUN cargo install clean_git_history --version 0.1.2 --locked
    DO +COPY_METADATA
    ARG from_reference="origin/HEAD"
    RUN ./ci/check-clean-git-history.sh --from-reference "${from_reference}"


check-conventional-commits-linting:
    FROM +rust-base
    RUN cargo install conventional_commits_linter --version 0.12.3 --locked
    DO +COPY_METADATA
    ARG from_reference="origin/HEAD"
    RUN ./ci/check-conventional-commits-linting.sh --from-reference "${from_reference}"


COPY_SOURCECODE:
    COMMAND
    DO +COPY_CI_DATA
    COPY --if-exists "Cargo.lock" "./"
    COPY --dir "Cargo.toml" "src/" "end-to-end-tests/" "./"


rust-formatting-base:
    FROM +rust-base
    RUN rustup component add rustfmt
    DO +COPY_SOURCECODE


check-rust-formatting:
    FROM +rust-formatting-base
    RUN ./ci/check-rust-formatting.sh


python-base:
    FROM python:3.9.19-alpine
    RUN apk add --no-cache git bash
    WORKDIR "/consistent_whitespace"
    DO +COPY_SOURCECODE


python-formatting-base:
    FROM +python-base
    RUN pip3 install -r "end-to-end-tests/autopep8.requirements.txt"


check-python-formatting:
    FROM +python-formatting-base
    RUN ./ci/check-python-formatting.sh


golang-base:
    FROM golang:1.22.1
    WORKDIR "/consistent_whitespace"


shell-formatting-base:
    FROM +golang-base
    RUN go install mvdan.cc/sh/v3/cmd/shfmt@v3.7.0
    DO +COPY_CI_DATA


check-shell-formatting:
    FROM +shell-formatting-base
    RUN ./ci/check-shell-formatting.sh


yaml-formatting-base:
    FROM +golang-base
    RUN go install github.com/google/yamlfmt/cmd/yamlfmt@v0.10.0
    COPY ".yamlfmt" "./"
    DO +COPY_CI_DATA


check-yaml-formatting:
    FROM +yaml-formatting-base
    RUN ./ci/check-yaml-formatting.sh


check-formatting:
    BUILD +check-rust-formatting
    BUILD +check-python-formatting
    BUILD +check-shell-formatting
    BUILD +check-yaml-formatting


fix-rust-formatting:
    FROM +rust-formatting-base
    RUN ./ci/fix-rust-formatting.sh
    SAVE ARTIFACT "src/" AS LOCAL "./"


fix-python-formatting:
    FROM +python-formatting-base
    RUN ./ci/fix-python-formatting.sh
    SAVE ARTIFACT "end-to-end-tests/" AS LOCAL "./"


fix-shell-formatting:
    FROM +shell-formatting-base
    RUN ./ci/fix-shell-formatting.sh
    SAVE ARTIFACT "ci/" AS LOCAL "./"


fix-yaml-formatting:
    FROM +yaml-formatting-base
    RUN ./ci/fix-yaml-formatting.sh
    SAVE ARTIFACT ".github/" AS LOCAL "./"


fix-formatting:
    BUILD +fix-rust-formatting
    BUILD +fix-python-formatting
    BUILD +fix-shell-formatting
    BUILD +fix-yaml-formatting


check-rust-linting:
    FROM +rust-base
    RUN rustup component add clippy
    DO +COPY_SOURCECODE
    RUN ./ci/check-rust-linting.sh


ubuntu-base:
    FROM ubuntu:22.04
    # https://askubuntu.com/questions/462690/what-does-apt-get-fix-missing-do-and-when-is-it-useful
    RUN apt-get update --fix-missing


check-shell-linting:
    FROM +ubuntu-base
    RUN apt-get install shellcheck -y
    DO +COPY_CI_DATA
    RUN ./ci/check-shell-linting.sh


check-github-actions-workflows-linting:
    FROM +golang-base
    RUN go install github.com/rhysd/actionlint/cmd/actionlint@v1.6.26
    DO +COPY_CI_DATA
    RUN ./ci/check-github-actions-workflows-linting.sh


check-linting:
    BUILD +check-rust-linting
    BUILD +check-shell-linting
    BUILD +check-github-actions-workflows-linting


compile:
    FROM +rust-base
    DO +COPY_SOURCECODE
    RUN ./ci/compile.sh
    SAVE ARTIFACT "target/" AS LOCAL "./"
    SAVE ARTIFACT "Cargo.lock" AS LOCAL "./"


unit-test:
    FROM +rust-base
    DO +COPY_SOURCECODE
    RUN ./ci/unit-test.sh


end-to-end-test:
    FROM +python-base
    RUN pip3 install -r "end-to-end-tests/requirements.txt"
    COPY "+compile/target/" "target/"
    RUN ./ci/end-to-end-test.sh


release-artifacts:
    FROM +rust-base
    DO +COPY_CI_DATA
    # Needed by the GitHub CLI.
    RUN apk add --no-cache git
    RUN ./ci/install-github-cli.sh
    DO +COPY_METADATA
    DO +COPY_SOURCECODE
    ARG release
    RUN --secret GH_TOKEN ./ci/release-artifacts.sh --release "${release}"


publish:
    FROM +rust-base
    COPY "README.md" "./"
    DO +COPY_SOURCECODE
    RUN --secret CARGO_REGISTRY_TOKEN ./ci/publish.sh
