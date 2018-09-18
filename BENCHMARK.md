# Benchmarks
Benchmark code under [benchmark](benchmark) directory.
More rounds per seconds is better.

As you can see, there is a lot to improve (especially for map)!

Running on Linux 4.18.6-arch1-1-ARCH #1 SMP PREEMPT Wed Sep 5 11:54:09 UTC 2018 x86_64 GNU/Linux (4 cores)

## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 13787348.619 r/s (41362100 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 11090939.150 r/s (33272900 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 5692020.035 r/s (17076100 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 15258958.099 r/s (45777000 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 4156422.018 r/s (12469400 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 5866923.090 r/s (17600800 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 9504391.316 r/s (28513200 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 4896558.862 r/s (14690000 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 5264233.911 r/s (15792800 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8256580.890 r/s (24770400 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 4355036.197 r/s (13065600 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 5972426.230 r/s (17918400 rounds in 3.000 seconds)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 13568914.970 r/s (40706800 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 10934615.583 r/s (32803900 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 9336203.358 r/s (28008700 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 14881355.505 r/s (44644200 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 4022445.589 r/s (12067400 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 10009390.688 r/s (30028200 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 9386304.848 r/s (28159200 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 4959712.744 r/s (14879600 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 6779769.727 r/s (20339600 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8233261.830 r/s (24700000 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 4435823.843 r/s (13308000 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 7303875.481 r/s (21912000 rounds in 3.000 seconds)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 2975505.421 r/s (8926600 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 1605161.237 r/s (4815500 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 1988421.610 r/s (5965400 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 2180879.934 r/s (6542800 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 1847543.384 r/s (5542800 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 2959721.436 r/s (8879600 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2107938.445 r/s (6324000 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 3802644.836 r/s (11408000 rounds in 3.000 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 3774739.192 r/s (11324300 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 2281703.972 r/s (6845200 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 2088240.139 r/s (6264800 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 3785664.302 r/s (11357200 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2060531.502 r/s (6181600 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 5341997.347 r/s (16026400 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2379208.710 r/s (7138400 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 7386359.963 r/s (22160000 rounds in 3.000 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 6441026.635 r/s (19323100 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 1344261.152 r/s (4032800 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 15028292.822 r/s (45085000 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 4989619.302 r/s (14969000 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10274776.765 r/s (30824400 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 8818358.727 r/s (26455200 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10305710.941 r/s (30917600 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 12791161.908 r/s (38373600 rounds in 3.000 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 3239921.424 r/s (9719900 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 1564541.369 r/s (4693700 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1154524.794 r/s (3463600 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 1265414.230 r/s (3796400 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1101146.982 r/s (3303600 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 1857102.966 r/s (5571600 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1158170.436 r/s (3475200 rounds in 3.001 seconds)
Target 1 (lockfree mixed):
mean of 2092426.150 r/s (6277600 rounds in 3.000 seconds)

```
