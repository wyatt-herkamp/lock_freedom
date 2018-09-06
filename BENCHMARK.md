# Benchmarks
Benchmark code under [benchmark] directory.
More rounds per seconds is better.

As you can see, there is a lot to improve!

## QUEUE
```
Result for 1 threads:
Target 0 (mutex):
mean of 7578326.078 r/s (18946000 rounds in 2.500)
Target 1 (lockfree):
mean of 35500203.442 r/s (88751000 rounds in 2.500)

Result for 4 threads:
Target 0 (mutex):
mean of 5716165.323 r/s (14292000 rounds in 2.500)
Target 1 (lockfree):
mean of 8733897.130 r/s (21836000 rounds in 2.500)

Result for 8 threads:
Target 0 (mutex):
mean of 6717495.239 r/s (16800000 rounds in 2.501)
Target 1 (lockfree):
mean of 7823488.012 r/s (19560000 rounds in 2.500)

Result for 16 threads:
Target 0 (mutex):
mean of 6492341.553 r/s (16240000 rounds in 2.501)
Target 1 (lockfree):
mean of 7670574.833 r/s (19184000 rounds in 2.501)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex):
mean of 15668846.774 r/s (39173000 rounds in 2.500)
Target 1 (lockfree):
mean of 34397331.783 r/s (85994000 rounds in 2.500)

Result for 4 threads:
Target 0 (mutex):
mean of 7847683.051 r/s (19620000 rounds in 2.500)
Target 1 (lockfree):
mean of 8161913.940 r/s (20408000 rounds in 2.500)

Result for 16 threads:
Target 0 (mutex):
mean of 7453866.289 r/s (18640000 rounds in 2.501)
Target 1 (lockfree):
mean of 7308851.102 r/s (18288000 rounds in 2.502)

Result for 32 threads:
Target 0 (mutex):
mean of 7068167.202 r/s (17696000 rounds in 2.504)
Target 1 (lockfree):
mean of 7316314.402 r/s (18304000 rounds in 2.502)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex):
mean of 473941.985 r/s (1185000 rounds in 2.500)
Target 1 (lockfree):
mean of 1062560.318 r/s (2657000 rounds in 2.501)

Result for 4 threads:
Target 0 (mutex):
mean of 1572685.432 r/s (3932000 rounds in 2.500)
Target 1 (lockfree):
mean of 508429.690 r/s (1272000 rounds in 2.502)

Result for 8 threads:
Target 0 (mutex):
mean of 2001115.586 r/s (5008000 rounds in 2.503)
Target 1 (lockfree):
mean of 505892.302 r/s (1272000 rounds in 2.514)

Result for 16 threads:
Target 0 (mutex):
mean of 2081918.688 r/s (5216000 rounds in 2.505)
Target 1 (lockfree):
mean of 500533.160 r/s (1264000 rounds in 2.525)

```
