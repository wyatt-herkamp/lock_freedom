# Benchmarks
Benchmark code under [benchmark](benchmark) directory.
More rounds per seconds is better.

As you can see, there is a lot to improve (especially for map)!

Running on Linux 4.18.6-arch1-1-ARCH #1 SMP PREEMPT Wed Sep 5 11:54:09 UTC 2018 x86_64 GNU/Linux (4 cores)

## QUEUE
```
Result for 1 threads:
Target 0 (mutex):
mean of 5614518.387 r/s (16843600 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 13862838.345 r/s (41588600 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 5235153.330 r/s (15705600 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 9582240.935 r/s (28747200 rounds in 3.000 seconds)

Result for 16 threads:
Target 0 (mutex):
mean of 5920433.931 r/s (17761600 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 7410661.783 r/s (22232000 rounds in 3.000 seconds)

Result for 32 threads:
Target 0 (mutex):
mean of 5622074.702 r/s (16876800 rounds in 3.002 seconds)
Target 1 (lockfree):
mean of 7192228.961 r/s (21577600 rounds in 3.000 seconds)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex):
mean of 9270426.822 r/s (27811300 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 13587389.488 r/s (40762200 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 6832074.152 r/s (20496400 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 9319367.602 r/s (27958400 rounds in 3.000 seconds)

Result for 16 threads:
Target 0 (mutex):
mean of 6869895.869 r/s (20611200 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 7511720.264 r/s (22536000 rounds in 3.000 seconds)

Result for 32 threads:
Target 0 (mutex):
mean of 6382981.140 r/s (19152000 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 7341156.261 r/s (22025600 rounds in 3.000 seconds)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 1547826.600 r/s (4643800 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 2969805.870 r/s (8909700 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2945904.654 r/s (8838000 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 1832324.207 r/s (5497200 rounds in 3.000 seconds)

Result for 16 threads:
Target 0 (mutex insert):
mean of 4140572.503 r/s (12422400 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 2183850.368 r/s (6552000 rounds in 3.000 seconds)

Result for 32 threads:
Target 0 (mutex insert):
mean of 3974763.969 r/s (11926400 rounds in 3.001 seconds)
Target 1 (lockfree insert):
mean of 2221918.238 r/s (6668800 rounds in 3.001 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 2167332.797 r/s (6502000 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 3701642.600 r/s (11105000 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 5415712.507 r/s (16247600 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 2040954.132 r/s (6123200 rounds in 3.000 seconds)

Result for 16 threads:
Target 0 (mutex get):
mean of 8552897.946 r/s (25659200 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 2479133.103 r/s (7438400 rounds in 3.000 seconds)

Result for 32 threads:
Target 0 (mutex get):
mean of 8328087.253 r/s (24985600 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 2524557.766 r/s (7574400 rounds in 3.000 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 1359161.564 r/s (4077600 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 6324893.146 r/s (18974700 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 7254462.931 r/s (21763600 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 10418467.648 r/s (31255600 rounds in 3.000 seconds)

Result for 16 threads:
Target 0 (mutex remove):
mean of 16175562.440 r/s (48528000 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 9635098.909 r/s (28905600 rounds in 3.000 seconds)

Result for 32 threads:
Target 0 (mutex remove):
mean of 16305979.427 r/s (48918400 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 9490894.406 r/s (28473600 rounds in 3.000 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 1532702.031 r/s (4598200 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 2950052.957 r/s (9437200 rounds in 3.199 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1713152.557 r/s (5139600 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 1383583.058 r/s (4150800 rounds in 3.000 seconds)

Result for 16 threads:
Target 0 (mutex mixed):
mean of 2050947.741 r/s (6153600 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 1485825.303 r/s (4457600 rounds in 3.000 seconds)

Result for 32 threads:
Target 0 (mutex mixed):
mean of 1918372.392 r/s (5756800 rounds in 3.001 seconds)
Target 1 (lockfree mixed):
mean of 1485536.377 r/s (4457600 rounds in 3.001 seconds)

```
