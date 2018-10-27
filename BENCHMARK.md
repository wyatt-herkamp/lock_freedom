# Benchmarks
Benchmark code under [benchmark](benchmark) directory.
More rounds per seconds is better.

As you can see, there is a lot to improve!

Running on Linux 4.18.12-arch1-1-ARCH #1 SMP PREEMPT Thu Oct 4 01:01:27 UTC 2018 x86_64 GNU/Linux (4 cores)

## THREAD-LOCAL STORAGE
```
Result for 1 threads:
Target 0 (std/global):
mean of 21274861.887 r/s (63824600 rounds in 3.000 seconds)
Target 1 (blocking):
mean of 17246838.664 r/s (51740600 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 17542621.814 r/s (52627900 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 46001520.203 r/s (138004800 rounds in 3.000 seconds)
Target 1 (blocking):
mean of 39021952.874 r/s (117066000 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 40929147.652 r/s (122787600 rounds in 3.000 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 46477978.987 r/s (139435200 rounds in 3.000 seconds)
Target 1 (blocking):
mean of 36686959.139 r/s (110062400 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 41888286.346 r/s (125665600 rounds in 3.000 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 40110274.549 r/s (120332800 rounds in 3.000 seconds)
Target 1 (blocking):
mean of 32109707.542 r/s (96329600 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 37734555.704 r/s (113206400 rounds in 3.000 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 41678953.502 r/s (125043200 rounds in 3.000 seconds)
Target 1 (blocking):
mean of 22856336.441 r/s (68569600 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 39749718.179 r/s (119257600 rounds in 3.000 seconds)

Result for 512 threads:
Target 0 (std/global):
mean of 32699278.421 r/s (98099200 rounds in 3.000 seconds)
Target 1 (blocking):
mean of 9275956.368 r/s (27852800 rounds in 3.003 seconds)
Target 2 (lockfree):
mean of 31926166.623 r/s (95795200 rounds in 3.001 seconds)

## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 13871824.700 r/s (41615500 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 11314179.872 r/s (33942600 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 5244346.359 r/s (15733100 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 17013024.401 r/s (51039200 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 4409371.191 r/s (13228200 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 4823935.266 r/s (14472000 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 9973961.903 r/s (29922000 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 5030561.499 r/s (15092000 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 5203756.068 r/s (15611600 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8601655.803 r/s (25805600 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 4452526.625 r/s (13357600 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 5947961.790 r/s (17844000 rounds in 3.000 seconds)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 14010945.982 r/s (42032900 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 10866182.915 r/s (32598600 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 9542194.714 r/s (28626600 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 15932856.438 r/s (47798600 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 3576789.729 r/s (10730400 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 8743491.773 r/s (26230600 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 9566350.963 r/s (28699200 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 4930410.827 r/s (14791600 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 7017178.403 r/s (21051600 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8260529.503 r/s (24781600 rounds in 3.000 seconds)
Target 1 (mutex linked list):
mean of 4427074.159 r/s (13281600 rounds in 3.000 seconds)
Target 2 (lockfree):
mean of 7462350.925 r/s (22387200 rounds in 3.000 seconds)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 2996691.736 r/s (8990100 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 1584833.146 r/s (4754600 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 1966713.577 r/s (5900200 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 2162185.849 r/s (6486600 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 1868578.889 r/s (5606000 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 2975092.721 r/s (8925600 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2145301.462 r/s (6436000 rounds in 3.000 seconds)
Target 1 (lockfree insert):
mean of 3807941.009 r/s (11424800 rounds in 3.000 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 3812205.527 r/s (11436700 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 2342186.228 r/s (7026600 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 2083582.223 r/s (6250800 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 3709196.453 r/s (11127800 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2097291.232 r/s (6292000 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 5417287.323 r/s (16252000 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2422343.597 r/s (7267200 rounds in 3.000 seconds)
Target 1 (lockfree get):
mean of 7502617.163 r/s (22508800 rounds in 3.000 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 6493727.816 r/s (19481200 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 1221287.225 r/s (3663900 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 16179575.984 r/s (48538800 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 4312716.320 r/s (12938200 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10393654.691 r/s (31181200 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 8953826.730 r/s (26861600 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10372254.013 r/s (31116800 rounds in 3.000 seconds)
Target 1 (lockfree remove):
mean of 12673911.193 r/s (38022400 rounds in 3.000 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 3280755.939 r/s (9842400 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 1516924.686 r/s (4550800 rounds in 3.000 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1171755.302 r/s (3515400 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 1204046.530 r/s (3612200 rounds in 3.000 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1096619.636 r/s (3290000 rounds in 3.000 seconds)
Target 1 (lockfree mixed):
mean of 1797785.341 r/s (5393600 rounds in 3.000 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1163269.594 r/s (3490400 rounds in 3.001 seconds)
Target 1 (lockfree mixed):
mean of 2015681.095 r/s (6047200 rounds in 3.000 seconds)

```
