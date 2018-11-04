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
CPU MHz:             1027.635
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
mean of 382976448.810 r/s (478804992 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 141491969.047 r/s (176894976 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 168138683.081 r/s (210205696 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 875055153.256 r/s (1093926912 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 192204951.174 r/s (240281600 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 247813531.624 r/s (309797888 rounds in 1.250 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 874015048.448 r/s (1093091328 rounds in 1.251 seconds)
Target 1 (blocking):
mean of 125644396.654 r/s (157110272 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 250557483.227 r/s (313274368 rounds in 1.250 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 874710588.134 r/s (1093868544 rounds in 1.251 seconds)
Target 1 (blocking):
mean of 88833276.601 r/s (111112192 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 233130814.862 r/s (291556352 rounds in 1.251 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 872981526.129 r/s (1093917696 rounds in 1.253 seconds)
Target 1 (blocking):
mean of 44073162.760 r/s (55349248 rounds in 1.256 seconds)
Target 2 (lockfree):
mean of 259472956.622 r/s (325004288 rounds in 1.253 seconds)

```
## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 45621318.675 r/s (57036800 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 26212242.442 r/s (32771072 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 11304821.335 r/s (14134272 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 9150681.870 r/s (11440128 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 2458757.626 r/s (3074048 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5544808.527 r/s (6932480 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 7836991.003 r/s (9799680 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4538477.699 r/s (5675008 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5001630.014 r/s (6254592 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 7413078.716 r/s (9272320 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4409662.694 r/s (5515264 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 4926683.752 r/s (6163456 rounds in 1.251 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 7369219.190 r/s (9221120 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4388091.403 r/s (5493760 rounds in 1.252 seconds)
Target 2 (lockfree):
mean of 4910420.701 r/s (6147072 rounds in 1.252 seconds)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 48345315.167 r/s (60441600 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 24574297.506 r/s (30723072 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 19236837.244 r/s (24050688 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 8301412.462 r/s (10378240 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 3247567.932 r/s (4060160 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5986439.920 r/s (7484416 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 7729290.955 r/s (9665536 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4426244.151 r/s (5534720 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5649741.614 r/s (7064576 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 7367773.291 r/s (9213952 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4357486.774 r/s (5450752 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 5558750.117 r/s (6969344 rounds in 1.254 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 7383921.925 r/s (9241600 rounds in 1.252 seconds)
Target 1 (mutex linked list):
mean of 4337292.214 r/s (5430272 rounds in 1.252 seconds)
Target 2 (lockfree):
mean of 5566823.500 r/s (7008256 rounds in 1.259 seconds)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 3226549.793 r/s (4520960 rounds in 1.401 seconds)
Target 1 (lockfree insert):
mean of 2417858.495 r/s (3022848 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 1951193.561 r/s (2440192 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 2247680.873 r/s (2811904 rounds in 1.251 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2226614.797 r/s (2785280 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 2824016.261 r/s (3532800 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2321219.133 r/s (2907136 rounds in 1.252 seconds)
Target 1 (lockfree insert):
mean of 2762057.004 r/s (3462144 rounds in 1.253 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 6170187.304 r/s (7714816 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 3703641.394 r/s (4630528 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 1990736.605 r/s (2489344 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 3544282.008 r/s (4431872 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2375643.678 r/s (2971648 rounds in 1.251 seconds)
Target 1 (lockfree get):
mean of 4927310.204 r/s (6160384 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2384735.607 r/s (2984960 rounds in 1.252 seconds)
Target 1 (lockfree get):
mean of 5156623.916 r/s (6450176 rounds in 1.251 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 20202995.699 r/s (25257984 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 1685627.329 r/s (2108416 rounds in 1.251 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 12888012.958 r/s (16111616 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 3763581.016 r/s (4706304 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10561294.048 r/s (13205504 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 7600924.371 r/s (9502720 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10431730.848 r/s (13044736 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 8386584.818 r/s (10500096 rounds in 1.252 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 4449050.395 r/s (5563392 rounds in 1.250 seconds)
Target 1 (lockfree mixed):
mean of 1568698.959 r/s (1961984 rounds in 1.251 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1516633.958 r/s (1896448 rounds in 1.250 seconds)
Target 1 (lockfree mixed):
mean of 916111.760 r/s (1146880 rounds in 1.252 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1308330.577 r/s (1637376 rounds in 1.252 seconds)
Target 1 (lockfree mixed):
mean of 908570.362 r/s (1136640 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1230896.319 r/s (1543168 rounds in 1.254 seconds)
Target 1 (lockfree mixed):
mean of 1145833.551 r/s (1472512 rounds in 1.285 seconds)

```
## REQUEST PROGRAM
```
A program simulating a concurrent server.

Mutexed structures with 2 threads total time: 788.622584ms
Lockfree structures with 2 threads total time: 774.831752ms

Mutexed structures with 4 threads total time: 881.279874ms
Lockfree structures with 4 threads total time: 1.069009616s

Mutexed structures with 8 threads total time: 923.821453ms
Lockfree structures with 8 threads total time: 1.356380976s

Mutexed structures with 16 threads total time: 1.721107142s
Lockfree structures with 16 threads total time: 2.155549363s
```
