FROM python:3.13.5-alpine3.21@sha256:30e1a4f45ca1690f8b0b744f570df0047ec460e3761bd4e9a2ee6d5e00037960
RUN apk add --no-cache \
	py3-autopep8=2.1.0-r1

WORKDIR /workspace

ENTRYPOINT ["autopep8", "--in-place", "--aggressive", "--aggressive", "--max-line-length", "120", "--recursive"]
CMD ["."]
