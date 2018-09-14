#!/usr/bin/env sh

if [ $# -eq 0 ]
then
    echo "Expected binary name to profile"
    exit 1
fi

pushd profiling > /dev/null || exit 1
RUSTFLAGS=-g cargo build --release --bin "$1" || exit 1
popd > /dev/null || exit 1

vallgrind --tool=callgrind --demangle=yes ./profiling/target/release/"$1" \
    || exit 1
