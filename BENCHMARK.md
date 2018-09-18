# Benchmarks
Benchmark code under [benchmark](benchmark) directory.
More rounds per seconds is better.

As you can see, there is a lot to improve (especially for map)!

Running on Linux 4.18.6-arch1-1-ARCH #1 SMP PREEMPT Wed Sep 5 11:54:09 UTC 2018 x86_64 GNU/Linux (4 cores)

## QUEUE
```
Result for 1 threads:
Target 0 (mutex):
mean of 13892417.491 r/s (41677300 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 5772194.699 r/s (17316600 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex):
mean of 16135109.265 r/s (48405400 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 5512131.667 r/s (16536400 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 9986235.749 r/s (29958800 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 5267242.479 r/s (15802000 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex):
mean of 8602168.453 r/s (25807200 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 5938198.186 r/s (17815200 rounds in 3.000 seconds)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex):
mean of 13592295.578 r/s (40776900 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 9272572.411 r/s (27817800 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex):
mean of 13912017.627 r/s (41736200 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 10451541.569 r/s (31354800 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 9612907.391 r/s (28838800 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 6788247.856 r/s (20364800 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex):
mean of 8366318.833 r/s (25099200 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 7215585.053 r/s (21647200 rounds in 3.000 seconds)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 2970964.738 r/s (8912900 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 1588733.252 r/s (4766200 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 1989142.513 r/s (5967600 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 2177356.163 r/s (6532200 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 1845394.758 r/s (5536400 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 2938633.056 r/s (8816000 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2101691.690 r/s (6305600 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 3765594.456 r/s (11296800 rounds in 3.000 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 3793074.412 r/s (11379300 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 2237517.294 r/s (6712600 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 2075827.991 r/s (6227600 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 3706439.274 r/s (11119400 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2041834.905 r/s (6125600 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 5335212.653 r/s (16006000 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2359074.629 r/s (7077600 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 7323352.432 r/s (21970400 rounds in 3.000 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 6379220.973 r/s (19137700 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 1340971.319 r/s (4023000 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 15104987.453 r/s (45315000 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 4957164.708 r/s (14871600 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10222316.753 r/s (30667200 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 8779494.830 r/s (26338800 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10185577.225 r/s (30556800 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 12655562.272 r/s (37967200 rounds in 3.000 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 3232930.225 r/s (9698800 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 1556422.533 r/s (4669300 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1154646.368 r/s (3464000 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 1253750.361 r/s (3761400 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1100273.434 r/s (3301200 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 1833710.748 r/s (5501200 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1159641.938 r/s (3479200 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 2052999.421 r/s (6159200 rounds in 3.000 seconds)

```
