# Benchmarks
Benchmark code under (benchmark)[benchmark] directory.
More rounds per seconds is better.

As you can see, there is a lot to improve!

## QUEUE
```
Result for 1 threads:
Target 0 (mutex):
mean of 7474537.889 r/s (18687000 rounds in 2.500)
Target 1 (lockfree):
mean of 36479845.209 r/s (91200000 rounds in 2.500)

Result for 4 threads:
Target 0 (mutex):
mean of 5814553.779 r/s (14540000 rounds in 2.501)
Target 1 (lockfree):
mean of 8450516.563 r/s (21128000 rounds in 2.500)

Result for 8 threads:
Target 0 (mutex):
mean of 6786552.025 r/s (16968000 rounds in 2.500)
Target 1 (lockfree):
mean of 7538779.764 r/s (18848000 rounds in 2.500)

Result for 16 threads:
Target 0 (mutex):
mean of 6610081.363 r/s (16544000 rounds in 2.503)
Target 1 (lockfree):
mean of 7487334.310 r/s (18720000 rounds in 2.500)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex):
mean of 15089740.396 r/s (37725000 rounds in 2.500)
Target 1 (lockfree):
mean of 34779235.861 r/s (86949000 rounds in 2.500)

Result for 4 threads:
Target 0 (mutex):
mean of 7555138.426 r/s (18892000 rounds in 2.501)
Target 1 (lockfree):
mean of 8478514.669 r/s (21200000 rounds in 2.500)

Result for 16 threads:
Target 0 (mutex):
mean of 7417360.241 r/s (18544000 rounds in 2.500)
Target 1 (lockfree):
mean of 7424148.650 r/s (18576000 rounds in 2.502)

Result for 32 threads:
Target 0 (mutex):
mean of 6742125.694 r/s (16864000 rounds in 2.501)
Target 1 (lockfree):
mean of 7461126.287 r/s (18656000 rounds in 2.500)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex):
mean of 470757.018 r/s (1177000 rounds in 2.500)
Target 1 (lockfree):
mean of 1031283.321 r/s (2579000 rounds in 2.501)

Result for 4 threads:
Target 0 (mutex):
mean of 1550263.182 r/s (3876000 rounds in 2.500)
Target 1 (lockfree):
mean of 506923.403 r/s (1268000 rounds in 2.501)

Result for 8 threads:
Target 0 (mutex):
mean of 1915913.423 r/s (4792000 rounds in 2.501)
Target 1 (lockfree):
mean of 503260.029 r/s (1264000 rounds in 2.512)

Result for 16 threads:
Target 0 (mutex):
mean of 2070594.395 r/s (5184000 rounds in 2.504)
Target 1 (lockfree):
mean of 498483.446 r/s (1248000 rounds in 2.504)

```
