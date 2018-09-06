#!/usr/bin/env sh

if [ $# -eq 0 ]
then
    echo "Expected binary name to profile"
    exit 1
fi

pushd benchmark > /dev/null || exit 1
RUSTFLAGS=-g cargo build --release --bin "$1" || exit 1
popd > /dev/null || exit 1

export PROFILING=1

perf record --call-graph=dwarf -F 2500 benchmark/target/release/"$1" \
    || exit 1
