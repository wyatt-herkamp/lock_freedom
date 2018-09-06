# Benchmarks
Benchmark code under [benchmark][] directory.
More rounds per seconds is better.

As you can see, there is a lot to improve!

## QUEUE
```
Result for 1 threads:
Target 0 (mutex):
mean of 7375457.276 r/s (18439000 rounds in 2.500)
Target 1 (lockfree):
mean of 36385114.131 r/s (90963000 rounds in 2.500)

Result for 4 threads:
Target 0 (mutex):
mean of 5646635.084 r/s (14120000 rounds in 2.501)
Target 1 (lockfree):
mean of 8440920.918 r/s (21104000 rounds in 2.500)

Result for 8 threads:
Target 0 (mutex):
mean of 6718133.238 r/s (16800000 rounds in 2.501)
Target 1 (lockfree):
mean of 7506785.761 r/s (18768000 rounds in 2.500)

Result for 16 threads:
Target 0 (mutex):
mean of 6408840.231 r/s (16032000 rounds in 2.502)
Target 1 (lockfree):
mean of 7375095.951 r/s (18448000 rounds in 2.501)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex):
mean of 15646454.651 r/s (39117000 rounds in 2.500)
Target 1 (lockfree):
mean of 34532999.115 r/s (86333000 rounds in 2.500)

Result for 4 threads:
Target 0 (mutex):
mean of 7808770.612 r/s (19524000 rounds in 2.500)
Target 1 (lockfree):
mean of 8190154.449 r/s (20476000 rounds in 2.500)

Result for 16 threads:
Target 0 (mutex):
mean of 7512253.807 r/s (18784000 rounds in 2.500)
Target 1 (lockfree):
mean of 7410657.175 r/s (18528000 rounds in 2.500)

Result for 32 threads:
Target 0 (mutex):
mean of 7149143.614 r/s (17888000 rounds in 2.502)
Target 1 (lockfree):
mean of 7449315.251 r/s (18624000 rounds in 2.500)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex):
mean of 473696.793 r/s (1185000 rounds in 2.502)
Target 1 (lockfree):
mean of 1034065.163 r/s (2586000 rounds in 2.501)

Result for 4 threads:
Target 0 (mutex):
mean of 1565199.629 r/s (3916000 rounds in 2.502)
Target 1 (lockfree):
mean of 506707.984 r/s (1268000 rounds in 2.502)

Result for 8 threads:
Target 0 (mutex):
mean of 1943266.363 r/s (4864000 rounds in 2.503)
Target 1 (lockfree):
mean of 501934.210 r/s (1256000 rounds in 2.502)

Result for 16 threads:
Target 0 (mutex):
mean of 2085303.386 r/s (5216000 rounds in 2.501)
Target 1 (lockfree):
mean of 497114.979 r/s (1248000 rounds in 2.510)

```
