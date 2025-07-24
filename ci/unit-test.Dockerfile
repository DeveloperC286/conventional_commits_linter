FROM rust:1.88.0-alpine3.21@sha256:54e937b1530d435dc83b94f5a61ef08365127f2fefbb3789712c5d6f55bbb58c
RUN apk add --no-cache \
	musl-dev=1.2.5-r9

ENTRYPOINT ["cargo", "test", "--verbose", "--target=x86_64-unknown-linux-musl", "--locked"]
