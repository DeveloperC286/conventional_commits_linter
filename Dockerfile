FROM alpine:3.23.4@sha256:5b10f432ef3da1b8d4c7eb6c487f2f5a8f096bc91145e68878dd4a5019afde11

RUN apk add --no-cache \
    git=2.52.0-r0 && \
    git config --system --add safe.directory '*'

ARG TARGET
COPY ./target/${TARGET}/release/conventional_commits_linter /usr/local/bin/

WORKDIR /workspace

ENTRYPOINT ["conventional_commits_linter"]
