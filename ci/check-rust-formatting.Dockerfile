FROM rust:1.90.0-alpine3.21@sha256:85ef0dac25e61cf7a81c43b861401c39577257b848769b95b5fba92bf0ece004
RUN rustup component add rustfmt

ENTRYPOINT ["cargo", "fmt", "--all", "--", "--check", "--config=group_imports=StdExternalCrate"]
