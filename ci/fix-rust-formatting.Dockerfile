FROM rust:1.90.0-alpine3.21@sha256:3757b14ddcc2057eb91a074dcdd0913bed839b22444bd2229a49eea910ed8736
RUN rustup component add rustfmt

ENTRYPOINT ["cargo", "fmt", "--all", "--", "--config=group_imports=StdExternalCrate"]
