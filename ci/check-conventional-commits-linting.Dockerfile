FROM rust:1.88.0-alpine3.21@sha256:9c6a4baf58661f99a5441b15e3ad8295dabf35e849c4935e77ad35d9809be1d2

# renovate: datasource=github-releases depName=DeveloperC286/conventional_commits_linter
ENV CONVENTIONAL_COMMITS_LINTER_VERSION="v0.15.0"
RUN wget -O - "https://github.com/DeveloperC286/conventional_commits_linter/releases/download/${CONVENTIONAL_COMMITS_LINTER_VERSION}/x86_64-unknown-linux-musl.gz" | gzip -d >/usr/bin/conventional_commits_linter && chmod 755 /usr/bin/conventional_commits_linter

WORKDIR /workspace

ENTRYPOINT ["conventional_commits_linter", "--allow-angular-type-only", "--from-reference"]
CMD ["origin/HEAD"]
