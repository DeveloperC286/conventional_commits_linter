FROM rust:1.90.0-alpine3.21@sha256:3757b14ddcc2057eb91a074dcdd0913bed839b22444bd2229a49eea910ed8736
RUN apk add --no-cache \
	musl-dev=1.2.5-r9
RUN rustup component add clippy

ENTRYPOINT ["cargo", "clippy", "--verbose", "--target=x86_64-unknown-linux-musl", "--locked", "--", "-D", "warnings"]
