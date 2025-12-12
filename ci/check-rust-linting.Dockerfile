FROM rust:1.92.0-alpine3.21@sha256:82e772a24cfceaea095cc3f36b7bdeab048fc4f1164e0948939ee7a3f070ddbb
RUN apk add --no-cache \
	musl-dev=1.2.5-r9
RUN rustup component add clippy

ENTRYPOINT ["cargo", "clippy", "--verbose", "--target=x86_64-unknown-linux-musl", "--locked", "--", "-D", "warnings"]
