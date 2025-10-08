FROM python:3.14.0-alpine3.21@sha256:4aaea239340ca20533a4719fd493e69fc03ed308be09d1d76fe878cd9c1cb512
RUN apk add --no-cache \
	git=2.47.3-r0

COPY end-to-end-tests/requirements.txt ./
RUN pip3 install -r requirements.txt

ENTRYPOINT ["behave"]
