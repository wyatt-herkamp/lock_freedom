# Benchmarks
Benchmark code under [benchmark](benchmark) directory.
More rounds per seconds is better.

As you can see, there is a lot to improve (especially for map)!

Running on Linux 4.18.12-arch1-1-ARCH #1 SMP PREEMPT Thu Oct 4 01:01:27 UTC 2018 x86_64 GNU/Linux (4 cores)

## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 14008769.223 r/s (42026400 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 11301396.176 r/s (33904200 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 5266028.562 r/s (15798100 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 16162859.663 r/s (48488600 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 4632876.258 r/s (13898800 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 4807419.639 r/s (14422400 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 9667274.779 r/s (29002000 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 5038310.793 r/s (15115200 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 5182021.111 r/s (15546400 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8276548.520 r/s (24830400 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 4478785.704 r/s (13436800 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 5902739.076 r/s (17708800 rounds in 3.000 seconds)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 13899054.755 r/s (41697200 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 10946933.892 r/s (32840900 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 9495781.281 r/s (28487400 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 16690857.167 r/s (50072800 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 4420429.548 r/s (13261400 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 12394718.620 r/s (37184400 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 9631499.919 r/s (28894800 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 5001926.450 r/s (15006000 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 7061495.097 r/s (21184800 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8474501.790 r/s (25424000 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 4435435.851 r/s (13306400 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 7653828.405 r/s (22961600 rounds in 3.000 seconds)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 2995830.214 r/s (8987600 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 1584266.480 r/s (4752900 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 2013950.697 r/s (6042000 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 2142233.385 r/s (6426800 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 1875137.530 r/s (5625600 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 2927215.011 r/s (8782000 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2151544.156 r/s (6455200 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 3756616.442 r/s (11270400 rounds in 3.000 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 3778177.747 r/s (11334600 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 2330676.655 r/s (6992100 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 2074672.787 r/s (6224200 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 3532053.337 r/s (10596200 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2088995.575 r/s (6267200 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 5128119.641 r/s (15384400 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2402947.048 r/s (7209600 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 7081828.767 r/s (21245600 rounds in 3.000 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 6415866.179 r/s (19247600 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 1241109.371 r/s (3723400 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 15262305.342 r/s (45787000 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 4407303.201 r/s (13222000 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10322342.842 r/s (30967200 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 8933752.067 r/s (26801600 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10242132.179 r/s (30726400 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 12926780.815 r/s (38780800 rounds in 3.000 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 3270595.500 r/s (9811900 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 1530399.089 r/s (4591200 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1225458.349 r/s (3676400 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 1188398.328 r/s (3565200 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1105964.770 r/s (3318000 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 1772122.053 r/s (5316400 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1168821.382 r/s (3507200 rounds in 3.001 seconds)
Target 1 (lockfree mixed):
mean of 1972352.203 r/s (5917600 rounds in 3.000 seconds)

```
