FROM rust:1.92.0-alpine3.21@sha256:82e772a24cfceaea095cc3f36b7bdeab048fc4f1164e0948939ee7a3f070ddbb
RUN rustup component add rustfmt

ENTRYPOINT ["cargo", "fmt", "--all", "--", "--check", "--config=group_imports=StdExternalCrate"]
