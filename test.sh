#!/usr/bin/env sh

RUSTFLAGS='-Z sanitizer=leak' \
    LSAN_OPTIONS='fast_unwind_on_malloc=0' \
    cargo test --target x86_64-unknown-linux-gnu -- --test-threads=1
