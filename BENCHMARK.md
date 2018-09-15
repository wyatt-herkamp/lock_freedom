# Benchmarks
Benchmark code under [benchmark](benchmark) directory.
More rounds per seconds is better.

As you can see, there is a lot to improve!

## QUEUE
```
Result for 1 threads:
Target 0 (mutex):
mean of 5823570.575 r/s (17470800 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 14013044.633 r/s (42039200 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 5356659.128 r/s (16070000 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 9749862.150 r/s (29250000 rounds in 3.000 seconds)

Result for 16 threads:
Target 0 (mutex):
mean of 6043471.035 r/s (18131200 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 7693841.064 r/s (23081600 rounds in 3.000 seconds)

Result for 32 threads:
Target 0 (mutex):
mean of 5731671.805 r/s (17196800 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 7473878.935 r/s (22422400 rounds in 3.000 seconds)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex):
mean of 9479061.975 r/s (28437200 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 13814949.937 r/s (41444900 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 6897873.408 r/s (20694000 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 9578042.374 r/s (28734400 rounds in 3.000 seconds)

Result for 16 threads:
Target 0 (mutex):
mean of 7073010.693 r/s (21219200 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 7684243.919 r/s (23054400 rounds in 3.000 seconds)

Result for 32 threads:
Target 0 (mutex):
mean of 6416647.692 r/s (19251200 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 7474216.323 r/s (22425600 rounds in 3.000 seconds)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex):
mean of 766666.328 r/s (2300000 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 821561.476 r/s (2464700 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 1545408.201 r/s (4636400 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 485746.760 r/s (1457600 rounds in 3.001 seconds)

Result for 16 threads:
Target 0 (mutex):
mean of 2215744.870 r/s (6648000 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 492013.985 r/s (1476800 rounds in 3.002 seconds)

Result for 32 threads:
Target 0 (mutex):
mean of 2077051.328 r/s (6233600 rounds in 3.001 seconds)
Target 1 (lockfree):
mean of 490244.601 r/s (1472000 rounds in 3.003 seconds)

```
