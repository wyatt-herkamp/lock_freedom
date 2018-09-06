#!/usr/bin/env sh

cd benchmark
FILE=../BENCHMARK.md

echo '# Benchmarks' > $FILE
echo 'Benchmark code under [benchmark] directory.' >> $FILE
echo 'More rounds per seconds is better.' >> $FILE
echo '' >> $FILE
echo '## QUEUE' >> $FILE
RUSTFLAGS=-g cargo run --bin queue --release >> $FILE
echo '## STACK' >> $FILE
RUSTFLAGS=-g cargo run --bin stack --release >> $FILE
echo '## MAP' >> $FILE
RUSTFLAGS=-g cargo run --bin map --release >> $FILE
