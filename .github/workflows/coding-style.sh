#!/usr/bin/env bash

# download rust
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

SOURCES=$(find $(git rev-parse --show-toplevel) | egrep "\.rs\$")

set -x

format_error=0
for file in ${SOURCES};
do
    rustfmt --check ${file}
    if [ $? -ne 0 ]; then
        format_error=1
    fi
done
exit ${format_error}
