FROM python:3.13.7-alpine3.21@sha256:45346dc01c597dd56ad621140a7321b6601885e2742af3cc9c9217fced5957f0
RUN apk add --no-cache \
	git=2.47.3-r0

COPY end-to-end-tests/requirements.txt ./
RUN pip3 install -r requirements.txt

ENTRYPOINT ["behave"]
