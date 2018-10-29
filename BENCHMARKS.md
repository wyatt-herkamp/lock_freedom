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
CPU MHz:             600.068
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
mean of 19307629.570 r/s (28961500 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 17438224.722 r/s (26157400 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 18085566.301 r/s (27128400 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 46099424.928 r/s (69149600 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 39090395.119 r/s (58636000 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 40573578.036 r/s (60860400 rounds in 1.500 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 46653679.865 r/s (69980800 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 33299775.162 r/s (49950400 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 41601740.807 r/s (62403200 rounds in 1.500 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 39638603.681 r/s (59459200 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 28368900.519 r/s (42553600 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 37016873.309 r/s (55526400 rounds in 1.500 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 41726210.975 r/s (62592000 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 15089703.034 r/s (22643200 rounds in 1.501 seconds)
Target 2 (lockfree):
mean of 39338931.035 r/s (59020800 rounds in 1.500 seconds)

Result for 512 threads:
Target 0 (std/global):
mean of 22145391.607 r/s (33228800 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 5931128.316 r/s (8908800 rounds in 1.502 seconds)
Target 2 (lockfree):
mean of 21698688.382 r/s (32563200 rounds in 1.501 seconds)

## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 13739246.525 r/s (20608900 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 11393127.629 r/s (17089700 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 5244341.316 r/s (7866600 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 17179090.317 r/s (25768800 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 5122744.021 r/s (7684400 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 4645714.224 r/s (6968600 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 9771251.895 r/s (14657200 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 5120673.444 r/s (7681200 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 5177550.102 r/s (7766400 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8486618.785 r/s (12730400 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 4473598.640 r/s (6710400 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 5897700.615 r/s (8847200 rounds in 1.500 seconds)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 13852562.496 r/s (20778900 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 11055193.839 r/s (16582800 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 9540829.318 r/s (14311300 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 18293865.252 r/s (27440800 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 3713984.757 r/s (5571200 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 9084217.987 r/s (13626400 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 9929642.720 r/s (14894800 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 4942023.993 r/s (7413200 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 6997453.422 r/s (10496400 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8590804.196 r/s (12886400 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 4374640.142 r/s (6562400 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 7578659.669 r/s (11368000 rounds in 1.500 seconds)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 2740833.065 r/s (4520600 rounds in 1.649 seconds)
Target 1 (lockfree insert):
mean of 1703090.184 r/s (2554700 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 2114166.204 r/s (3171400 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 2292863.933 r/s (3439400 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2250019.604 r/s (3375200 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 3058853.611 r/s (4588400 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2442584.423 r/s (3664000 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 3831379.048 r/s (5747200 rounds in 1.500 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 4618462.402 r/s (6927700 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 2706264.379 r/s (4059400 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 2304420.168 r/s (3456800 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 4281571.510 r/s (6422400 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2559457.681 r/s (3839200 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 5820897.865 r/s (8731600 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2830957.649 r/s (4247200 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 7758174.330 r/s (11638400 rounds in 1.500 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 7976246.476 r/s (11964400 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 1332932.671 r/s (1999400 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 15230198.405 r/s (22845400 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 4673261.072 r/s (7010000 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10451664.018 r/s (15677600 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 9161129.663 r/s (13742000 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10434898.390 r/s (15652800 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 13163971.364 r/s (19746400 rounds in 1.500 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 3596719.886 r/s (5395100 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 1644878.897 r/s (2467400 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1766460.691 r/s (2649800 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 1207514.305 r/s (1811400 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1402121.728 r/s (2103200 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 1769589.690 r/s (2654400 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1488337.549 r/s (2232800 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 2031371.177 r/s (3047200 rounds in 1.500 seconds)

```
