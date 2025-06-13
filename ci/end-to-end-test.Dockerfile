FROM python:3.13.5-alpine3.21@sha256:30e1a4f45ca1690f8b0b744f570df0047ec460e3761bd4e9a2ee6d5e00037960
RUN apk add --no-cache \
	git=2.47.2-r0

COPY end-to-end-tests/requirements.txt ./
RUN pip3 install -r requirements.txt

WORKDIR /workspace/end-to-end-tests
ENTRYPOINT ["behave"]
