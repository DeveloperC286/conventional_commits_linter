FROM rust:1.91.1-alpine3.21@sha256:32817dd3ce1cd4ed459dcc289df1b906dc53719975156636e605063821668b1e
RUN apk add --no-cache \
	musl-dev=1.2.5-r9

ENTRYPOINT ["cargo", "publish", "--verbose", "--target=x86_64-unknown-linux-musl", "--locked"]
