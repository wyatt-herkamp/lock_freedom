#!/usr/bin/env sh

cd benchmark
FILE=../BENCHMARK.md

echo '# Benchmarks' > $FILE
echo 'Benchmark code under [benchmark][] directory.' >> $FILE
echo 'More rounds per seconds is better.' >> $FILE
echo '' >> $FILE
echo 'As you can see, there is a lot to improve!' >> $FILE
echo '' >> $FILE
echo '## QUEUE' >> $FILE
echo '```' >> $FILE
RUSTFLAGS=-g cargo run --bin queue --release >> $FILE
echo '```' >> $FILE
echo '## STACK' >> $FILE
echo '```' >> $FILE
RUSTFLAGS=-g cargo run --bin stack --release >> $FILE
echo '```' >> $FILE
echo '## MAP' >> $FILE
echo '```' >> $FILE
RUSTFLAGS=-g cargo run --bin map --release >> $FILE
echo '```' >> $FILE
