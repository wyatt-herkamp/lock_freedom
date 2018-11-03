# CPU info
```
Architecture:        x86_64
CPU op-mode(s):      32-bit, 64-bit
Byte Order:          Little Endian
CPU(s):              4
On-line CPU(s) list: 0-3
Thread(s) per core:  2
Core(s) per socket:  2
Socket(s):           1
NUMA node(s):        1
Vendor ID:           GenuineIntel
CPU family:          6
Model:               142
Model name:          Intel(R) Core(TM) i5-7200U CPU @ 2.50GHz
Stepping:            9
CPU MHz:             2185.301
CPU max MHz:         3100.0000
CPU min MHz:         400.0000
BogoMIPS:            5426.00
Virtualization:      VT-x
L1d cache:           32K
L1i cache:           32K
L2 cache:            256K
L3 cache:            3072K
NUMA node0 CPU(s):   0-3
Flags:               fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx pdpe1gb rdtscp lm constant_tsc art arch_perfmon pebs bts rep_good nopl xtopology nonstop_tsc cpuid aperfmperf tsc_known_freq pni pclmulqdq dtes64 monitor ds_cpl vmx est tm2 ssse3 sdbg fma cx16 xtpr pdcm pcid sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave avx f16c rdrand lahf_lm abm 3dnowprefetch cpuid_fault epb invpcid_single pti ssbd ibrs ibpb stibp tpr_shadow vnmi flexpriority ept vpid fsgsbase tsc_adjust bmi1 avx2 smep bmi2 erms invpcid mpx rdseed adx smap clflushopt intel_pt xsaveopt xsavec xgetbv1 xsaves dtherm ida arat pln pts hwp hwp_notify hwp_act_window hwp_epp flush_l1d
```
Running on Linux 4.18.16-arch1-1-ARCH #1 SMP PREEMPT Sat Oct 20 22:06:45 UTC 2018 x86_64 GNU/Linux

# Benchmarks
Benchmark code under [benchmark](benchmark) directory.
More rounds per seconds is better.

As you can see, there is a lot to improve!


## THREAD-LOCAL STORAGE
```
Result for 1 threads:
Target 0 (std/global):
mean of 19798405.740 r/s (29697700 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 17200921.843 r/s (25801400 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 17559277.858 r/s (26339000 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 46221236.577 r/s (69332000 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 38705549.049 r/s (58058800 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 40745749.319 r/s (61118800 rounds in 1.500 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 46708563.514 r/s (70064000 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 33704037.412 r/s (50556800 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 41740343.667 r/s (62611200 rounds in 1.500 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 40054252.329 r/s (60083200 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 29336849.837 r/s (44006400 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 37381879.395 r/s (56073600 rounds in 1.500 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 41725860.020 r/s (62592000 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 15492158.978 r/s (23244800 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 39706805.654 r/s (59571200 rounds in 1.500 seconds)

Result for 512 threads:
Target 0 (std/global):
mean of 33273517.964 r/s (49920000 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 5368051.468 r/s (8089600 rounds in 1.507 seconds)
Target 2 (lockfree):
mean of 32075698.350 r/s (48128000 rounds in 1.500 seconds)

```
## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 13762828.874 r/s (20644300 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 11204234.511 r/s (16806400 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 6183697.843 r/s (9275600 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 17528690.387 r/s (26293200 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 5090024.549 r/s (7635200 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 4208103.007 r/s (6312200 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 9908519.911 r/s (14862800 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 5125599.597 r/s (7688400 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 4038657.935 r/s (6058000 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8685688.876 r/s (13028800 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 4467668.880 r/s (6701600 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 4497811.845 r/s (6747200 rounds in 1.500 seconds)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 14057291.489 r/s (21086000 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 10815431.257 r/s (16223200 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 9749813.589 r/s (14624800 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 16301622.026 r/s (24452600 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 3905533.163 r/s (5858400 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 8453172.481 r/s (12680000 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 9723872.073 r/s (14586000 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 5074817.567 r/s (7612400 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 5514523.977 r/s (8272000 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8451179.424 r/s (12676800 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 4524546.499 r/s (6787200 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 6025253.046 r/s (9038400 rounds in 1.500 seconds)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 2783768.699 r/s (4520600 rounds in 1.624 seconds)
Target 1 (lockfree insert):
mean of 2119653.937 r/s (3179500 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 2087381.451 r/s (3131200 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 2218217.947 r/s (3327400 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2284379.732 r/s (3426800 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 2739886.857 r/s (4110000 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2468515.828 r/s (3703200 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 3249454.650 r/s (4874400 rounds in 1.500 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 4718325.274 r/s (7077500 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 3250585.156 r/s (4875900 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 2322394.603 r/s (3483800 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 3601367.079 r/s (5402200 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2590894.180 r/s (3886400 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 4824320.433 r/s (7236800 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2847375.072 r/s (4271200 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 5439917.647 r/s (8160000 rounds in 1.500 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 7874851.563 r/s (11812300 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 1614481.640 r/s (2421800 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 15730351.100 r/s (23595600 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 3789996.199 r/s (5685200 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10283215.530 r/s (15425200 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 6741843.416 r/s (10113200 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10420748.584 r/s (15631200 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 8538988.717 r/s (12808800 rounds in 1.500 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 3575327.963 r/s (5363000 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 1508073.608 r/s (2262200 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1750634.345 r/s (2626000 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 935473.950 r/s (1403400 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1406025.096 r/s (2109200 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 1137285.333 r/s (1706000 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1491527.615 r/s (2237600 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 1305461.670 r/s (1958400 rounds in 1.500 seconds)

```
## REQUEST PROGRAM
```
A program simulating a concurrent server.

Mutexed structures with 2 threads total time: 420.769704ms
Lockfree structures with 2 threads total time: 373.580732ms

Mutexed structures with 4 threads total time: 419.241569ms
Lockfree structures with 4 threads total time: 446.663156ms

Mutexed structures with 8 threads total time: 518.194191ms
Lockfree structures with 8 threads total time: 716.145307ms

Mutexed structures with 16 threads total time: 1.595897188s
Lockfree structures with 16 threads total time: 1.440252299s
```
