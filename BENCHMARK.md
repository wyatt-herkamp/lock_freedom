# Benchmarks
Benchmark code under [benchmark](benchmark) directory.
More rounds per seconds is better.

As you can see, there is a lot to improve (especially for map)!

## QUEUE
```
Result for 1 threads:
Target 0 (mutex):
mean of 7907095.968 r/s (19768000 rounds in 2.500 seconds)
Target 1 (lockfree):
mean of 36722042.797 r/s (91806000 rounds in 2.500 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 5842980.330 r/s (14608000 rounds in 2.500 seconds)
Target 1 (lockfree):
mean of 8565735.254 r/s (21416000 rounds in 2.500 seconds)

Result for 16 threads:
Target 0 (mutex):
mean of 6478328.820 r/s (16208000 rounds in 2.502 seconds)
Target 1 (lockfree):
mean of 7528351.016 r/s (18832000 rounds in 2.501 seconds)

Result for 32 threads:
Target 0 (mutex):
mean of 6231611.017 r/s (15584000 rounds in 2.501 seconds)
Target 1 (lockfree):
mean of 7510128.761 r/s (18784000 rounds in 2.501 seconds)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex):
mean of 15786490.124 r/s (39467000 rounds in 2.500 seconds)
Target 1 (lockfree):
mean of 34789013.044 r/s (86973000 rounds in 2.500 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 7703704.495 r/s (19260000 rounds in 2.500 seconds)
Target 1 (lockfree):
mean of 8523047.099 r/s (21312000 rounds in 2.501 seconds)

Result for 16 threads:
Target 0 (mutex):
mean of 7447785.979 r/s (18624000 rounds in 2.501 seconds)
Target 1 (lockfree):
mean of 7679960.765 r/s (19216000 rounds in 2.502 seconds)

Result for 32 threads:
Target 0 (mutex):
mean of 6832753.396 r/s (17088000 rounds in 2.501 seconds)
Target 1 (lockfree):
mean of 7659306.561 r/s (19168000 rounds in 2.503 seconds)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex):
mean of 471515.159 r/s (1179000 rounds in 2.500 seconds)
Target 1 (lockfree):
mean of 1047429.140 r/s (2619000 rounds in 2.500 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 1577379.292 r/s (3944000 rounds in 2.500 seconds)
Target 1 (lockfree):
mean of 509087.590 r/s (1276000 rounds in 2.506 seconds)

Result for 16 threads:
Target 0 (mutex):
mean of 2183162.160 r/s (5472000 rounds in 2.506 seconds)
Target 1 (lockfree):
mean of 503059.260 r/s (1264000 rounds in 2.513 seconds)

Result for 32 threads:
Target 0 (mutex):
mean of 2008777.025 r/s (5024000 rounds in 2.501 seconds)
Target 1 (lockfree):
mean of 494882.893 r/s (1248000 rounds in 2.522 seconds)

```
