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
CPU MHz:             2314.909
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
Running on Linux 4.18.12-arch1-1-ARCH #1 SMP PREEMPT Thu Oct 4 01:01:27 UTC 2018 x86_64 GNU/Linux

# Benchmarks
Benchmark code under [benchmark](benchmark) directory.
More rounds per seconds is better.

As you can see, there is a lot to improve!


## THREAD-LOCAL STORAGE
```
Result for 1 threads:
Target 0 (std/global):
mean of 19592453.017 r/s (29388700 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 18008560.945 r/s (27012900 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 17857713.118 r/s (26786600 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 46167336.783 r/s (69251200 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 39091701.330 r/s (58637600 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 40595327.993 r/s (60893200 rounds in 1.500 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 46602234.726 r/s (69904000 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 33653639.517 r/s (50481600 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 41624150.336 r/s (62436800 rounds in 1.500 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 40249555.645 r/s (60377600 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 29210045.557 r/s (43817600 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 37649837.553 r/s (56476800 rounds in 1.500 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 41347900.356 r/s (62028800 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 15472734.645 r/s (23219200 rounds in 1.501 seconds)
Target 2 (lockfree):
mean of 38968046.411 r/s (58457600 rounds in 1.500 seconds)

Result for 512 threads:
Target 0 (std/global):
mean of 29192836.884 r/s (43827200 rounds in 1.501 seconds)
Target 1 (blocking):
mean of 5732569.842 r/s (8652800 rounds in 1.509 seconds)
Target 2 (lockfree):
mean of 32001657.442 r/s (48025600 rounds in 1.501 seconds)

## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 14045613.012 r/s (21068500 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 11265410.610 r/s (16898200 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 5572491.993 r/s (8358800 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 17392438.475 r/s (26088800 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 4620298.212 r/s (6930600 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 4351050.791 r/s (6526600 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 9905584.410 r/s (14858800 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 5151786.019 r/s (7728000 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 4096224.323 r/s (6144400 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8457217.198 r/s (12686400 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 4547086.750 r/s (6820800 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 4576317.584 r/s (6864800 rounds in 1.500 seconds)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 13921756.666 r/s (20882700 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 11193389.441 r/s (16790100 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 9965603.133 r/s (14948500 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 19034511.291 r/s (28551800 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 3680467.225 r/s (5520800 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 7386341.722 r/s (11079600 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 10092375.946 r/s (15138800 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 5064983.540 r/s (7597600 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 5471257.263 r/s (8207200 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8573077.581 r/s (12860000 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 4484097.813 r/s (6726400 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 5920300.559 r/s (8880800 rounds in 1.500 seconds)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 2718657.851 r/s (4078100 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 1680671.416 r/s (2521100 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 2042876.867 r/s (3064400 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 2256001.672 r/s (3384200 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2016659.913 r/s (3025200 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 3009663.508 r/s (4514800 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2257868.034 r/s (3387200 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 3779979.376 r/s (5670400 rounds in 1.500 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 4301810.476 r/s (6452800 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 2729775.445 r/s (4094700 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 2138907.126 r/s (3208400 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 3945443.260 r/s (5918200 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2286485.476 r/s (3430000 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 5472509.506 r/s (8208800 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2502181.198 r/s (3753600 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 6117641.872 r/s (9176800 rounds in 1.500 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 7875112.506 r/s (11812700 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 1324099.026 r/s (1986200 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 15687448.427 r/s (23531200 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 4711854.664 r/s (7067800 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10414392.266 r/s (15621600 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 9234718.435 r/s (13852400 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10445818.964 r/s (15668800 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 13264770.906 r/s (19897600 rounds in 1.500 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 3334658.746 r/s (5002100 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 1626436.327 r/s (2439700 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1245832.330 r/s (1868800 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 1196461.329 r/s (1794800 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1013028.755 r/s (1520000 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 1443302.634 r/s (2165200 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1141784.073 r/s (1712800 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 1984465.663 r/s (2976800 rounds in 1.500 seconds)

```
