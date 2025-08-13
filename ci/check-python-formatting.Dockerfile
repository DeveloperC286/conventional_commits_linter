FROM python:3.13.6-alpine3.21@sha256:2af1351b0537aa47ce369561142d0277fdff0d4e591ef90c544eda5b52bd3955
RUN apk add --no-cache \
	py3-autopep8=2.1.0-r1

ENTRYPOINT ["autopep8", "--exit-code", "--diff", "--aggressive", "--aggressive", "--max-line-length", "120", "--recursive"]
CMD ["."]
