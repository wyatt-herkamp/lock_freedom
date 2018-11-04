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
CPU MHz:             2804.964
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
mean of 383210731.543 r/s (479053824 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 141378311.357 r/s (176752640 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 168401240.822 r/s (210526208 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 875101862.503 r/s (1093997568 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 200418954.271 r/s (250550272 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 240186220.606 r/s (300263424 rounds in 1.250 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 874125646.699 r/s (1092964352 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 127666316.891 r/s (159628288 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 244329500.749 r/s (305483776 rounds in 1.250 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 873587399.257 r/s (1092482048 rounds in 1.251 seconds)
Target 1 (blocking):
mean of 86268332.679 r/s (107896832 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 225658028.779 r/s (282209280 rounds in 1.251 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 873454034.129 r/s (1093856256 rounds in 1.252 seconds)
Target 1 (blocking):
mean of 40021968.647 r/s (50223104 rounds in 1.255 seconds)
Target 2 (lockfree):
mean of 235390429.737 r/s (294930432 rounds in 1.253 seconds)

```
## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 37009987.001 r/s (46269440 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 24925541.309 r/s (31160320 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 11915968.563 r/s (14899200 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 8872508.611 r/s (11092992 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 3168208.032 r/s (3960832 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 3714449.369 r/s (4643840 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 8067707.582 r/s (10087424 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4461798.352 r/s (5579776 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 3640893.445 r/s (4553728 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 7580306.462 r/s (9479168 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4380624.156 r/s (5480448 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 3560934.511 r/s (4485120 rounds in 1.260 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 7547100.584 r/s (9442304 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4351644.911 r/s (5449728 rounds in 1.252 seconds)
Target 2 (lockfree):
mean of 3514535.235 r/s (4404224 rounds in 1.253 seconds)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 48382033.719 r/s (60483584 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 24530419.686 r/s (30668800 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 19242886.351 r/s (24059904 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 7723787.161 r/s (9656320 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 2485771.185 r/s (3107840 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5862137.457 r/s (7328768 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 7941661.603 r/s (9930752 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4383825.565 r/s (5482496 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 4463179.630 r/s (5580800 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 6846763.341 r/s (8561664 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4412928.482 r/s (5520384 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 5549476.812 r/s (6946816 rounds in 1.252 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 7589481.138 r/s (9494528 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4385051.563 r/s (5489664 rounds in 1.252 seconds)
Target 2 (lockfree):
mean of 5299615.313 r/s (6794240 rounds in 1.282 seconds)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 3613279.939 r/s (4518912 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 2416780.769 r/s (3021824 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 1779841.691 r/s (2226176 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 2214808.302 r/s (2768896 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 1872755.748 r/s (2342912 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 2777523.770 r/s (3473408 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2086771.829 r/s (2611200 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 2795298.394 r/s (3531776 rounds in 1.263 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 4596742.119 r/s (5746688 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 3718630.453 r/s (4648960 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 1765663.916 r/s (2208768 rounds in 1.251 seconds)
Target 1 (lockfree get):
mean of 3871426.086 r/s (4841472 rounds in 1.251 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2090037.202 r/s (2613248 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 4952824.715 r/s (6193152 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2077695.890 r/s (2601984 rounds in 1.252 seconds)
Target 1 (lockfree get):
mean of 5065835.567 r/s (6336512 rounds in 1.251 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 16657003.992 r/s (20825088 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 1700232.235 r/s (2125824 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 12731770.877 r/s (15917056 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 3779299.651 r/s (4726784 rounds in 1.251 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10623333.794 r/s (13281280 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 7713982.402 r/s (9646080 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10514894.739 r/s (13149184 rounds in 1.251 seconds)
Target 1 (lockfree remove):
mean of 8327229.331 r/s (10414080 rounds in 1.251 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 4046166.700 r/s (5058560 rounds in 1.250 seconds)
Target 1 (lockfree mixed):
mean of 1591902.632 r/s (1990656 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1121059.806 r/s (1401856 rounds in 1.250 seconds)
Target 1 (lockfree mixed):
mean of 902807.973 r/s (1130496 rounds in 1.252 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1010983.376 r/s (1265664 rounds in 1.252 seconds)
Target 1 (lockfree mixed):
mean of 1139703.132 r/s (1427456 rounds in 1.252 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1045545.865 r/s (1311744 rounds in 1.255 seconds)
Target 1 (lockfree mixed):
mean of 1128913.911 r/s (1472512 rounds in 1.304 seconds)

```
## REQUEST PROGRAM
```
A program simulating a concurrent server.

Mutexed structures with 2 threads total time: 834.105838ms
Lockfree structures with 2 threads total time: 758.517858ms

Mutexed structures with 4 threads total time: 949.146694ms
Lockfree structures with 4 threads total time: 1.026231648s

Mutexed structures with 8 threads total time: 1.101566304s
Lockfree structures with 8 threads total time: 1.264692423s

Mutexed structures with 16 threads total time: 1.951851334s
Lockfree structures with 16 threads total time: 2.145521866s
```
