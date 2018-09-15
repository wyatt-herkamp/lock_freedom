#!/usr/bin/env sh

cd benchmark
FILE=../BENCHMARK.md

echo '# Benchmarks' > $FILE
echo 'Benchmark code under [benchmark](benchmark) directory.' >> $FILE
echo 'More rounds per seconds is better.' >> $FILE
echo '' >> $FILE
echo 'As you can see, there is a lot to improve (especially for map)!' >> $FILE
echo '' >> $FILE
echo 'Running on' $(uname -s -r -v -m -o) >> $FILE
echo '' >> $FILE
echo '## QUEUE' >> $FILE
echo '```' >> $FILE
cargo run --bin queue --release >> $FILE || exit 1
echo '```' >> $FILE
echo '## STACK' >> $FILE
echo '```' >> $FILE
cargo run --bin stack --release >> $FILE || exit 1
echo '```' >> $FILE
echo '## MAP' >> $FILE
echo '```' >> $FILE
cargo run --bin map --release >> $FILE || exit 1
echo '```' >> $FILE
