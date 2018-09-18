# Benchmarks
Benchmark code under [benchmark](benchmark) directory.
More rounds per seconds is better.

As you can see, there is a lot to improve (especially for map)!

Running on Linux 4.18.6-arch1-1-ARCH #1 SMP PREEMPT Wed Sep 5 11:54:09 UTC 2018 x86_64 GNU/Linux (4 cores)

## QUEUE
```
Result for 1 threads:
Target 0 (mutex):
mean of 5685278.737 r/s (17055900 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 13696814.245 r/s (41090500 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex):
mean of 5159783.260 r/s (15479400 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 16355065.745 r/s (49065200 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 5230533.143 r/s (15691600 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 9758095.869 r/s (29274400 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex):
mean of 5910858.191 r/s (17732800 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 8367072.561 r/s (25101600 rounds in 3.000 seconds)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex):
mean of 9290998.972 r/s (27873000 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 13541335.458 r/s (40624100 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex):
mean of 10199671.649 r/s (30599200 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 14603307.753 r/s (43810000 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 7049001.952 r/s (21147200 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 9541187.019 r/s (28624000 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex):
mean of 7527453.443 r/s (22582400 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 8227472.262 r/s (24683200 rounds in 3.000 seconds)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 1568225.930 r/s (4704700 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 2799695.196 r/s (8415100 rounds in 3.006 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 2125668.875 r/s (6377200 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 2190343.074 r/s (6571200 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2893119.138 r/s (8679600 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 2156554.756 r/s (6470000 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 3701033.585 r/s (11104000 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 2386031.362 r/s (7158400 rounds in 3.000 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 2291995.448 r/s (6876100 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 4513932.641 r/s (13541800 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 3576530.668 r/s (10729600 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 2399392.973 r/s (7198200 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 5029570.721 r/s (15088800 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 2451019.909 r/s (7353200 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 6878913.269 r/s (20637600 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 2735485.060 r/s (8207200 rounds in 3.000 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 1348680.811 r/s (4046100 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 7657297.871 r/s (22971900 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 5104797.796 r/s (15314400 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 14862435.936 r/s (44587400 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 8825193.534 r/s (26475600 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 10295181.750 r/s (30885600 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 12642922.587 r/s (37928800 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 10259120.509 r/s (30777600 rounds in 3.000 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 1559333.598 r/s (4678100 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 3427968.692 r/s (10284000 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1214195.357 r/s (3642600 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 1624915.496 r/s (4874800 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1808152.407 r/s (5424800 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 1353399.351 r/s (4060400 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1969125.751 r/s (5908000 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 1449414.399 r/s (4348800 rounds in 3.000 seconds)

```
