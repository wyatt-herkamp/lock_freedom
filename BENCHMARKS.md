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
CPU MHz:             1750.955
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
mean of 43427032.727 r/s (65140700 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 17538785.057 r/s (26308200 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 17422690.113 r/s (26134100 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 45877951.932 r/s (68817200 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 38891997.148 r/s (58338000 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 41027347.496 r/s (61541200 rounds in 1.500 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 46352625.421 r/s (69529600 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 36352278.598 r/s (54529600 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 41720598.567 r/s (62582400 rounds in 1.500 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 40160818.709 r/s (60243200 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 31910852.674 r/s (47868800 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 37990232.083 r/s (56985600 rounds in 1.500 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 41664160.886 r/s (62502400 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 22630074.338 r/s (33958400 rounds in 1.501 seconds)
Target 2 (lockfree):
mean of 39563938.204 r/s (59353600 rounds in 1.500 seconds)

Result for 512 threads:
Target 0 (std/global):
mean of 33098199.496 r/s (49664000 rounds in 1.501 seconds)
Target 1 (blocking):
mean of 9296218.776 r/s (13977600 rounds in 1.504 seconds)
Target 2 (lockfree):
mean of 31627463.241 r/s (47462400 rounds in 1.501 seconds)

## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 13798979.991 r/s (20698500 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 11204907.121 r/s (16807400 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 5309949.888 r/s (7965000 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 14821516.328 r/s (22232400 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 3234483.585 r/s (4851800 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 4803558.263 r/s (7205400 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 9902645.239 r/s (14854400 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 5076788.506 r/s (7615200 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 5227606.362 r/s (7841600 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8631749.593 r/s (12948000 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 4509635.386 r/s (6764800 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 5929958.672 r/s (8895200 rounds in 1.500 seconds)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 14222107.174 r/s (21333200 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 10905424.150 r/s (16358200 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 9545574.602 r/s (14318400 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 15498489.266 r/s (23247800 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 4677584.910 r/s (7016400 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 8319110.780 r/s (12478800 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 10078460.307 r/s (15118000 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 5049424.691 r/s (7574400 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 6896484.290 r/s (10344800 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8623756.419 r/s (12936000 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 4469440.094 r/s (6704800 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 7454143.597 r/s (11181600 rounds in 1.500 seconds)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 2817207.983 r/s (4520600 rounds in 1.605 seconds)
Target 1 (lockfree insert):
mean of 1709683.112 r/s (2564600 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 2112262.956 r/s (3168600 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 2287887.358 r/s (3432000 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2269324.068 r/s (3404000 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 3042969.051 r/s (4564800 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2476905.597 r/s (3716000 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 3838174.430 r/s (5757600 rounds in 1.500 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 4637279.040 r/s (6956000 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 2718280.981 r/s (4077500 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 2313613.721 r/s (3470600 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 4343113.685 r/s (6514800 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2566179.743 r/s (3849600 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 5774800.166 r/s (8662400 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2849322.030 r/s (4274400 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 7712773.257 r/s (11569600 rounds in 1.500 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 7932316.839 r/s (11898500 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 1312033.822 r/s (1968100 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 14918247.193 r/s (22377400 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 4530627.812 r/s (6796000 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10425456.435 r/s (15638400 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 9180928.096 r/s (13771600 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10411098.471 r/s (15616800 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 13127738.014 r/s (19692000 rounds in 1.500 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 3577162.948 r/s (5365800 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 1644163.885 r/s (2466300 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1754561.382 r/s (2632000 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 1207164.076 r/s (1810800 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1413556.291 r/s (2120400 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 1779027.938 r/s (2668800 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1504446.930 r/s (2256800 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 2045085.329 r/s (3068000 rounds in 1.500 seconds)

```
