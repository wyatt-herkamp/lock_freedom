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
CPU MHz:             2658.937
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
mean of 19301746.714 r/s (28952700 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 17534854.720 r/s (26302300 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 17477215.167 r/s (26215900 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 46368753.539 r/s (69553200 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 38843180.578 r/s (58264800 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 41019295.179 r/s (61529200 rounds in 1.500 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 46849102.372 r/s (70275200 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 33266472.861 r/s (49900800 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 41545055.012 r/s (62318400 rounds in 1.500 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 40210413.484 r/s (60316800 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 28589213.635 r/s (42886400 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 37503695.545 r/s (56256000 rounds in 1.500 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 41579484.426 r/s (62374400 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 15552913.459 r/s (23334400 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 39119445.257 r/s (58688000 rounds in 1.500 seconds)

Result for 512 threads:
Target 0 (std/global):
mean of 29395381.630 r/s (44134400 rounds in 1.501 seconds)
Target 1 (blocking):
mean of 5768373.023 r/s (8652800 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 31907798.088 r/s (47872000 rounds in 1.500 seconds)

```
## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 14123950.378 r/s (21186000 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 11318793.956 r/s (16978200 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 5514154.997 r/s (8271300 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 16816478.893 r/s (25224800 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 5382729.436 r/s (8074200 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 3648140.896 r/s (5472400 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 9979712.429 r/s (14969600 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 5201028.141 r/s (7801600 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 3867928.534 r/s (5802000 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8559691.646 r/s (12840000 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 4536728.780 r/s (6805600 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 4340310.887 r/s (6511200 rounds in 1.500 seconds)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 14108511.935 r/s (21162800 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 10876830.788 r/s (16315300 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 9870726.220 r/s (14806100 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 18443091.051 r/s (27664800 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 3340106.399 r/s (5010200 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 6685478.119 r/s (10028400 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 9767031.701 r/s (14650800 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 5061209.722 r/s (7592000 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 5459797.755 r/s (8190000 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8507859.316 r/s (12762400 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 4532746.695 r/s (6799200 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 5982511.056 r/s (8974400 rounds in 1.500 seconds)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 2739419.231 r/s (4109200 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 2113087.631 r/s (3169700 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 2140678.564 r/s (3211200 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 2183696.537 r/s (3275600 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2023579.501 r/s (3035600 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 2741015.019 r/s (4111600 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2262921.880 r/s (3394400 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 3254698.769 r/s (4882400 rounds in 1.500 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 4160097.532 r/s (6240200 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 3164332.015 r/s (4746500 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 2203879.348 r/s (3306000 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 3661433.465 r/s (5492200 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2266833.231 r/s (3400400 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 5004846.330 r/s (7507600 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2565790.405 r/s (3848800 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 5681362.811 r/s (8522400 rounds in 1.500 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 7789714.056 r/s (11684600 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 1603105.494 r/s (2404700 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 16414465.115 r/s (24621800 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 3760697.611 r/s (5641200 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10459127.658 r/s (15688800 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 6686142.258 r/s (10029600 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10447352.965 r/s (15671200 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 8407044.336 r/s (12611200 rounds in 1.500 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 3310054.905 r/s (4965100 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 1498439.762 r/s (2247700 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1249187.183 r/s (1873800 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 931311.051 r/s (1397000 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1100943.082 r/s (1651600 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 1134100.836 r/s (1701200 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1154183.100 r/s (1732000 rounds in 1.501 seconds)
Target 1 (lockfree mixed):
mean of 1332658.079 r/s (1999200 rounds in 1.500 seconds)

```
## REQUEST PROGRAM
```
A program simulating a concurrent server.

Mutexed structures with 4 threads total time: 442.754369ms
Lockfree structures with 4 threads total time: 478.298914ms

Mutexed structures with 8 threads total time: 532.94558ms
Lockfree structures with 8 threads total time: 541.725409ms

Mutexed structures with 16 threads total time: 1.279965s
Lockfree structures with 16 threads total time: 1.17312474s

Mutexed structures with 32 threads total time: 2.741485557s
Lockfree structures with 32 threads total time: 2.664984742s
```
