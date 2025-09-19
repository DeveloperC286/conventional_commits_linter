FROM rust:1.90.0-alpine3.21@sha256:85ef0dac25e61cf7a81c43b861401c39577257b848769b95b5fba92bf0ece004
RUN apk add --no-cache \
	musl-dev=1.2.5-r9
RUN rustup component add clippy

ENTRYPOINT ["cargo", "clippy", "--verbose", "--target=x86_64-unknown-linux-musl", "--locked", "--", "-D", "warnings"]
