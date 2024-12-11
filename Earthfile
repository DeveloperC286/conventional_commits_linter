VERSION 0.7


COPY_CI_DATA:
    COMMAND
    COPY --dir "ci/" ".github/" "./"


COPY_METADATA:
    COMMAND
    DO +COPY_CI_DATA
    COPY --dir ".git/" "./"


alpine-base:
    FROM alpine:3.21.0@sha256:21dc6063fd678b478f57c0e13f47560d0ea4eeba26dfc947b2a4f81f686b9f45
    # renovate: datasource=repology depName=alpine_3_20/bash versioning=loose
    ENV BASH_VERSION="5.2.26-r0"
    RUN apk add --no-cache bash=$BASH_VERSION
    WORKDIR "/conventional_commits_linter"


rust-base:
    FROM +alpine-base
    # renovate: datasource=repology depName=alpine_3_20/rust versioning=loose
    ENV RUST_VERSION="1.78.0-r0"
    RUN apk add --no-cache cargo=$RUST_VERSION


check-clean-git-history:
    FROM +rust-base
    # renovate: datasource=github-releases depName=DeveloperC286/clean_git_history
    ENV CLEAN_GIT_HISTORY_VERSION="v0.2.0"
    RUN wget -O - "https://github.com/DeveloperC286/clean_git_history/releases/download/${CLEAN_GIT_HISTORY_VERSION}/x86_64-unknown-linux-musl.gz" | gzip -d > /usr/bin/clean_git_history && chmod 755 /usr/bin/clean_git_history
    DO +COPY_METADATA
    ARG from_reference="origin/HEAD"
    RUN ./ci/check-clean-git-history.sh --from-reference "${from_reference}"


check-conventional-commits-linting:
    FROM +rust-base
    # renovate: datasource=github-releases depName=DeveloperC286/conventional_commits_linter
    ENV CONVENTIONAL_COMMITS_LINTER_VERSION="v0.14.2"
    RUN wget -O - "https://github.com/DeveloperC286/conventional_commits_linter/releases/download/${CONVENTIONAL_COMMITS_LINTER_VERSION}/x86_64-alpine-linux-musl.gz" | gzip -d > /usr/bin/conventional_commits_linter && chmod 755 /usr/bin/conventional_commits_linter
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
    RUN apk add --no-cache rustfmt=$RUST_VERSION
    DO +COPY_SOURCECODE


check-rust-formatting:
    FROM +rust-formatting-base
    RUN ./ci/check-rust-formatting.sh


python-base:
    FROM +alpine-base
    # renovate: datasource=repology depName=alpine_3_20/python3 versioning=loose
    ENV PYTHON_VERSION="3.12.8-r1"
    # renovate: datasource=repology depName=alpine_3_20/git versioning=loose
    ENV GIT_VERSION="2.45.2-r0"
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
    FROM golang:1.22.1
    WORKDIR "/conventional_commits_linter"


shell-formatting-base:
    FROM +golang-base
    # renovate: datasource=github-releases depName=mvdan/sh
    ENV SHFMT_VERSION="v3.7.0"
    RUN go install mvdan.cc/sh/v3/cmd/shfmt@$SHFMT_VERSION
    DO +COPY_CI_DATA


check-shell-formatting:
    FROM +shell-formatting-base
    RUN ./ci/check-shell-formatting.sh


yaml-formatting-base:
    FROM +golang-base
    # renovate: datasource=github-releases depName=google/yamlfmt
    ENV YAMLFMT_VERSION="v0.10.0"
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
    RUN apk add --no-cache rust-clippy=$RUST_VERSION
    DO +COPY_SOURCECODE
    RUN ./ci/check-rust-linting.sh


check-shell-linting:
    FROM +alpine-base
    # renovate: datasource=repology depName=alpine_3_20/shellcheck versioning=loose
    ENV SHELLCHECK_VERSION="0.10.0-r1"
    RUN apk add --no-cache shellcheck=$SHELLCHECK_VERSION
    DO +COPY_CI_DATA
    RUN ./ci/check-shell-linting.sh


check-github-actions-workflows-linting:
    FROM +golang-base
    # renovate: datasource=github-releases depName=rhysd/actionlint
    ENV ACTIONLINT_VERSION="v1.6.26"
    RUN go install github.com/rhysd/actionlint/cmd/actionlint@$ACTIONLINT_VERSION
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
    RUN pip3 install -r "end-to-end-tests/requirements.txt" --break-system-packages
    COPY "+compile/target/" "target/"
    RUN ./ci/end-to-end-test.sh


release-artifacts:
    FROM +rust-base
    # renovate: datasource=repology depName=alpine_3_20/github-cli versioning=loose
    ENV GITHUB_CLI_VERSION="2.47.0-r4"
    RUN apk add --no-cache github-cli=$GITHUB_CLI_VERSION
    DO +COPY_METADATA
    DO +COPY_SOURCECODE
    ARG release
    RUN --secret GH_TOKEN ./ci/release-artifacts.sh --release "${release}"


publish:
    FROM +rust-base
    COPY "README.md" "./"
    DO +COPY_SOURCECODE
    RUN --secret CARGO_REGISTRY_TOKEN ./ci/publish.sh
