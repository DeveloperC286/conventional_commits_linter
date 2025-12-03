FROM python:3.14.1-alpine3.21@sha256:27174290ede5c6787491596ef1c0f7692474e027e426c36b18f8244dd9043405
RUN apk add --no-cache \
	py3-autopep8=2.1.0-r1

ENTRYPOINT ["autopep8", "--in-place", "--aggressive", "--aggressive", "--max-line-length", "120", "--recursive"]
CMD ["."]
