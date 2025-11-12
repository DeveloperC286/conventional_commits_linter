FROM alpine:3.22.2@sha256:4b7ce07002c69e8f3d704a9c5d6fd3053be500b7f1c69fc0d80990c2ad8dd412

RUN apk add --no-cache \
    git=2.49.1-r0 && \
    git config --system --add safe.directory '*'

COPY ./target/x86_64-unknown-linux-musl/release/conventional_commits_linter /usr/local/bin/

WORKDIR /workspace

ENTRYPOINT ["conventional_commits_linter"]
