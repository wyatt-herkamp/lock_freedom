# Benchmarks
Benchmark code under [benchmark](benchmark) directory.
More rounds per seconds is better.

As you can see, there is a lot to improve (especially for map)!

Running on Linux 4.18.6-arch1-1-ARCH #1 SMP PREEMPT Wed Sep 5 11:54:09 UTC 2018 x86_64 GNU/Linux

## QUEUE
```
Result for 1 threads:
Target 0 (mutex):
mean of 5747335.418 r/s (17242100 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 13919963.595 r/s (41759900 rounds in 3.000 seconds)

