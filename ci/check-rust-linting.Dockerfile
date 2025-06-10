FROM rust:1.87.0-alpine3.21@sha256:fa7c28576553c431224a85c897c38f3a6443bd831be37061ab3560d9e797dc82
RUN apk add --no-cache \
	musl-dev=1.2.5-r9
RUN rustup component add clippy

WORKDIR /workspace

ENTRYPOINT ["cargo", "clippy", "--verbose", "--target=x86_64-unknown-linux-musl", "--locked", "--", "-D", "warnings"]
