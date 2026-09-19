FROM alpine:3.24.2@sha256:294b683cb724975bec92580e1e685676bd4b50bda910ddb8c51d4cabeaec77e6

RUN apk add --no-cache \
    git=2.54.0-r0 && \
    git config --system --add safe.directory '*'

ARG TARGET
COPY ./target/${TARGET}/release/conventional_commits_linter /usr/local/bin/

WORKDIR /workspace

ENTRYPOINT ["conventional_commits_linter"]
