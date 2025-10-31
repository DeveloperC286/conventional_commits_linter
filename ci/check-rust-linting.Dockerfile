FROM rust:1.91.0-alpine3.21@sha256:e9cf108518171a523b0f4f73989d1f35a6c6dc593df52990c29150e57e2ec80a
RUN apk add --no-cache \
	musl-dev=1.2.5-r9
RUN rustup component add clippy

ENTRYPOINT ["cargo", "clippy", "--verbose", "--target=x86_64-unknown-linux-musl", "--locked", "--", "-D", "warnings"]
