# Benchmarks
Benchmark code under [benchmark](benchmark) directory.
More rounds per seconds is better.

As you can see, there is a lot to improve!

Running on Linux 4.18.12-arch1-1-ARCH #1 SMP PREEMPT Thu Oct 4 01:01:27 UTC 2018 x86_64 GNU/Linux (4 cores)

## THREAD-LOCAL STORAGE
```
Result for 1 threads:
Target 0 (lock):
mean of 17252590.810 r/s (51757800 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 17600301.799 r/s (52801000 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (lock):
mean of 39245860.100 r/s (117737600 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 40960342.642 r/s (122881200 rounds in 3.000 seconds)

Result for 16 threads:
Target 0 (lock):
mean of 33826402.584 r/s (101480000 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 41480922.728 r/s (124443200 rounds in 3.000 seconds)

Result for 32 threads:
Target 0 (lock):
mean of 29096477.535 r/s (87292800 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 37373078.282 r/s (112121600 rounds in 3.000 seconds)

Result for 128 threads:
Target 0 (lock):
mean of 15654293.144 r/s (46963200 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 39570617.918 r/s (118720000 rounds in 3.000 seconds)

Result for 512 threads:
Target 0 (lock):
mean of 5596243.004 r/s (16793600 rounds in 3.001 seconds)
Target 1 (lockfree):
mean of 32521058.183 r/s (97587200 rounds in 3.001 seconds)

## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 13782804.785 r/s (41348500 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 11262708.022 r/s (33788200 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 5266908.668 r/s (15800800 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 17897149.452 r/s (53691600 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 4086772.710 r/s (12260400 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 4819596.374 r/s (14458800 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 10006069.361 r/s (30018400 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 5058523.542 r/s (15176000 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 5200713.741 r/s (15602400 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8676781.704 r/s (26030400 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 4477275.500 r/s (13432000 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 5932113.084 r/s (17796800 rounds in 3.000 seconds)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 13907346.538 r/s (41722100 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 10949379.506 r/s (32848200 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 9531881.181 r/s (28595700 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 17874598.993 r/s (53623800 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 3356775.372 r/s (10070400 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 8670220.977 r/s (26010800 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 9755437.829 r/s (29266800 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 4960842.004 r/s (14882800 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 7068224.924 r/s (21204800 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8612611.068 r/s (25838400 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 4406444.608 r/s (13220000 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 7575719.293 r/s (22727200 rounds in 3.000 seconds)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 2994456.308 r/s (8983500 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 1590053.597 r/s (4770200 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 1970572.699 r/s (5911800 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 2143506.372 r/s (6430600 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 1883627.757 r/s (5651200 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 2924273.705 r/s (8773200 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2158729.600 r/s (6476800 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 3739676.794 r/s (11219200 rounds in 3.000 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 3813120.308 r/s (11439400 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 2340351.071 r/s (7021100 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 2083618.723 r/s (6251000 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 3528564.584 r/s (10585800 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2117648.027 r/s (6353200 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 5145409.489 r/s (15436400 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2441569.861 r/s (7324800 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 7094348.857 r/s (21283200 rounds in 3.000 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 6419698.132 r/s (19259100 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 1246361.572 r/s (3739100 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 16291131.921 r/s (48873400 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 4419363.523 r/s (13258200 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10420847.167 r/s (31262800 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 8969380.931 r/s (26908400 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10336798.973 r/s (31010400 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 12933585.810 r/s (38801600 rounds in 3.000 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 3276300.884 r/s (9829000 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 1533573.468 r/s (4600800 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1216387.062 r/s (3649200 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 1195350.046 r/s (3586200 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1100053.981 r/s (3300400 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 1778301.598 r/s (5335200 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1161715.577 r/s (3485600 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 1973488.800 r/s (5920800 rounds in 3.000 seconds)

```
