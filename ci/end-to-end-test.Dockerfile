FROM python:3.13.4-alpine3.21@sha256:70f816e6bc36b178b2629c10b44b6ff7735c1ffc75d3b1d871f85490a4e62310
RUN apk add --no-cache \
	git=2.47.2-r0

COPY end-to-end-tests/requirements.txt ./
RUN pip3 install -r requirements.txt

WORKDIR /workspace/end-to-end-tests
ENTRYPOINT ["behave"]
