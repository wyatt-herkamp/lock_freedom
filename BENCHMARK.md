# Benchmarks
Benchmark code under [benchmark](benchmark) directory.
More rounds per seconds is better.

As you can see, there is a lot to improve (especially for map)!

Running on Linux 4.18.6-arch1-1-ARCH #1 SMP PREEMPT Wed Sep 5 11:54:09 UTC 2018 x86_64 GNU/Linux

## QUEUE
```
Result for 1 threads:
Target 0 (mutex):
mean of 5737181.844 r/s (17211600 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 14011798.912 r/s (42035400 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 5295235.957 r/s (15886000 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 9848454.385 r/s (29545600 rounds in 3.000 seconds)

Result for 16 threads:
Target 0 (mutex):
mean of 6046661.887 r/s (18140800 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 7745023.589 r/s (23235200 rounds in 3.000 seconds)

Result for 32 threads:
Target 0 (mutex):
mean of 5703839.737 r/s (17113600 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 7530025.841 r/s (22592000 rounds in 3.000 seconds)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex):
mean of 9400802.273 r/s (28202500 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 13728686.331 r/s (41186100 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 6903166.046 r/s (20709600 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 9713189.821 r/s (29139600 rounds in 3.000 seconds)

Result for 16 threads:
Target 0 (mutex):
mean of 7089290.992 r/s (21268800 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 7781919.402 r/s (23347200 rounds in 3.000 seconds)

Result for 32 threads:
Target 0 (mutex):
mean of 6462319.941 r/s (19388800 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 7002693.513 r/s (21011200 rounds in 3.000 seconds)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex):
mean of 715831.335 r/s (2147500 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 802440.978 r/s (2407400 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 1546693.997 r/s (4640400 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 474261.954 r/s (1422800 rounds in 3.000 seconds)

Result for 16 threads:
Target 0 (mutex):
mean of 1947406.331 r/s (5843200 rounds in 3.001 seconds)
Target 1 (lockfree):
mean of 487771.229 r/s (1464000 rounds in 3.001 seconds)

Result for 32 threads:
Target 0 (mutex):
mean of 1794539.926 r/s (5385600 rounds in 3.001 seconds)
Target 1 (lockfree):
mean of 479323.392 r/s (1440000 rounds in 3.004 seconds)

```
