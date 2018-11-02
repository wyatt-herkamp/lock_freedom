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
CPU MHz:             2486.318
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
mean of 19599633.504 r/s (29399500 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 17543747.158 r/s (26315700 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 17485398.345 r/s (26228100 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 46233133.606 r/s (69350000 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 38724903.970 r/s (58087600 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 40727182.581 r/s (61091200 rounds in 1.500 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 46680982.037 r/s (70022400 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 33699401.918 r/s (50550400 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 41508481.220 r/s (62264000 rounds in 1.500 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 40084596.846 r/s (60128000 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 28555987.563 r/s (42835200 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 37345757.610 r/s (56019200 rounds in 1.500 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 41097663.224 r/s (61657600 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 15390641.884 r/s (23091200 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 39423960.050 r/s (59136000 rounds in 1.500 seconds)

Result for 512 threads:
Target 0 (std/global):
mean of 32357767.902 r/s (48537600 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 5796671.398 r/s (8755200 rounds in 1.510 seconds)
Target 2 (lockfree):
mean of 32200335.783 r/s (48332800 rounds in 1.501 seconds)

## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 14170295.442 r/s (21255500 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 11323299.756 r/s (16985000 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 5482514.397 r/s (8223800 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 16362968.565 r/s (24544600 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 3996328.143 r/s (5994600 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 3803997.048 r/s (5706000 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 10181057.766 r/s (15272000 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 5191916.867 r/s (7788000 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 3955106.106 r/s (5932800 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8678616.807 r/s (13018400 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 4546356.487 r/s (6820000 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 4462923.205 r/s (6695200 rounds in 1.500 seconds)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 14292901.298 r/s (21439400 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 10887149.767 r/s (16330800 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 9666324.621 r/s (14499500 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 17995077.454 r/s (26992800 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 3957113.595 r/s (5935800 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 8074484.655 r/s (12111800 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 9639633.762 r/s (14459600 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 4993248.671 r/s (7490000 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 5434104.290 r/s (8151200 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8456837.188 r/s (12685600 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 4476218.580 r/s (6714400 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 6008422.354 r/s (9012800 rounds in 1.500 seconds)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 2792611.395 r/s (4520600 rounds in 1.619 seconds)
Target 1 (lockfree insert):
mean of 2119729.563 r/s (3179600 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 2085792.746 r/s (3128800 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 2262045.397 r/s (3393200 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2255719.301 r/s (3383600 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 2775497.934 r/s (4163600 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2441670.264 r/s (3663200 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 3299731.274 r/s (4949600 rounds in 1.500 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 4621944.564 r/s (6933000 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 3208985.057 r/s (4813500 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 2291113.642 r/s (3436800 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 3760844.791 r/s (5641400 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2551733.330 r/s (3828000 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 5033363.861 r/s (7550400 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2807889.983 r/s (4212000 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 5707713.448 r/s (8562400 rounds in 1.500 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 7730502.329 r/s (11595800 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 1606639.275 r/s (2410000 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 16060855.877 r/s (24091400 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 3757862.755 r/s (5637000 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10490816.235 r/s (15736400 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 6696512.952 r/s (10045200 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10479484.004 r/s (15720000 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 8539828.795 r/s (12810400 rounds in 1.500 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 3576720.655 r/s (5365100 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 1492807.093 r/s (2239300 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1775394.157 r/s (2663200 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 956595.743 r/s (1435000 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1392905.160 r/s (2089600 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 1170614.602 r/s (1756000 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1478217.241 r/s (2217600 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 1345594.300 r/s (2018400 rounds in 1.500 seconds)

```
