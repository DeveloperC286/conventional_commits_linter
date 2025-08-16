FROM python:3.13.7-alpine3.21@sha256:8f70fe393f5f8ba6c28f24e8f0670d62ae1dcdf3d73bc5f0a849d5764496dd34
RUN apk add --no-cache \
	git=2.47.3-r0

COPY end-to-end-tests/requirements.txt ./
RUN pip3 install -r requirements.txt

ENTRYPOINT ["behave"]
