#!/bin/bash

set -xe

rustc \
    --target=wasm32-unknown-emscripten \
    --out-dir target \
    -g \
    -C opt-level=3 \
    -C linker=./linker.sh \
    -C panic=abort \
    -C lto \
    --verbose \
    main.rs
