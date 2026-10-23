FROM alpine:3.24.3@sha256:be2656061b536c1883c4034790e21d0a09c5aaaf51968b89368a3c0371ea2c9a

RUN apk add --no-cache \
    git=2.54.0-r0 && \
    git config --system --add safe.directory '*'

ARG TARGET
COPY ./target/${TARGET}/release/conventional_commits_linter /usr/local/bin/

WORKDIR /workspace

ENTRYPOINT ["conventional_commits_linter"]
