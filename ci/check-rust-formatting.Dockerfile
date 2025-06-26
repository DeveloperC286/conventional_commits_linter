FROM rust:1.88.0-alpine3.21@sha256:9c6a4baf58661f99a5441b15e3ad8295dabf35e849c4935e77ad35d9809be1d2
RUN rustup component add rustfmt

WORKDIR /workspace

ENTRYPOINT ["cargo", "fmt", "--all", "--", "--check", "--config=group_imports=StdExternalCrate"]
