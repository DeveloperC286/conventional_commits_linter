FROM python:3.13.4-alpine3.21@sha256:70f816e6bc36b178b2629c10b44b6ff7735c1ffc75d3b1d871f85490a4e62310
RUN apk add --no-cache \
	py3-autopep8=2.1.0-r1

WORKDIR /workspace

ENTRYPOINT ["autopep8", "--exit-code", "--diff", "--aggressive", "--aggressive", "--max-line-length", "120", "--recursive"]
CMD ["."]
