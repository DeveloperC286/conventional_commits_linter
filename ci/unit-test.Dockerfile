FROM rust:1.91.1-alpine3.21@sha256:33398e1909eae993d43395f85cb102294222bc4ead52a701f72887cb556ff40a
RUN apk add --no-cache \
	musl-dev=1.2.5-r9

ENTRYPOINT ["cargo", "test", "--verbose", "--target=x86_64-unknown-linux-musl", "--locked"]
