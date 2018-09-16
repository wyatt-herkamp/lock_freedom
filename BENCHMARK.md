# Benchmarks
Benchmark code under [benchmark](benchmark) directory.
More rounds per seconds is better.

As you can see, there is a lot to improve (especially for map)!

Running on Linux 4.18.6-arch1-1-ARCH #1 SMP PREEMPT Wed Sep 5 11:54:09 UTC 2018 x86_64 GNU/Linux

## QUEUE
```
Result for 1 threads:
Target 0 (mutex):
mean of 5735115.532 r/s (17205400 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 14024945.746 r/s (42074900 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 5292347.032 r/s (15877200 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 9803442.099 r/s (29410800 rounds in 3.000 seconds)

Result for 16 threads:
Target 0 (mutex):
mean of 5960137.864 r/s (17881600 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 7750288.786 r/s (23251200 rounds in 3.000 seconds)

Result for 32 threads:
Target 0 (mutex):
mean of 5699145.058 r/s (17097600 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 7522942.633 r/s (22569600 rounds in 3.000 seconds)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex):
mean of 9306075.392 r/s (27918300 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 13761949.960 r/s (41285900 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 6900214.418 r/s (20700800 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 9460133.173 r/s (28380400 rounds in 3.000 seconds)

Result for 16 threads:
Target 0 (mutex):
mean of 6946630.406 r/s (20841600 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 7582748.683 r/s (22748800 rounds in 3.000 seconds)

Result for 32 threads:
Target 0 (mutex):
mean of 6440093.342 r/s (19321600 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 7401487.295 r/s (22204800 rounds in 3.000 seconds)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 1575631.583 r/s (4726900 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 2986160.354 r/s (8958700 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2967647.537 r/s (8903200 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 1839995.762 r/s (5520000 rounds in 3.000 seconds)

Result for 16 threads:
Target 0 (mutex insert):
mean of 4172992.965 r/s (12520000 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 2207221.514 r/s (6622400 rounds in 3.000 seconds)

Result for 32 threads:
Target 0 (mutex insert):
mean of 4031767.114 r/s (12096000 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 2243256.808 r/s (6732800 rounds in 3.001 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 2346748.120 r/s (7040300 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 3779718.683 r/s (11339200 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 5266606.754 r/s (15800400 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 2047157.890 r/s (6141600 rounds in 3.000 seconds)

Result for 16 threads:
Target 0 (mutex get):
mean of 7656404.928 r/s (22969600 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 2494782.504 r/s (7484800 rounds in 3.000 seconds)

Result for 32 threads:
Target 0 (mutex get):
mean of 7369066.286 r/s (22108800 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 2547248.441 r/s (7644800 rounds in 3.001 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 1366237.783 r/s (4098800 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 6379090.421 r/s (19137300 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 7057943.901 r/s (21174000 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 10278923.914 r/s (30836800 rounds in 3.000 seconds)

Result for 16 threads:
Target 0 (mutex remove):
mean of 16170050.764 r/s (48510400 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 9446039.948 r/s (28339200 rounds in 3.000 seconds)

Result for 32 threads:
Target 0 (mutex remove):
mean of 15992021.706 r/s (47977600 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 8683154.847 r/s (26051200 rounds in 3.000 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 1490607.008 r/s (4471900 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 3238799.796 r/s (9716400 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1683587.927 r/s (5050800 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 1102236.600 r/s (3306800 rounds in 3.000 seconds)

Result for 16 threads:
Target 0 (mutex mixed):
mean of 2022179.316 r/s (6067200 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 1053175.791 r/s (3160000 rounds in 3.000 seconds)

Result for 32 threads:
Target 0 (mutex mixed):
mean of 1891813.077 r/s (5676800 rounds in 3.001 seconds)
Target 1 (lockfree mixed):
mean of 1152882.706 r/s (3459200 rounds in 3.000 seconds)

```
