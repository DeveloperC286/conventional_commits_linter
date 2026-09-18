FROM alpine:3.24.2@sha256:3cf95fe0816180395592b8373f3ec60663f076127617bbacb4eacf9667afe2e9

RUN apk add --no-cache \
    git=2.54.0-r0 && \
    git config --system --add safe.directory '*'

ARG TARGET
COPY ./target/${TARGET}/release/conventional_commits_linter /usr/local/bin/

WORKDIR /workspace

ENTRYPOINT ["conventional_commits_linter"]
