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
CPU MHz:             2274.145
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
mean of 19362652.532 r/s (29044000 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 17701201.550 r/s (26551900 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 17817780.662 r/s (26726700 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 46628114.099 r/s (69942400 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 39166532.378 r/s (58750000 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 40875159.997 r/s (61312800 rounds in 1.500 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 46558274.161 r/s (69838400 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 33850260.464 r/s (50776000 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 41589007.747 r/s (62384000 rounds in 1.500 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 40389161.830 r/s (60585600 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 28638189.856 r/s (42960000 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 37743813.169 r/s (56617600 rounds in 1.500 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 41322658.697 r/s (61990400 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 15612575.683 r/s (23424000 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 39334408.297 r/s (59008000 rounds in 1.500 seconds)

Result for 512 threads:
Target 0 (std/global):
mean of 32575132.360 r/s (48896000 rounds in 1.501 seconds)
Target 1 (blocking):
mean of 5744066.002 r/s (8652800 rounds in 1.506 seconds)
Target 2 (lockfree):
mean of 32010601.360 r/s (48076800 rounds in 1.502 seconds)

## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 13755928.345 r/s (20633900 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 11097069.044 r/s (16645700 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 5549971.099 r/s (8325000 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 16064493.258 r/s (24096800 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 4179179.820 r/s (6268800 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 4554557.154 r/s (6832000 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 10014123.112 r/s (15021200 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 5023899.900 r/s (7536000 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 4048220.058 r/s (6072400 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8584667.485 r/s (12877600 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 4425684.761 r/s (6639200 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 4563674.182 r/s (6845600 rounds in 1.500 seconds)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 14096270.943 r/s (21144500 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 10922813.596 r/s (16384300 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 9283604.330 r/s (13925500 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 18167095.987 r/s (27250800 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 3259613.521 r/s (4889600 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 9976203.165 r/s (14964400 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 10270634.095 r/s (15406400 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 4984738.375 r/s (7477200 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 6773471.945 r/s (10160400 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8889055.200 r/s (13333600 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 4408011.578 r/s (6612800 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 7436409.251 r/s (11155200 rounds in 1.500 seconds)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 2787639.554 r/s (4520600 rounds in 1.622 seconds)
Target 1 (lockfree insert):
mean of 1709690.642 r/s (2564600 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 2070144.792 r/s (3105400 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 2288716.134 r/s (3433200 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2258363.925 r/s (3388000 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 3063934.599 r/s (4596000 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2449313.213 r/s (3674400 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 3827267.305 r/s (5741600 rounds in 1.500 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 4595843.792 r/s (6893800 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 2710004.270 r/s (4065100 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 2311647.355 r/s (3467600 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 4393933.449 r/s (6591000 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2563061.285 r/s (3844800 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 5782657.561 r/s (8674000 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2833228.415 r/s (4250400 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 7705102.220 r/s (11558400 rounds in 1.500 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 7836373.252 r/s (11754600 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 1321279.880 r/s (1982000 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 15698743.265 r/s (23548200 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 4606238.369 r/s (6909400 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10405942.433 r/s (15609200 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 9139710.119 r/s (13710000 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10411993.532 r/s (15618400 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 13084553.801 r/s (19627200 rounds in 1.500 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 3582814.224 r/s (5374300 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 1641434.697 r/s (2462200 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1795220.896 r/s (2693000 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 1213916.546 r/s (1821000 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1402433.200 r/s (2104000 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 1773831.985 r/s (2660800 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1488751.362 r/s (2233600 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 2041163.044 r/s (3062400 rounds in 1.500 seconds)

```
