FROM rust:1.87.0-alpine3.21@sha256:fa7c28576553c431224a85c897c38f3a6443bd831be37061ab3560d9e797dc82

# renovate: datasource=github-releases depName=DeveloperC286/conventional_commits_linter
ENV CONVENTIONAL_COMMITS_LINTER_VERSION="v0.14.3"
RUN wget -O - "https://github.com/DeveloperC286/conventional_commits_linter/releases/download/${CONVENTIONAL_COMMITS_LINTER_VERSION}/x86_64-unknown-linux-musl.gz" | gzip -d >/usr/bin/conventional_commits_linter && chmod 755 /usr/bin/conventional_commits_linter

WORKDIR /workspace

ENTRYPOINT ["conventional_commits_linter", "--allow-angular-type-only", "--from-reference"]
CMD ["origin/HEAD"]
