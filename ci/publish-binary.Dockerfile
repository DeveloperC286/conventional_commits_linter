FROM alpine:3.23@sha256:be171b562d67532ea8b3c9d1fc0904288818bb36fc8359f954a7b7f1f9130fb2
RUN apk add --no-cache \
	github-cli=2.83.0-r1

ENTRYPOINT ["/workspace/ci/publish-binary.sh"]
