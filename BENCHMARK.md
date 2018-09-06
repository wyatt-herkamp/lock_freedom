# Benchmarks
Benchmark code under [benchmark](benchmark) directory.
More rounds per seconds is better.

As you can see, there is a lot to improve!

## QUEUE
```
Result for 1 threads:
Target 0 (mutex):
mean of 7624604.316 r/s (19062000 rounds in 2.500 seconds)
Target 1 (lockfree):
mean of 36472387.526 r/s (91181000 rounds in 2.500 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 5763403.493 r/s (14412000 rounds in 2.501 seconds)
Target 1 (lockfree):
mean of 8459811.620 r/s (21152000 rounds in 2.500 seconds)

Result for 8 threads:
Target 0 (mutex):
mean of 6751943.910 r/s (16880000 rounds in 2.500 seconds)
Target 1 (lockfree):
mean of 7564411.737 r/s (18912000 rounds in 2.500 seconds)

Result for 16 threads:
Target 0 (mutex):
mean of 6502375.528 r/s (16272000 rounds in 2.502 seconds)
Target 1 (lockfree):
mean of 7360589.454 r/s (18416000 rounds in 2.502 seconds)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex):
mean of 15743256.652 r/s (39359000 rounds in 2.500 seconds)
Target 1 (lockfree):
mean of 34529410.710 r/s (86324000 rounds in 2.500 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 7678178.448 r/s (19196000 rounds in 2.500 seconds)
Target 1 (lockfree):
mean of 8381340.485 r/s (20956000 rounds in 2.500 seconds)

Result for 16 threads:
Target 0 (mutex):
mean of 7579776.994 r/s (18960000 rounds in 2.501 seconds)
Target 1 (lockfree):
mean of 7556087.780 r/s (18896000 rounds in 2.501 seconds)

Result for 32 threads:
Target 0 (mutex):
mean of 7003149.958 r/s (17536000 rounds in 2.504 seconds)
Target 1 (lockfree):
mean of 7524944.751 r/s (18816000 rounds in 2.500 seconds)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex):
mean of 478129.372 r/s (1196000 rounds in 2.501 seconds)
Target 1 (lockfree):
mean of 1050698.806 r/s (2627000 rounds in 2.500 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 1577654.338 r/s (3948000 rounds in 2.502 seconds)
Target 1 (lockfree):
mean of 511743.823 r/s (1280000 rounds in 2.501 seconds)

Result for 8 threads:
Target 0 (mutex):
mean of 2010070.307 r/s (5032000 rounds in 2.503 seconds)
Target 1 (lockfree):
mean of 508972.729 r/s (1280000 rounds in 2.515 seconds)

Result for 16 threads:
Target 0 (mutex):
mean of 2204377.075 r/s (5520000 rounds in 2.504 seconds)
Target 1 (lockfree):
mean of 505250.036 r/s (1264000 rounds in 2.502 seconds)

```
