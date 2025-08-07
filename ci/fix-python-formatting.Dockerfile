FROM python:3.13.6-alpine3.21@sha256:ae6439a27c05140111b240c4ff1ec49c29281470bdde8945a4dfa4ac2fee19f9
RUN apk add --no-cache \
	py3-autopep8=2.1.0-r1

ENTRYPOINT ["autopep8", "--in-place", "--aggressive", "--aggressive", "--max-line-length", "120", "--recursive"]
CMD ["."]
