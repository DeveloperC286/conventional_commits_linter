FROM rust:1.90.0-alpine3.21@sha256:1b3ecdc66183eb821f89f9a085af55b842593d711b95894ec4d90e00dc234198
RUN rustup component add rustfmt

ENTRYPOINT ["cargo", "fmt", "--all", "--", "--check", "--config=group_imports=StdExternalCrate"]
