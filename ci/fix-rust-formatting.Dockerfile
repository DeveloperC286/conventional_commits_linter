FROM rust:1.89.0-alpine3.21@sha256:8f717b9506b922815d461317b6ac40d2cddc0f77867809509a5f1d32b8372ea4
RUN rustup component add rustfmt

ENTRYPOINT ["cargo", "fmt", "--all", "--", "--config=group_imports=StdExternalCrate"]
