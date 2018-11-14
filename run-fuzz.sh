#!/usr/bin/env sh

run () {
    cargo fuzz run \
          $@ -- \
          -trace_malloc=[12] \
          -max_len=268435456 \
          $LFUZ_OPTIONS
}

export LSAN_OPTIONS='fast_unwind_on_malloc=0'
export ASAN_OPTIONS='fast_unwind_on_malloc=0'

echo "============ Starting fuzz $(date) ============" >> fuzz.log
echo '' >> fuzz.log

COND=true

while $COND
do
    # We are trying to tee stdout and stderr separately.
    # Saint Mother of Redirection
    { run $@ 2>&1 1>&3 | tee fuzz.log 1>&2 ; } 3>&1 | tee fuzz.log
    # COND=false
done
