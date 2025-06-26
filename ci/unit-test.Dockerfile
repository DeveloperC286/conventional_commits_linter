FROM rust:1.88.0-alpine3.21@sha256:9c6a4baf58661f99a5441b15e3ad8295dabf35e849c4935e77ad35d9809be1d2
RUN apk add --no-cache \
	musl-dev=1.2.5-r9

WORKDIR /workspace

ENTRYPOINT ["cargo", "test", "--verbose", "--target=x86_64-unknown-linux-musl", "--locked"]
