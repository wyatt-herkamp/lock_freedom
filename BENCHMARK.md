# Benchmarks
Benchmark code under [benchmark](benchmark) directory.
More rounds per seconds is better.

As you can see, there is a lot to improve!

## QUEUE
```
Result for 1 threads:
Target 0 (mutex):
mean of 7045524.582 r/s (17614000 rounds in 2.500 seconds)
Target 1 (lockfree):
mean of 36791703.871 r/s (91980000 rounds in 2.500 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 5835607.669 r/s (14592000 rounds in 2.501 seconds)
Target 1 (lockfree):
mean of 8492612.918 r/s (21232000 rounds in 2.500 seconds)

Result for 16 threads:
Target 0 (mutex):
mean of 6540207.678 r/s (16368000 rounds in 2.503 seconds)
Target 1 (lockfree):
mean of 7527737.846 r/s (18832000 rounds in 2.502 seconds)

Result for 32 threads:
Target 0 (mutex):
mean of 6288076.904 r/s (15744000 rounds in 2.504 seconds)
Target 1 (lockfree):
mean of 7490009.539 r/s (18752000 rounds in 2.504 seconds)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex):
mean of 15896525.353 r/s (39742000 rounds in 2.500 seconds)
Target 1 (lockfree):
mean of 35117231.480 r/s (87794000 rounds in 2.500 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 7937172.082 r/s (19844000 rounds in 2.500 seconds)
Target 1 (lockfree):
mean of 8352512.339 r/s (20884000 rounds in 2.500 seconds)

Result for 16 threads:
Target 0 (mutex):
mean of 7465527.674 r/s (18672000 rounds in 2.501 seconds)
Target 1 (lockfree):
mean of 7513533.938 r/s (18784000 rounds in 2.500 seconds)

Result for 32 threads:
Target 0 (mutex):
mean of 6833659.975 r/s (17088000 rounds in 2.501 seconds)
Target 1 (lockfree):
mean of 7478796.709 r/s (18720000 rounds in 2.503 seconds)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex):
mean of 481363.279 r/s (1204000 rounds in 2.501 seconds)
Target 1 (lockfree):
mean of 880789.814 r/s (2203000 rounds in 2.501 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 1607858.288 r/s (4020000 rounds in 2.500 seconds)
Target 1 (lockfree):
mean of 436442.110 r/s (1092000 rounds in 2.502 seconds)

Result for 16 threads:
Target 0 (mutex):
mean of 2278156.922 r/s (5696000 rounds in 2.500 seconds)
Target 1 (lockfree):
mean of 485059.679 r/s (1216000 rounds in 2.507 seconds)

Result for 32 threads:
Target 0 (mutex):
mean of 2056958.690 r/s (5152000 rounds in 2.505 seconds)
Target 1 (lockfree):
mean of 477546.046 r/s (1216000 rounds in 2.546 seconds)

```
