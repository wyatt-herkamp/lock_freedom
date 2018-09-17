#!/usr/bin/env sh


export LSAN_OPTIONS='fast_unwind_on_malloc=0'
export ASAN_OPTIONS='fast_unwind_on_malloc=0'

cargo fuzz run $@ -- -trace_malloc=[12] -max_len=1048576
