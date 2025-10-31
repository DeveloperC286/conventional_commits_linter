FROM rust:1.91.0-alpine3.21@sha256:e9cf108518171a523b0f4f73989d1f35a6c6dc593df52990c29150e57e2ec80a
RUN rustup component add rustfmt

ENTRYPOINT ["cargo", "fmt", "--all", "--", "--config=group_imports=StdExternalCrate"]
