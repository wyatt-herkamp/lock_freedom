# Benchmarks
Benchmark code under [benchmark](benchmark) directory.
More rounds per seconds is better.

As you can see, there is a lot to improve!

## QUEUE
```
Result for 1 threads:
Target 0 (mutex):
mean of 5775159.944 r/s (17325500 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 13847305.376 r/s (41542000 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 5298489.909 r/s (15895600 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 9828195.484 r/s (29484800 rounds in 3.000 seconds)

Result for 16 threads:
Target 0 (mutex):
mean of 6002063.751 r/s (18006400 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 7833954.816 r/s (23502400 rounds in 3.000 seconds)

Result for 32 threads:
Target 0 (mutex):
mean of 5692495.118 r/s (17078400 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 7632930.047 r/s (22899200 rounds in 3.000 seconds)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex):
mean of 9368962.647 r/s (28106900 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 13764230.177 r/s (41292700 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 6879483.971 r/s (20638800 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 9582333.326 r/s (28747200 rounds in 3.000 seconds)

Result for 16 threads:
Target 0 (mutex):
mean of 7103974.279 r/s (21312000 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 7706506.790 r/s (23120000 rounds in 3.000 seconds)

Result for 32 threads:
Target 0 (mutex):
mean of 6452221.583 r/s (19360000 rounds in 3.001 seconds)
Target 1 (lockfree):
mean of 7494302.764 r/s (22483200 rounds in 3.000 seconds)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex):
mean of 460172.343 r/s (1380600 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 793177.355 r/s (2383200 rounds in 3.005 seconds)

Result for 4 threads:
Target 0 (mutex):
mean of 1495029.555 r/s (4485200 rounds in 3.000 seconds)
Target 1 (lockfree):
mean of 477760.725 r/s (1433600 rounds in 3.001 seconds)

Result for 16 threads:
Target 0 (mutex):
mean of 2128644.911 r/s (6387200 rounds in 3.001 seconds)
Target 1 (lockfree):
mean of 486337.969 r/s (1459200 rounds in 3.000 seconds)

Result for 32 threads:
Target 0 (mutex):
mean of 2004618.390 r/s (6016000 rounds in 3.001 seconds)
Target 1 (lockfree):
mean of 484248.492 r/s (1452800 rounds in 3.000 seconds)

```
