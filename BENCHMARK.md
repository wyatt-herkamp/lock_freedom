# Benchmarks
Benchmark code under [benchmark] directory.
More rounds per seconds is better.

As you can see, there is a lot to improve!

## QUEUE
Result for 1 threads:
Target 0 (mutex):
mean of 7365917.655 r/s (18415000 rounds in 2.500)
Target 1 (lockfree):
mean of 36244852.368 r/s (90613000 rounds in 2.500)

Result for 4 threads:
Target 0 (mutex):
mean of 5716103.772 r/s (14292000 rounds in 2.500)
Target 1 (lockfree):
mean of 8338002.366 r/s (20848000 rounds in 2.500)

Result for 8 threads:
Target 0 (mutex):
mean of 6687103.685 r/s (16720000 rounds in 2.500)
Target 1 (lockfree):
mean of 7511844.719 r/s (18784000 rounds in 2.501)

Result for 16 threads:
Target 0 (mutex):
mean of 6448743.555 r/s (16128000 rounds in 2.501)
Target 1 (lockfree):
mean of 7382206.019 r/s (18464000 rounds in 2.501)

## STACK
Result for 1 threads:
Target 0 (mutex):
mean of 15639096.275 r/s (39098000 rounds in 2.500)
Target 1 (lockfree):
mean of 34810525.763 r/s (87027000 rounds in 2.500)

Result for 4 threads:
Target 0 (mutex):
mean of 7837993.672 r/s (19596000 rounds in 2.500)
Target 1 (lockfree):
mean of 8160738.918 r/s (20404000 rounds in 2.500)

Result for 16 threads:
Target 0 (mutex):
mean of 7595285.846 r/s (18992000 rounds in 2.500)
Target 1 (lockfree):
mean of 7381125.585 r/s (18464000 rounds in 2.502)

Result for 32 threads:
Target 0 (mutex):
mean of 6960878.258 r/s (17408000 rounds in 2.501)
Target 1 (lockfree):
mean of 7391481.461 r/s (18496000 rounds in 2.502)

## MAP
Result for 1 threads:
Target 0 (mutex):
mean of 469831.232 r/s (1175000 rounds in 2.501)
Target 1 (lockfree):
mean of 1061920.415 r/s (2655000 rounds in 2.500)

Result for 4 threads:
Target 0 (mutex):
mean of 1567608.092 r/s (3920000 rounds in 2.501)
Target 1 (lockfree):
mean of 511163.574 r/s (1280000 rounds in 2.504)

Result for 8 threads:
Target 0 (mutex):
mean of 1951228.168 r/s (4888000 rounds in 2.505)
Target 1 (lockfree):
mean of 508937.972 r/s (1280000 rounds in 2.515)

Result for 16 threads:
Target 0 (mutex):
mean of 2127282.816 r/s (5328000 rounds in 2.505)
Target 1 (lockfree):
mean of 503105.724 r/s (1264000 rounds in 2.512)
