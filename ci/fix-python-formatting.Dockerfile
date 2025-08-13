FROM python:3.13.6-alpine3.21@sha256:4358f4bd970e6005fb09893ea7efea3491a132ca758b2cd2cde4e0bc07ec56b9
RUN apk add --no-cache \
	py3-autopep8=2.1.0-r1

ENTRYPOINT ["autopep8", "--in-place", "--aggressive", "--aggressive", "--max-line-length", "120", "--recursive"]
CMD ["."]
