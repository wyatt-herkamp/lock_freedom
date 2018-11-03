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
CPU MHz:             2727.738
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
mean of 20289023.126 r/s (30433600 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 17437771.297 r/s (26156700 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 17481812.916 r/s (26222800 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 46264776.247 r/s (69397600 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 38890656.659 r/s (58336000 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 40797276.680 r/s (61196000 rounds in 1.500 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 46560918.417 r/s (69841600 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 33764596.815 r/s (50648000 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 41564872.241 r/s (62348800 rounds in 1.500 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 40110395.186 r/s (60166400 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 28334225.884 r/s (42502400 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 37535527.678 r/s (56304000 rounds in 1.500 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 41313359.880 r/s (61977600 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 15384920.238 r/s (23091200 rounds in 1.501 seconds)
Target 2 (lockfree):
mean of 39534902.496 r/s (59302400 rounds in 1.500 seconds)

Result for 512 threads:
Target 0 (std/global):
mean of 33086295.522 r/s (49664000 rounds in 1.501 seconds)
Target 1 (blocking):
mean of 5971389.090 r/s (9011200 rounds in 1.509 seconds)
Target 2 (lockfree):
mean of 32147968.426 r/s (48230400 rounds in 1.500 seconds)

```
## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 14100434.649 r/s (21150700 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 11264901.799 r/s (16897400 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 5494754.339 r/s (8242200 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 16040337.496 r/s (24060600 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 5041747.396 r/s (7562800 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 3990305.842 r/s (5985600 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 10121562.628 r/s (15182800 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 5244197.800 r/s (7866400 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 3999345.561 r/s (5999200 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8678200.147 r/s (13017600 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 4595081.478 r/s (6892800 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 4480871.892 r/s (6721600 rounds in 1.500 seconds)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 14073944.605 r/s (21111000 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 10898119.885 r/s (16347200 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 9830120.928 r/s (14745200 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 18009982.458 r/s (27015000 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 4645694.712 r/s (6968600 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 6716306.446 r/s (10074600 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 9595450.022 r/s (14393200 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 5087570.551 r/s (7631600 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 4856901.409 r/s (7285600 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8408512.396 r/s (12612800 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 4541142.278 r/s (6812000 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 5237426.698 r/s (7856800 rounds in 1.500 seconds)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 2781778.253 r/s (4520600 rounds in 1.625 seconds)
Target 1 (lockfree insert):
mean of 2117908.925 r/s (3176900 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 2067735.088 r/s (3101800 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 2201087.590 r/s (3301800 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2272392.027 r/s (3408800 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 2751870.761 r/s (4128000 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2465157.024 r/s (3698400 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 3281435.758 r/s (4922400 rounds in 1.500 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 4659740.234 r/s (6989700 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 3175064.487 r/s (4762700 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 2312239.331 r/s (3468400 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 3658797.073 r/s (5488200 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2584490.348 r/s (3876800 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 5002796.453 r/s (7504400 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2842340.957 r/s (4264000 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 5698985.821 r/s (8548800 rounds in 1.500 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 7819575.947 r/s (11729400 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 1604650.732 r/s (2407100 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 16053977.439 r/s (24081000 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 3746370.196 r/s (5619800 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10330055.279 r/s (15495600 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 6679388.571 r/s (10019200 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10366448.764 r/s (15550400 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 8536642.350 r/s (12805600 rounds in 1.500 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 3578713.660 r/s (5368100 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 1499360.444 r/s (2249100 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1795881.445 r/s (2694000 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 935558.981 r/s (1403400 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1402074.546 r/s (2103200 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 1155637.683 r/s (1733600 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1484214.938 r/s (2226400 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 1334280.301 r/s (2001600 rounds in 1.500 seconds)

```
## REQUEST PROGRAM
```
A program simulating a concurrent server.

Mutexed structures with 2 threads total time: 414.015654ms
Lockfree structures with 2 threads total time: 374.9175ms

Mutexed structures with 4 threads total time: 412.114771ms
Lockfree structures with 4 threads total time: 373.98174ms

Mutexed structures with 8 threads total time: 462.807764ms
Lockfree structures with 8 threads total time: 885.819422ms

Mutexed structures with 16 threads total time: 1.532378875s
Lockfree structures with 16 threads total time: 1.473968029s
```
