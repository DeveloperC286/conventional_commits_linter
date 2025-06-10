FROM rust:1.87.0-alpine3.21@sha256:fa7c28576553c431224a85c897c38f3a6443bd831be37061ab3560d9e797dc82

# renovate: datasource=github-releases depName=DeveloperC286/clean_git_history
ENV CLEAN_GIT_HISTORY_VERSION="v1.0.3"
RUN wget -O - "https://github.com/DeveloperC286/clean_git_history/releases/download/${CLEAN_GIT_HISTORY_VERSION}/x86_64-unknown-linux-musl.tar.gz" | tar xz --directory "/usr/bin/"

WORKDIR /workspace

ENTRYPOINT ["clean_git_history"]
CMD ["origin/HEAD"]
