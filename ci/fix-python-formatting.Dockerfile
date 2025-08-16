FROM python:3.13.7-alpine3.21@sha256:45346dc01c597dd56ad621140a7321b6601885e2742af3cc9c9217fced5957f0
RUN apk add --no-cache \
	py3-autopep8=2.1.0-r1

ENTRYPOINT ["autopep8", "--in-place", "--aggressive", "--aggressive", "--max-line-length", "120", "--recursive"]
CMD ["."]
