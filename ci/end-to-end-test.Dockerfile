FROM python:3.14.1-alpine3.21@sha256:27174290ede5c6787491596ef1c0f7692474e027e426c36b18f8244dd9043405
RUN apk add --no-cache \
	git=2.47.3-r0

COPY end-to-end-tests/requirements.txt ./
RUN pip3 install -r requirements.txt

ENTRYPOINT ["behave"]
