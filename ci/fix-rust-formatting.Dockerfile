FROM rust:1.88.0-alpine3.21@sha256:54e937b1530d435dc83b94f5a61ef08365127f2fefbb3789712c5d6f55bbb58c
RUN rustup component add rustfmt

ENTRYPOINT ["cargo", "fmt", "--all", "--", "--config=group_imports=StdExternalCrate"]
