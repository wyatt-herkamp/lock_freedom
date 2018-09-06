# Benchmarks
Benchmark code under [benchmark](benchmark) directory.
More rounds per seconds is better.

As you can see, there is a lot to improve (especially for map)!

## QUEUE
```
Result for 1 threads:
Target 0 (mutex):
mean of 7561980.239 r/s (18905000 rounds in 2.500 seconds)
Target 1 (lockfree):
mean of 36725064.720 r/s (91813000 rounds in 2.500 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 5824688.136 r/s (14564000 rounds in 2.500 seconds)
Target 1 (lockfree):
mean of 8537505.964 r/s (21344000 rounds in 2.500 seconds)

Result for 16 threads:
Target 0 (mutex):
mean of 6402724.693 r/s (16016000 rounds in 2.501 seconds)
Target 1 (lockfree):
mean of 7421332.657 r/s (18560000 rounds in 2.501 seconds)

Result for 32 threads:
Target 0 (mutex):
mean of 6131644.535 r/s (15360000 rounds in 2.505 seconds)
Target 1 (lockfree):
mean of 7429631.182 r/s (18592000 rounds in 2.502 seconds)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex):
mean of 15834303.512 r/s (39586000 rounds in 2.500 seconds)
Target 1 (lockfree):
mean of 34651758.089 r/s (86630000 rounds in 2.500 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 7416442.599 r/s (18544000 rounds in 2.500 seconds)
Target 1 (lockfree):
mean of 8367975.964 r/s (20920000 rounds in 2.500 seconds)

Result for 16 threads:
Target 0 (mutex):
mean of 7390689.051 r/s (18480000 rounds in 2.500 seconds)
Target 1 (lockfree):
mean of 7519298.961 r/s (18800000 rounds in 2.500 seconds)

Result for 32 threads:
Target 0 (mutex):
mean of 6725417.173 r/s (16832000 rounds in 2.503 seconds)
Target 1 (lockfree):
mean of 7513793.505 r/s (18816000 rounds in 2.504 seconds)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex):
mean of 472384.579 r/s (1181000 rounds in 2.500 seconds)
Target 1 (lockfree):
mean of 1041266.268 r/s (2604000 rounds in 2.501 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 1587745.981 r/s (3972000 rounds in 2.502 seconds)
Target 1 (lockfree):
mean of 510424.857 r/s (1280000 rounds in 2.508 seconds)

Result for 16 threads:
Target 0 (mutex):
mean of 2206633.936 r/s (5520000 rounds in 2.502 seconds)
Target 1 (lockfree):
mean of 504200.234 r/s (1264000 rounds in 2.507 seconds)

Result for 32 threads:
Target 0 (mutex):
mean of 2043951.260 r/s (5120000 rounds in 2.505 seconds)
Target 1 (lockfree):
mean of 497541.270 r/s (1248000 rounds in 2.508 seconds)

```
