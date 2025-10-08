FROM alpine:3.22@sha256:56b31e2dadc083b6b067d6cd4e97a9c6e5a953e6595830c60d9197589ff88ad4
RUN apk add --no-cache \
	github-cli=2.72.0-r3

ENTRYPOINT ["/workspace/ci/publish-binary.sh"]
