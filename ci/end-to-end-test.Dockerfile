FROM python:3.13.6-alpine3.21@sha256:4358f4bd970e6005fb09893ea7efea3491a132ca758b2cd2cde4e0bc07ec56b9
RUN apk add --no-cache \
	git=2.47.3-r0

COPY end-to-end-tests/requirements.txt ./
RUN pip3 install -r requirements.txt

ENTRYPOINT ["behave"]
