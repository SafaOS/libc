#!/bin/bash
# Builds the libc library
# output is in the `out` directory

target=$1
if [ -z "$target" ]; then
    echo "Usage: $0 [arch=x86_64|aarch64]"
    exit 1
fi

set -euo pipefail
crt0="src/crt0/$target.o"
salibc=$(cargo rustc --crate-type=staticlib --target "$target-unknown-none" --release --message-format=json-render-diagnostics | jq -r 'select(.reason == "compiler-artifact" and (.target.kind | index("staticlib"))) | .filenames[] | select(endswith(".a"))')

mkdir -p out
cp $salibc out/salibc.a
cp "$crt0" out/crt0.o
