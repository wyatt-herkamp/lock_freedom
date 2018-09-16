# Benchmarks
Benchmark code under [benchmark](benchmark) directory.
More rounds per seconds is better.

As you can see, there is a lot to improve (especially for map)!

Running on Linux 4.18.6-arch1-1-ARCH #1 SMP PREEMPT Wed Sep 5 11:54:09 UTC 2018 x86_64 GNU/Linux

## QUEUE
```
Result for 1 threads:
Target 0 (mutex):
mean of 5690051.368 r/s (17070200 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 13765961.243 r/s (41297900 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 5282600.430 r/s (15848000 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 9641830.221 r/s (28925600 rounds in 3.000 seconds)

Result for 16 threads:
Target 0 (mutex):
mean of 5972761.016 r/s (17918400 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 7468683.090 r/s (22406400 rounds in 3.000 seconds)

Result for 32 threads:
Target 0 (mutex):
mean of 5688034.218 r/s (17065600 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 7278403.388 r/s (21836800 rounds in 3.000 seconds)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex):
mean of 9267921.640 r/s (27803800 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 13539067.037 r/s (40617300 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 6976260.472 r/s (20928800 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 9445655.130 r/s (28337200 rounds in 3.000 seconds)

Result for 16 threads:
Target 0 (mutex):
mean of 7090291.427 r/s (21272000 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 7566893.700 r/s (22700800 rounds in 3.000 seconds)

Result for 32 threads:
Target 0 (mutex):
mean of 6524255.588 r/s (19574400 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 7321676.905 r/s (21968000 rounds in 3.000 seconds)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 1568427.978 r/s (4705300 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 2879885.698 r/s (8639700 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2981101.662 r/s (8943600 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 2156084.781 r/s (6468400 rounds in 3.000 seconds)

Result for 16 threads:
Target 0 (mutex insert):
mean of 4151288.439 r/s (12454400 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 2453601.295 r/s (7361600 rounds in 3.000 seconds)

Result for 32 threads:
Target 0 (mutex insert):
mean of 4038894.761 r/s (12118400 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 2488968.297 r/s (7468800 rounds in 3.001 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 2158821.081 r/s (6476500 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 4514167.358 r/s (13542600 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 5236571.024 r/s (15710000 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 2446283.385 r/s (7339200 rounds in 3.000 seconds)

Result for 16 threads:
Target 0 (mutex get):
mean of 7452728.807 r/s (22358400 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 2831465.902 r/s (8494400 rounds in 3.000 seconds)

Result for 32 threads:
Target 0 (mutex get):
mean of 7261108.822 r/s (21785600 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 2852021.704 r/s (8556800 rounds in 3.000 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 1369724.946 r/s (4109200 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 7555796.759 r/s (22667400 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 7182772.356 r/s (21548400 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 10491367.093 r/s (31474400 rounds in 3.000 seconds)

Result for 16 threads:
Target 0 (mutex remove):
mean of 15939620.794 r/s (47819200 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 9491760.791 r/s (28476800 rounds in 3.000 seconds)

Result for 32 threads:
Target 0 (mutex remove):
mean of 15807338.679 r/s (47424000 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 8320577.170 r/s (24963200 rounds in 3.000 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 1432749.098 r/s (4298300 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 2508652.948 r/s (7526000 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1678468.756 r/s (5035600 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 1297351.079 r/s (3892400 rounds in 3.000 seconds)

Result for 16 threads:
Target 0 (mutex mixed):
mean of 2044575.499 r/s (6134400 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 1227461.947 r/s (3683200 rounds in 3.001 seconds)

Result for 32 threads:
Target 0 (mutex mixed):
mean of 1894393.282 r/s (5683200 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 1334391.281 r/s (4006400 rounds in 3.002 seconds)

```
