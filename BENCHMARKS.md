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
CPU MHz:             2234.211
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
mean of 25822098.801 r/s (38733200 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 17293017.981 r/s (25939600 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 17357703.443 r/s (26036600 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 46313318.575 r/s (69470000 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 39141103.184 r/s (58712000 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 41127117.664 r/s (61690800 rounds in 1.500 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 46537411.741 r/s (69806400 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 33802025.217 r/s (50704000 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 41639760.472 r/s (62460800 rounds in 1.500 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 40099026.858 r/s (60150400 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 28682352.232 r/s (43024000 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 37480339.935 r/s (56220800 rounds in 1.500 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 42007643.116 r/s (63014400 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 15274261.196 r/s (22912000 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 39866363.499 r/s (59801600 rounds in 1.500 seconds)

Result for 512 threads:
Target 0 (std/global):
mean of 33096377.447 r/s (49664000 rounds in 1.501 seconds)
Target 1 (blocking):
mean of 5349499.391 r/s (8038400 rounds in 1.503 seconds)
Target 2 (lockfree):
mean of 31873318.218 r/s (47872000 rounds in 1.502 seconds)

```
## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 13821417.204 r/s (20732200 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 11308385.872 r/s (16962600 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 6307011.152 r/s (9460600 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 17054890.876 r/s (25582600 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 4777829.170 r/s (7166800 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 4069557.527 r/s (6104400 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 9911971.791 r/s (14868000 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 5087090.953 r/s (7630800 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 3990083.157 r/s (5985200 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8539396.955 r/s (12809600 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 4530917.014 r/s (6796800 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 4495506.274 r/s (6744000 rounds in 1.500 seconds)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 14016171.697 r/s (21024300 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 10973534.217 r/s (16460400 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 9775536.511 r/s (14663400 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 16136772.729 r/s (24205400 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 5080509.875 r/s (7620800 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 6839008.111 r/s (10258600 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 9684974.524 r/s (14527600 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 5100974.842 r/s (7651600 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 5014713.064 r/s (7522400 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8465257.236 r/s (12698400 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 4547175.894 r/s (6820800 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 5359315.237 r/s (8040000 rounds in 1.500 seconds)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 2781591.748 r/s (4520600 rounds in 1.625 seconds)
Target 1 (lockfree insert):
mean of 2125709.280 r/s (3188600 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 2064053.717 r/s (3096200 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 2206829.687 r/s (3310400 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2295885.559 r/s (3444000 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 2748533.645 r/s (4123200 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2491651.326 r/s (3737600 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 3227581.131 r/s (4841600 rounds in 1.500 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 4671721.850 r/s (7007600 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 3176981.832 r/s (4765500 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 2301176.620 r/s (3452000 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 3674938.353 r/s (5512600 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2601272.115 r/s (3902000 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 5014321.452 r/s (7521600 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2866701.989 r/s (4300800 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 5665540.238 r/s (8499200 rounds in 1.500 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 7891594.392 r/s (11837400 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 1605973.887 r/s (2409000 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 15906394.549 r/s (23859600 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 3740041.713 r/s (5610400 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10485150.102 r/s (15728000 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 6682736.669 r/s (10024400 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10509420.100 r/s (15764800 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 8488524.766 r/s (12732800 rounds in 1.500 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 3568037.069 r/s (5352100 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 1502527.791 r/s (2253800 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1741907.415 r/s (2613000 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 952533.449 r/s (1429000 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1404793.511 r/s (2107200 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 1161886.583 r/s (1743200 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1487639.454 r/s (2232000 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 1328733.541 r/s (1993600 rounds in 1.500 seconds)

```
## REQUEST PROGRAM
```
A program simulating a concurrent server.

Mutexed structures with 2 threads total time: 414.831888ms
Lockfree structures with 2 threads total time: 369.396325ms

Mutexed structures with 4 threads total time: 420.799606ms
Lockfree structures with 4 threads total time: 408.129844ms

Mutexed structures with 8 threads total time: 459.51381ms
Lockfree structures with 8 threads total time: 767.642672ms

Mutexed structures with 16 threads total time: 1.38639048s
Lockfree structures with 16 threads total time: 1.213413045s
```
