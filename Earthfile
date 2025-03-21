VERSION 0.7


COPY_CI_DATA:
    COMMAND
    COPY --dir "ci/" ".github/" "./"


COPY_METADATA:
    COMMAND
    DO +COPY_CI_DATA
    COPY --dir ".git/" "./"


rust-base:
    FROM rust:1.85.0-alpine3.20@sha256:c2f212dabdc0bf8d252b0e49427705be87f5061530fd6ea5b99a28d4807a3d3f
    # renovate: datasource=repology depName=alpine_3_20/bash versioning=loose
    ENV BASH_VERSION="5.2.26-r0"
    # renovate: datasource=repology depName=alpine_3_20/musl-dev versioning=loose
    ENV MUSL_VERSION="1.2.5-r1"
    RUN apk add --no-cache bash=$BASH_VERSION musl-dev=$MUSL_VERSION
    RUN rustup component add rustfmt clippy
    WORKDIR "/conventional_commits_linter"


check-clean-git-history:
    FROM +rust-base
    # renovate: datasource=github-releases depName=DeveloperC286/clean_git_history
    ENV CLEAN_GIT_HISTORY_VERSION="v1.0.0"
    RUN wget -O - "https://github.com/DeveloperC286/clean_git_history/releases/download/${CLEAN_GIT_HISTORY_VERSION}/x86_64-unknown-linux-musl.tar.gz" | tar xz --directory "/usr/bin/"
    DO +COPY_METADATA
    ARG from="origin/HEAD"
    RUN ./ci/check-clean-git-history.sh "${from_reference}"


check-conventional-commits-linting:
    FROM +rust-base
    # renovate: datasource=github-releases depName=DeveloperC286/conventional_commits_linter
    ENV CONVENTIONAL_COMMITS_LINTER_VERSION="v0.14.3"
    RUN wget -O - "https://github.com/DeveloperC286/conventional_commits_linter/releases/download/${CONVENTIONAL_COMMITS_LINTER_VERSION}/x86_64-unknown-linux-musl.gz" | gzip -d > /usr/bin/conventional_commits_linter && chmod 755 /usr/bin/conventional_commits_linter
    DO +COPY_METADATA
    ARG from_reference="origin/HEAD"
    RUN ./ci/check-conventional-commits-linting.sh --from-reference "${from_reference}"


COPY_SOURCECODE:
    COMMAND
    DO +COPY_CI_DATA
    COPY --if-exists "Cargo.lock" "./"
    COPY --dir "Cargo.toml" "src/" "end-to-end-tests/" "./"


sourcecode-base:
    FROM +rust-base
    DO +COPY_SOURCECODE


check-rust-formatting:
    FROM +sourcecode-base
    RUN ./ci/check-rust-formatting.sh


python-base:
    FROM +rust-base
    # renovate: datasource=repology depName=alpine_3_20/python3 versioning=loose
    ENV PYTHON_VERSION="3.12.9-r0"
    # renovate: datasource=repology depName=alpine_3_20/git versioning=loose
    ENV GIT_VERSION="2.45.3-r0"
    # renovate: datasource=repology depName=alpine_3_20/py3-pip versioning=loose
    ENV PIP_VERSION="24.0-r2"
    RUN apk add --no-cache py3-pip=$PIP_VERSION python3=$PYTHON_VERSION git=$GIT_VERSION
    DO +COPY_SOURCECODE


python-formatting-base:
    FROM +python-base
    RUN pip3 install -r "end-to-end-tests/autopep8.requirements.txt" --break-system-packages


check-python-formatting:
    FROM +python-formatting-base
    RUN ./ci/check-python-formatting.sh


golang-base:
    FROM golang:1.24.1@sha256:52ff1b35ff8de185bf9fd26c70077190cd0bed1e9f16a2d498ce907e5c421268
    WORKDIR "/conventional_commits_linter"


shell-formatting-base:
    FROM +golang-base
    # renovate: datasource=github-releases depName=mvdan/sh
    ENV SHFMT_VERSION="v3.11.0"
    RUN go install mvdan.cc/sh/v3/cmd/shfmt@$SHFMT_VERSION
    DO +COPY_CI_DATA


check-shell-formatting:
    FROM +shell-formatting-base
    RUN ./ci/check-shell-formatting.sh


yaml-formatting-base:
    FROM +golang-base
    # renovate: datasource=github-releases depName=google/yamlfmt
    ENV YAMLFMT_VERSION="v0.16.0"
    RUN go install github.com/google/yamlfmt/cmd/yamlfmt@$YAMLFMT_VERSION
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
    FROM +sourcecode-base
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
    FROM +sourcecode-base
    RUN ./ci/check-rust-linting.sh


check-shell-linting:
    FROM +rust-base
    # renovate: datasource=repology depName=alpine_3_20/shellcheck versioning=loose
    ENV SHELLCHECK_VERSION="0.10.0-r1"
    RUN apk add --no-cache shellcheck=$SHELLCHECK_VERSION
    DO +COPY_CI_DATA
    RUN ./ci/check-shell-linting.sh


check-github-actions-workflows-linting:
    FROM +golang-base
    # renovate: datasource=github-releases depName=rhysd/actionlint
    ENV ACTIONLINT_VERSION="v1.7.7"
    RUN go install github.com/rhysd/actionlint/cmd/actionlint@$ACTIONLINT_VERSION
    DO +COPY_CI_DATA
    RUN ./ci/check-github-actions-workflows-linting.sh


check-linting:
    BUILD +check-rust-linting
    BUILD +check-shell-linting
    BUILD +check-github-actions-workflows-linting


compile:
    FROM +sourcecode-base
    RUN ./ci/compile.sh
    SAVE ARTIFACT "target/" AS LOCAL "./"
    SAVE ARTIFACT "Cargo.lock" AS LOCAL "./"


unit-test:
    FROM +sourcecode-base
    RUN ./ci/unit-test.sh


static-binary-test:
    FROM ubuntu:24.04@sha256:72297848456d5d37d1262630108ab308d3e9ec7ed1c3286a32fe09856619a782
    COPY "+compile/target/" "target/"
    RUN ./target/debug/conventional_commits_linter --help


end-to-end-test:
    FROM +python-base
    RUN pip3 install -r "end-to-end-tests/requirements.txt" --break-system-packages
    COPY "+compile/target/" "target/"
    RUN ./ci/end-to-end-test.sh


publish-binary:
    FROM +rust-base
    # renovate: datasource=repology depName=alpine_3_20/github-cli versioning=loose
    ENV GITHUB_CLI_VERSION="2.47.0-r4"
    RUN apk add --no-cache github-cli=$GITHUB_CLI_VERSION
    DO +COPY_METADATA
    DO +COPY_SOURCECODE
    ARG release
    RUN --secret GH_TOKEN ./ci/publish-binary.sh --release "${release}"


publish-crate:
    FROM +sourcecode-base
    COPY "README.md" "./"
    RUN --secret CARGO_REGISTRY_TOKEN ./ci/publish-crate.sh
