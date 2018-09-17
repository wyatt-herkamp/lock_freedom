# Benchmarks
Benchmark code under [benchmark](benchmark) directory.
More rounds per seconds is better.

As you can see, there is a lot to improve (especially for map)!

Running on Linux 4.18.6-arch1-1-ARCH #1 SMP PREEMPT Wed Sep 5 11:54:09 UTC 2018 x86_64 GNU/Linux (4 cores)

## QUEUE
```
Result for 1 threads:
Target 0 (mutex):
mean of 5714373.261 r/s (17143200 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 13894954.726 r/s (41684900 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex):
mean of 5089733.925 r/s (15269400 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 16505418.950 r/s (49516400 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 5283526.085 r/s (15850800 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 10006711.234 r/s (30020400 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex):
mean of 5976570.326 r/s (17930400 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 8626921.489 r/s (25880800 rounds in 3.000 seconds)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex):
mean of 9308025.502 r/s (27924100 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 13707854.590 r/s (41123600 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex):
mean of 10288648.147 r/s (30866200 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 14775646.862 r/s (44327000 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 6811989.530 r/s (20436000 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 9565038.399 r/s (28695600 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex):
mean of 7251300.779 r/s (21754400 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 8317427.430 r/s (24952800 rounds in 3.000 seconds)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 1575866.542 r/s (4727700 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 2978345.447 r/s (8935200 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 2117570.000 r/s (6352800 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 2054349.432 r/s (6163200 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2905852.416 r/s (8717600 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 1882544.499 r/s (5648000 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 3727320.757 r/s (11182400 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 2129767.517 r/s (6389600 rounds in 3.000 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 2232860.750 r/s (6698600 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 3749862.753 r/s (11249600 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 3609460.016 r/s (10828400 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 2208260.159 r/s (6624800 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 5047884.234 r/s (15144000 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 2098504.203 r/s (6295600 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 6883641.659 r/s (20651200 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 2383959.867 r/s (7152000 rounds in 3.000 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 1337177.957 r/s (4011600 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 6349218.356 r/s (19047700 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 5033699.396 r/s (15101200 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 16674170.470 r/s (50022600 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 8838362.487 r/s (26515200 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 11530804.930 r/s (34592800 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 12686884.621 r/s (38061600 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 11210748.416 r/s (33632800 rounds in 3.000 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 1560556.429 r/s (4681700 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 3227228.676 r/s (9681700 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1243282.128 r/s (3730000 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 1169691.008 r/s (3509200 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1812863.362 r/s (5438800 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 1096799.358 r/s (3290400 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1999744.403 r/s (6000000 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 1157076.926 r/s (3472000 rounds in 3.001 seconds)

```
