FROM rust:1.88.0-alpine3.21@sha256:b7a47e914d8c624ddb824dff64de033ff24eff7c6140b3f1408b70981aa5a751
RUN rustup component add rustfmt

ENTRYPOINT ["cargo", "fmt", "--all", "--", "--config=group_imports=StdExternalCrate"]
