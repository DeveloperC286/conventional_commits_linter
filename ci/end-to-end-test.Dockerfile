FROM python:3.13.6-alpine3.21@sha256:2af1351b0537aa47ce369561142d0277fdff0d4e591ef90c544eda5b52bd3955
RUN apk add --no-cache \
	git=2.47.3-r0

COPY end-to-end-tests/requirements.txt ./
RUN pip3 install -r requirements.txt

ENTRYPOINT ["behave"]
