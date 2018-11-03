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
CPU MHz:             3000.069
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
mean of 21709020.137 r/s (32563600 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 17217403.111 r/s (25826200 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 17398685.475 r/s (26098100 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 45160787.897 r/s (67741200 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 38077521.992 r/s (57116400 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 39608223.758 r/s (59412400 rounds in 1.500 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 45357091.937 r/s (68036800 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 33127176.649 r/s (49691200 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 40806074.531 r/s (61209600 rounds in 1.500 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 39135635.986 r/s (58704000 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 27813141.808 r/s (41721600 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 36778603.677 r/s (55168000 rounds in 1.500 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 40377931.832 r/s (60569600 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 15270503.581 r/s (22912000 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 38788939.512 r/s (58188800 rounds in 1.500 seconds)

Result for 512 threads:
Target 0 (std/global):
mean of 31954295.106 r/s (47974400 rounds in 1.501 seconds)
Target 1 (blocking):
mean of 5820576.520 r/s (8755200 rounds in 1.504 seconds)
Target 2 (lockfree):
mean of 30958081.677 r/s (46438400 rounds in 1.500 seconds)

```
## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 13764928.084 r/s (20647400 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 11159868.747 r/s (16739900 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 6375561.827 r/s (9563400 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 15230278.503 r/s (22845600 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 5063410.254 r/s (7595200 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 4653708.808 r/s (6980600 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 9841136.675 r/s (14762000 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 5111697.797 r/s (7667600 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 3776561.769 r/s (5665200 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8440321.312 r/s (12660800 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 4498777.889 r/s (6748800 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 4226670.993 r/s (6340800 rounds in 1.500 seconds)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 14184222.053 r/s (21276400 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 10935377.036 r/s (16403100 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 9666782.187 r/s (14500200 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 17114089.921 r/s (25671200 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 4087740.279 r/s (6131800 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 8238619.267 r/s (12358000 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 9532206.334 r/s (14298400 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 4977810.653 r/s (7466800 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 5444284.951 r/s (8166800 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8329577.323 r/s (12495200 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 4440812.303 r/s (6661600 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 6005701.766 r/s (9008800 rounds in 1.500 seconds)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 2759402.444 r/s (4520600 rounds in 1.638 seconds)
Target 1 (lockfree insert):
mean of 2104503.681 r/s (3156800 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 2108369.146 r/s (3162600 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 2279443.451 r/s (3419400 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2205300.885 r/s (3308000 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 2681377.526 r/s (4022400 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2400867.902 r/s (3601600 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 3182807.124 r/s (4774400 rounds in 1.500 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 4692914.734 r/s (7039400 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 3177626.963 r/s (4766500 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 2286757.645 r/s (3430200 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 3740510.715 r/s (5611000 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2494743.480 r/s (3742400 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 4897868.159 r/s (7347200 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2755961.901 r/s (4134400 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 5618625.100 r/s (8428000 rounds in 1.500 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 7581986.256 r/s (11373000 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 1559506.232 r/s (2339300 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 14600429.651 r/s (21900800 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 3733287.360 r/s (5600400 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10211790.039 r/s (15318000 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 6557485.926 r/s (9836400 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10165706.930 r/s (15248800 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 8318056.315 r/s (12477600 rounds in 1.500 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 3538972.950 r/s (5308500 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 1466696.146 r/s (2200100 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1644193.386 r/s (2466400 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 939720.526 r/s (1409800 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1373843.811 r/s (2060800 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 1146247.815 r/s (1719600 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1457521.102 r/s (2186400 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 1318707.361 r/s (1978400 rounds in 1.500 seconds)

```
## REQUEST PROGRAM
```
A program simulating a concurrent server.

Mutexed structures with 2 threads total time: 782.007566ms
Lockfree structures with 2 threads total time: 726.243388ms

Mutexed structures with 4 threads total time: 885.574784ms
Lockfree structures with 4 threads total time: 996.521326ms

Mutexed structures with 8 threads total time: 1.004586234s
Lockfree structures with 8 threads total time: 1.241793604s

Mutexed structures with 16 threads total time: 1.839483929s
Lockfree structures with 16 threads total time: 2.057071475s
```
