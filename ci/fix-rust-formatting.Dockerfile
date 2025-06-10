FROM rust:1.87.0-alpine3.21@sha256:fa7c28576553c431224a85c897c38f3a6443bd831be37061ab3560d9e797dc82
RUN rustup component add rustfmt

WORKDIR /workspace

ENTRYPOINT ["cargo", "fmt", "--all", "--", "--config=group_imports=StdExternalCrate"]
