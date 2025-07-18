If you make changes to the following directories, run the following make targets to verify your work:

**If you change `ci/`:**
1. `make fix-shell-formatting`

**If you change `src/`:**
1. `make fix-rust-formatting`
2. `make check-rust-linting`
3. `make compile`
4. `make unit-test`
5. `make end-to-end-test`

**If you change `end-to-end-tests/`:**
1. `make fix-python-formatting`
2. `make end-to-end-test`

**If you change `.github/workflows/*`:**
1. `make fix-yaml-formatting`
