FROM python:3.13.6-alpine3.21@sha256:f5f25747d6589917174824189a07ed31051ec6312a7e069a7c3498bbc935f0c3
RUN apk add --no-cache \
	py3-autopep8=2.1.0-r1

ENTRYPOINT ["autopep8", "--exit-code", "--diff", "--aggressive", "--aggressive", "--max-line-length", "120", "--recursive"]
CMD ["."]
