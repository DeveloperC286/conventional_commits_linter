FROM python:3.14.0-alpine3.21@sha256:a9bf2b3856bc05b63ee77a83af8b38b4378135e8bce3c5f7455252fa2ac5abe4
RUN apk add --no-cache \
	py3-autopep8=2.1.0-r1

ENTRYPOINT ["autopep8", "--in-place", "--aggressive", "--aggressive", "--max-line-length", "120", "--recursive"]
CMD ["."]
