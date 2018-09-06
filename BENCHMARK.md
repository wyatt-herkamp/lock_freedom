# Benchmarks
Benchmark code under [../../tree/master/benchmark] directory.
More rounds per seconds is better.

As you can see, there is a lot to improve!

## QUEUE
```
Result for 1 threads:
Target 0 (mutex):
mean of 7397475.015 r/s (18494000 rounds in 2.500)
Target 1 (lockfree):
mean of 36421572.363 r/s (91054000 rounds in 2.500)

Result for 4 threads:
Target 0 (mutex):
mean of 5726939.348 r/s (14320000 rounds in 2.500)
Target 1 (lockfree):
mean of 9361119.471 r/s (23404000 rounds in 2.500)

Result for 8 threads:
Target 0 (mutex):
mean of 6711032.431 r/s (16784000 rounds in 2.501)
Target 1 (lockfree):
mean of 8519120.561 r/s (21304000 rounds in 2.501)

Result for 16 threads:
Target 0 (mutex):
mean of 6437129.769 r/s (16096000 rounds in 2.500)
Target 1 (lockfree):
mean of 8305546.396 r/s (20768000 rounds in 2.500)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex):
mean of 15694241.720 r/s (39236000 rounds in 2.500)
Target 1 (lockfree):
mean of 34770585.827 r/s (86927000 rounds in 2.500)

Result for 4 threads:
Target 0 (mutex):
mean of 7240610.834 r/s (18104000 rounds in 2.500)
Target 1 (lockfree):
mean of 8238358.960 r/s (20596000 rounds in 2.500)

Result for 16 threads:
Target 0 (mutex):
mean of 7492029.063 r/s (18736000 rounds in 2.501)
Target 1 (lockfree):
mean of 7404181.623 r/s (18512000 rounds in 2.500)

Result for 32 threads:
Target 0 (mutex):
mean of 6809211.524 r/s (17056000 rounds in 2.505)
Target 1 (lockfree):
mean of 7485985.414 r/s (18720000 rounds in 2.501)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex):
mean of 472523.593 r/s (1182000 rounds in 2.501)
Target 1 (lockfree):
mean of 1045688.431 r/s (2615000 rounds in 2.501)

Result for 4 threads:
Target 0 (mutex):
mean of 1575237.380 r/s (3940000 rounds in 2.501)
Target 1 (lockfree):
mean of 506041.066 r/s (1268000 rounds in 2.506)

Result for 8 threads:
Target 0 (mutex):
mean of 1974099.443 r/s (4944000 rounds in 2.504)
Target 1 (lockfree):
mean of 500271.786 r/s (1256000 rounds in 2.511)

Result for 16 threads:
Target 0 (mutex):
mean of 2136597.177 r/s (5344000 rounds in 2.501)
Target 1 (lockfree):
mean of 497136.160 r/s (1248000 rounds in 2.510)

```
