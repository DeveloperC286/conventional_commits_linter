#!/usr/bin/env sh

set -o errexit
set -o xtrace

github_cli_version="2.30.0"
target="gh_${github_cli_version}_linux_amd64"

wget "https://github.com/cli/cli/releases/download/v${github_cli_version}/${target}.tar.gz"
tar -xzvf "${target}.tar.gz"
mv "${target}/bin/gh" "/usr/local/bin/"
rm -rf "${target:?}/"
