FROM rust:1.88.0-alpine3.21@sha256:0c004cf201a139a3ce08f10cb200402e59499352a21bc01e1b1a1351fcd45c8a
RUN rustup component add rustfmt

ENTRYPOINT ["cargo", "fmt", "--all", "--", "--config=group_imports=StdExternalCrate"]
