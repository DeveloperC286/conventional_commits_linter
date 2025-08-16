FROM python:3.13.7-alpine3.21@sha256:8f70fe393f5f8ba6c28f24e8f0670d62ae1dcdf3d73bc5f0a849d5764496dd34
RUN apk add --no-cache \
	py3-autopep8=2.1.0-r1

ENTRYPOINT ["autopep8", "--in-place", "--aggressive", "--aggressive", "--max-line-length", "120", "--recursive"]
CMD ["."]
