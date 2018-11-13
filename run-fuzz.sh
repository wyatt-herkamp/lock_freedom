#!/usr/bin/env sh


export LSAN_OPTIONS='fast_unwind_on_malloc=0'
export ASAN_OPTIONS='fast_unwind_on_malloc=0'

echo "============ Starting fuzz $(date) ============" >> fuzz.log

while true
do
    cargo fuzz run $@ -- \
          -trace_malloc=[12] \
          -max_len=268435456 \
          $LFUZ_OPTIONS \
          1>>fuzz.log 2>>fuzz.log
done
