#!/usr/bin/env sh

set -e

test_with_toolchain() {
    cargo $1 test --target x86_64-unknown-linux-gnu -- --nocapture
    cargo $1 test --release --target x86_64-unknown-linux-gnu -- --nocapture
}

test_with_toolchain +stable
test_with_toolchain +nightly

export RUSTFLAGS='-Z sanitizer=leak'
export LSAN_OPTIONS='fast_unwind_on_malloc=0'

test_with_toolchain +nightly
