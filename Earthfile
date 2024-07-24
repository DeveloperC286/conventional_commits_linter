VERSION 0.7


COPY_CI_DATA:
    COMMAND
    COPY --dir "ci/" ".github/" "./"


golang-base:
    FROM golang:1.22.1
    ENV GOPROXY=direct
    ENV CGO_ENABLED=0
    ENV GOOS=linux
    ENV GOARCH=amd64


check-github-actions-workflows-linting:
    FROM +golang-base
    RUN go install github.com/rhysd/actionlint/cmd/actionlint@v1.6.26
    DO +COPY_CI_DATA
    RUN ./ci/check-github-actions-workflows-linting.sh


check-linting:
    BUILD +check-github-actions-workflows-linting


e2e-formatting-base:
		FROM python:3.12.0-slim
		COPY "./ci" "./ci"
		COPY "./conventional_commits_linter/end-to-end-tests" "./conventional_commits_linter/end-to-end-tests"
		RUN pip3 install -r "./conventional_commits_linter/end-to-end-tests/autopep8.requirements.txt"


check-e2e-formatting:
		FROM +e2e-formatting-base
		RUN ./ci/check-e2e-formatting.sh


fix-e2e-formatting:
		FROM +e2e-formatting-base
		RUN ./ci/fix-e2e-formatting.sh
		SAVE ARTIFACT "./conventional_commits_linter/end-to-end-tests" AS LOCAL "./conventional_commits_linter/end-to-end-tests"
