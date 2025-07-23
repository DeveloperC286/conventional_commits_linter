FROM rust:1.88.0-alpine3.21@sha256:0c004cf201a139a3ce08f10cb200402e59499352a21bc01e1b1a1351fcd45c8a
RUN apk add --no-cache \
	musl-dev=1.2.5-r9

ENTRYPOINT ["cargo", "publish", "--verbose", "--target=x86_64-unknown-linux-musl", "--locked"]
