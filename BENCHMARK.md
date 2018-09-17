# Benchmarks
Benchmark code under [benchmark](benchmark) directory.
More rounds per seconds is better.

As you can see, there is a lot to improve (especially for map)!

Running on Linux 4.18.6-arch1-1-ARCH #1 SMP PREEMPT Wed Sep 5 11:54:09 UTC 2018 x86_64 GNU/Linux (4 cores)

## QUEUE
```
Result for 1 threads:
Target 0 (mutex):
mean of 5667029.624 r/s (17002000 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 13820066.118 r/s (41460200 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 5262953.303 r/s (15789200 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 8907904.201 r/s (26724000 rounds in 3.000 seconds)

Result for 16 threads:
Target 0 (mutex):
mean of 5914295.024 r/s (17744000 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 6962040.850 r/s (20886400 rounds in 3.000 seconds)

Result for 32 threads:
Target 0 (mutex):
mean of 5693593.646 r/s (17081600 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 6785674.086 r/s (20358400 rounds in 3.000 seconds)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex):
mean of 9256678.694 r/s (27770100 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 13583329.802 r/s (40750000 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 6701604.377 r/s (20105200 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 8767302.212 r/s (26302000 rounds in 3.000 seconds)

Result for 16 threads:
Target 0 (mutex):
mean of 6606755.847 r/s (19820800 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 6751847.773 r/s (20256000 rounds in 3.000 seconds)

Result for 32 threads:
Target 0 (mutex):
mean of 6340973.730 r/s (19024000 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 6572368.344 r/s (19724800 rounds in 3.001 seconds)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 1546822.950 r/s (4640500 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 2768064.780 r/s (8585600 rounds in 3.102 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2990331.981 r/s (8971200 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 1905444.818 r/s (5716400 rounds in 3.000 seconds)

Result for 16 threads:
Target 0 (mutex insert):
mean of 4146519.627 r/s (12440000 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 2254237.873 r/s (6763200 rounds in 3.000 seconds)

Result for 32 threads:
Target 0 (mutex insert):
mean of 3664558.369 r/s (10995200 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 2300808.071 r/s (6905600 rounds in 3.001 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 2075310.733 r/s (6226000 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 4503296.926 r/s (13509900 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 5473045.453 r/s (16419200 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 2310186.543 r/s (6930800 rounds in 3.000 seconds)

Result for 16 threads:
Target 0 (mutex get):
mean of 8125335.419 r/s (24377600 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 2481015.739 r/s (7443200 rounds in 3.000 seconds)

Result for 32 threads:
Target 0 (mutex get):
mean of 8400802.257 r/s (25203200 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 2647961.588 r/s (7945600 rounds in 3.001 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 1313004.141 r/s (3939100 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 7522592.826 r/s (22567800 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 6915863.591 r/s (20747600 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 10417260.492 r/s (31252000 rounds in 3.000 seconds)

Result for 16 threads:
Target 0 (mutex remove):
mean of 16281045.751 r/s (48843200 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 8572518.393 r/s (25718400 rounds in 3.000 seconds)

Result for 32 threads:
Target 0 (mutex remove):
mean of 16278081.308 r/s (48835200 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 8390048.756 r/s (25171200 rounds in 3.000 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 1538808.343 r/s (4616500 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 3446267.080 r/s (10338900 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1730807.510 r/s (5192800 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 1251241.698 r/s (3754000 rounds in 3.000 seconds)

Result for 16 threads:
Target 0 (mutex mixed):
mean of 1979155.786 r/s (5937600 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 1287889.569 r/s (3864000 rounds in 3.000 seconds)

Result for 32 threads:
Target 0 (mutex mixed):
mean of 1911512.152 r/s (5737600 rounds in 3.002 seconds)
Target 1 (lockfree mixed):
mean of 1291995.191 r/s (3878400 rounds in 3.002 seconds)

```
