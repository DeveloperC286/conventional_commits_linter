FROM python:3.13.7-alpine3.21@sha256:0c3d4f28025c9adc2c03326aa160dde8f53faaa8684134a0e146e4edca28a946
RUN apk add --no-cache \
	py3-autopep8=2.1.0-r1

ENTRYPOINT ["autopep8", "--in-place", "--aggressive", "--aggressive", "--max-line-length", "120", "--recursive"]
CMD ["."]
