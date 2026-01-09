# Auto-detect musl target for static binaries (Linux only)
# Only set MUSL_TARGET on supported architectures; targets that need it will check
MUSL_TARGET := $(shell uname -m | sed 's/^x86_64$$/x86_64-unknown-linux-musl/;s/^aarch64$$/aarch64-unknown-linux-musl/')

define check-musl-target
$(if $(filter %unknown-linux-musl,$(MUSL_TARGET)),,$(error Unsupported architecture: $(shell uname -m). Static musl builds only supported on Linux x86_64 and aarch64))
endef

# Use --locked in CI to ensure reproducible builds
CARGO_LOCKED := $(if $(CI),--locked,)

.PHONY: default
default: compile

.PHONY: check-shell-permissions
check-shell-permissions:
	./ci/check-scripts-permissions.sh

.PHONY: check-rust-formatting
check-rust-formatting:
	cargo fmt --all -- --check --config=group_imports=StdExternalCrate

.PHONY: fix-rust-formatting
fix-rust-formatting:
	cargo fmt --all -- --config=group_imports=StdExternalCrate

.PHONY: check-shell-formatting
check-shell-formatting:
	shfmt --simplify --diff ci/*

.PHONY: fix-shell-formatting
fix-shell-formatting:
	shfmt --simplify --write ci/*

.PHONY: check-python-formatting
check-python-formatting:
	autopep8 --exit-code --diff --aggressive --aggressive --max-line-length 120 --recursive end-to-end-tests/

.PHONY: fix-python-formatting
fix-python-formatting:
	autopep8 --in-place --aggressive --aggressive --max-line-length 120 --recursive end-to-end-tests/

.PHONY: check-yaml-formatting
check-yaml-formatting:
	yamlfmt -verbose -lint -dstar .github/workflows/*

.PHONY: fix-yaml-formatting
fix-yaml-formatting:
	yamlfmt -verbose -dstar .github/workflows/*

.PHONY: check-rust-linting
check-rust-linting:
	cargo clippy --verbose $(CARGO_LOCKED) -- -D warnings

.PHONY: check-shell-linting
check-shell-linting:
	shellcheck ci/*.sh

.PHONY: check-python-linting
check-python-linting:
	ruff check --line-length 120 end-to-end-tests/

.PHONY: fix-python-linting
fix-python-linting:
	ruff check --fix --line-length 120 end-to-end-tests/

.PHONY: check-github-actions-workflows-linting
check-github-actions-workflows-linting:
	actionlint -verbose -color

.PHONY: check-rust-dependencies
check-rust-dependencies:
	cargo machete

.PHONY: compile
compile:
	cargo build --verbose $(CARGO_LOCKED)

.PHONY: unit-test
unit-test:
	cargo test --verbose $(CARGO_LOCKED)

.PHONY: end-to-end-test
end-to-end-test: compile
	cd end-to-end-tests/ && behave

.PHONY: release
release:
	$(call check-musl-target)
	cargo build --release --target=$(MUSL_TARGET) --locked --verbose

.PHONY: publish-binary
publish-binary: release
	./ci/publish-binary.sh ${RELEASE} $(MUSL_TARGET)

.PHONY: publish-crate
publish-crate:
	cargo publish --verbose

# Emulate GitHub Actions CI environment for testing
GITHUB_ACTIONS_ENV := --env HOME=/github/home --env GITHUB_ACTIONS=true --env CI=true

.PHONY: dogfood-docker
dogfood-docker: release
	docker build --build-arg TARGET=$(MUSL_TARGET) --tag conventional_commits_linter --file Dockerfile .
	docker run --rm --volume $(PWD):/workspace --workdir /workspace $(GITHUB_ACTIONS_ENV) conventional_commits_linter --verbose $(FROM)

.PHONY: publish-docker-image
publish-docker-image:
	./ci/publish-docker-image.sh ${RELEASE} ${PLATFORM} ${TARGET} ${SUFFIX}

.PHONY: publish-docker-manifest
publish-docker-manifest:
	./ci/publish-docker-manifest.sh ${RELEASE}
