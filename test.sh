#!/usr/bin/env sh

test_with_toolchain() {
    cargo $1 test --target x86_64-unknown-linux-gnu -- --test-threads=1 && \
    cargo $1 test --release --target x86_64-unknown-linux-gnu -- --test-threads=1
}

test_with_toolchain +stable && \
test_with_toolchain +nightly && \

export RUSTFLAGS='-Z sanitizer=leak' && \
export LSAN_OPTIONS='fast_unwind_on_malloc=0' && \

test_with_toolchain +nightly
