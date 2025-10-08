FROM python:3.14.0-alpine3.21@sha256:4aaea239340ca20533a4719fd493e69fc03ed308be09d1d76fe878cd9c1cb512
RUN apk add --no-cache \
	py3-autopep8=2.1.0-r1

ENTRYPOINT ["autopep8", "--in-place", "--aggressive", "--aggressive", "--max-line-length", "120", "--recursive"]
CMD ["."]
