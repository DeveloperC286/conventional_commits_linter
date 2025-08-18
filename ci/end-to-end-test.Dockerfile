FROM python:3.13.7-alpine3.21@sha256:0c3d4f28025c9adc2c03326aa160dde8f53faaa8684134a0e146e4edca28a946
RUN apk add --no-cache \
	git=2.47.3-r0

COPY end-to-end-tests/requirements.txt ./
RUN pip3 install -r requirements.txt

ENTRYPOINT ["behave"]
