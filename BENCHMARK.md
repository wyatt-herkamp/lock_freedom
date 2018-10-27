# Benchmarks
Benchmark code under [benchmark](benchmark) directory.
More rounds per seconds is better.

As you can see, there is a lot to improve!

Running on Linux 4.18.12-arch1-1-ARCH #1 SMP PREEMPT Thu Oct 4 01:01:27 UTC 2018 x86_64 GNU/Linux (4 cores)

## THREAD-LOCAL STORAGE
```
Result for 1 threads:
Target 0 (lock):
mean of 17401520.381 r/s (52204600 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 17690954.879 r/s (53072900 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (lock):
mean of 39217302.574 r/s (117652000 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 40864101.800 r/s (122592400 rounds in 3.000 seconds)

Result for 64 threads:
Target 0 (lock):
mean of 23274088.321 r/s (69824000 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 39445736.629 r/s (118342400 rounds in 3.000 seconds)

Result for 256 threads:
Target 0 (lock):
mean of 9418787.456 r/s (28262400 rounds in 3.001 seconds)
Target 1 (lockfree):
mean of 36417793.012 r/s (109260800 rounds in 3.000 seconds)

## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 13733943.282 r/s (41201900 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 11258244.004 r/s (33774800 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 5286052.267 r/s (15858200 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 15011034.643 r/s (45033200 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 4751597.382 r/s (14254800 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 4500758.441 r/s (13502400 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 9817400.386 r/s (29452400 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 5069866.735 r/s (15210000 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 5166765.903 r/s (15500400 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8472368.108 r/s (25417600 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 4457906.777 r/s (13374400 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 5876532.138 r/s (17629600 rounds in 3.000 seconds)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 13972173.010 r/s (41916600 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 10857464.564 r/s (32572400 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 9585820.996 r/s (28757500 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 15467589.358 r/s (46402800 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 3576910.793 r/s (10730800 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 9029065.216 r/s (27087200 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 9665396.067 r/s (28996400 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 4977454.898 r/s (14932400 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 6948430.307 r/s (20845600 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8591977.993 r/s (25776800 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 4431217.068 r/s (13294400 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 7466975.172 r/s (22401600 rounds in 3.000 seconds)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 2994683.422 r/s (8984200 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 1589264.008 r/s (4767900 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 1970658.847 r/s (5912000 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 2168476.714 r/s (6505600 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 1871515.621 r/s (5614800 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 2984478.246 r/s (8953600 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2143537.358 r/s (6431200 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 3824907.438 r/s (11475200 rounds in 3.000 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 3814855.988 r/s (11444600 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 2312656.408 r/s (6938000 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 2079342.015 r/s (6238200 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 3794802.280 r/s (11384600 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2098520.434 r/s (6296000 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 5439723.247 r/s (16319600 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2415160.951 r/s (7245600 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 7527082.379 r/s (22581600 rounds in 3.000 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 6556622.417 r/s (19669900 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 1243576.114 r/s (3730800 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 15886180.905 r/s (47658600 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 4435255.114 r/s (13305800 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10348756.314 r/s (31046400 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 8956729.917 r/s (26870400 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10292366.401 r/s (30877600 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 12930908.680 r/s (38793600 rounds in 3.000 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 3278957.437 r/s (9837000 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 1528956.815 r/s (4586900 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1191107.851 r/s (3573400 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 1213388.721 r/s (3640200 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1099126.004 r/s (3297600 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 1813241.187 r/s (5440000 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1160225.710 r/s (3480800 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 2026456.328 r/s (6080000 rounds in 3.000 seconds)

```
