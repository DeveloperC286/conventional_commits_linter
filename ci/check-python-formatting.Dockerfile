FROM python:3.14.0-alpine3.21@sha256:814983b0c51602a3fecc22afaff2321f79b547230020e1428e2d778367e2ffed
RUN apk add --no-cache \
	py3-autopep8=2.1.0-r1

ENTRYPOINT ["autopep8", "--exit-code", "--diff", "--aggressive", "--aggressive", "--max-line-length", "120", "--recursive"]
CMD ["."]
