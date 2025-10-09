FROM python:3.14.0-alpine3.21@sha256:c2410fbf9690a8c4ee6482dd0da61cd0b5ee78bcbd3b3b0855a5295a35da0089
RUN apk add --no-cache \
	py3-autopep8=2.1.0-r1

ENTRYPOINT ["autopep8", "--exit-code", "--diff", "--aggressive", "--aggressive", "--max-line-length", "120", "--recursive"]
CMD ["."]
