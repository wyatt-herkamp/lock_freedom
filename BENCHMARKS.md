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
CPU MHz:             600.033
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
mean of 507702516.249 r/s (634727424 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 137870401.565 r/s (172363776 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 167908969.795 r/s (209916928 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 1201494164.238 r/s (1502033920 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 199097907.138 r/s (248896512 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 275372574.913 r/s (344248320 rounds in 1.250 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 1197968280.139 r/s (1497885696 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 127151648.832 r/s (158982144 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 276306975.238 r/s (345464832 rounds in 1.250 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 1202302934.960 r/s (1503462400 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 90471055.235 r/s (113153024 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 274143314.557 r/s (342829056 rounds in 1.251 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 1196797913.474 r/s (1498395648 rounds in 1.252 seconds)
Target 1 (blocking):
mean of 38954430.311 r/s (48899072 rounds in 1.255 seconds)
Target 2 (lockfree):
mean of 204507608.385 r/s (256094208 rounds in 1.252 seconds)

```
## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 45865666.688 r/s (57341952 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 29777985.284 r/s (37225472 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 12613278.250 r/s (15769600 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 11975288.826 r/s (14972928 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4009181.359 r/s (5012480 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 3604348.289 r/s (4506624 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 8641242.663 r/s (10804224 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4961305.473 r/s (6202368 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 3850200.523 r/s (4815872 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8247637.752 r/s (10313728 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4887162.681 r/s (6111232 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 3966698.284 r/s (4977664 rounds in 1.255 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 8232350.886 r/s (10301440 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4868416.249 r/s (6092800 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 3949844.057 r/s (4956160 rounds in 1.255 seconds)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 48698296.138 r/s (60882944 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 27281847.399 r/s (34105344 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 19892717.537 r/s (24868864 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 8732940.111 r/s (10917888 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4728348.861 r/s (5911552 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5831042.409 r/s (7290880 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 7672210.405 r/s (9592832 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 5115147.730 r/s (6395904 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5600703.837 r/s (7003136 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 7238489.980 r/s (9052160 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4924829.077 r/s (6159360 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 5641496.023 r/s (7062528 rounds in 1.252 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 7218661.524 r/s (9033728 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4884313.874 r/s (6112256 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 5604583.804 r/s (7041024 rounds in 1.256 seconds)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 3245663.189 r/s (4520960 rounds in 1.393 seconds)
Target 1 (lockfree insert):
mean of 1878293.411 r/s (2349056 rounds in 1.251 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 1955374.895 r/s (2445312 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 2074364.447 r/s (2594816 rounds in 1.251 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2248484.902 r/s (2811904 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 2529447.944 r/s (3166208 rounds in 1.252 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2418818.767 r/s (3026944 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 2386679.528 r/s (2997248 rounds in 1.256 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 6234657.210 r/s (7794688 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 3893628.721 r/s (4868096 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 1997583.613 r/s (2498560 rounds in 1.251 seconds)
Target 1 (lockfree get):
mean of 3422334.020 r/s (4280320 rounds in 1.251 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2368185.870 r/s (2963456 rounds in 1.251 seconds)
Target 1 (lockfree get):
mean of 4928136.404 r/s (6163456 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2389979.966 r/s (2990080 rounds in 1.251 seconds)
Target 1 (lockfree get):
mean of 4957998.278 r/s (6203392 rounds in 1.251 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 21530076.185 r/s (26916864 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 1813681.443 r/s (2268160 rounds in 1.251 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 13327937.249 r/s (16661504 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 3660741.868 r/s (4583424 rounds in 1.252 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10562804.879 r/s (13205504 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 7386938.751 r/s (9244672 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10364420.400 r/s (12960768 rounds in 1.251 seconds)
Target 1 (lockfree remove):
mean of 8097692.443 r/s (10127360 rounds in 1.251 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 4440336.780 r/s (5552128 rounds in 1.250 seconds)
Target 1 (lockfree mixed):
mean of 1373250.940 r/s (1717248 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1481844.670 r/s (1853440 rounds in 1.251 seconds)
Target 1 (lockfree mixed):
mean of 836986.122 r/s (1046528 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1285632.698 r/s (1608704 rounds in 1.251 seconds)
Target 1 (lockfree mixed):
mean of 1024728.383 r/s (1283072 rounds in 1.252 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1370936.161 r/s (1720320 rounds in 1.255 seconds)
Target 1 (lockfree mixed):
mean of 896221.335 r/s (1274880 rounds in 1.423 seconds)

```
## REQUEST PROGRAM
```
A program simulating a concurrent server.

Mutexed structures with 2 threads total time: 801.557332ms
Lockfree structures with 2 threads total time: 766.807919ms

Mutexed structures with 4 threads total time: 849.087745ms
Lockfree structures with 4 threads total time: 987.866822ms

Mutexed structures with 8 threads total time: 997.107577ms
Lockfree structures with 8 threads total time: 1.332784084s

Mutexed structures with 16 threads total time: 1.53092204s
Lockfree structures with 16 threads total time: 1.946800344s
```
