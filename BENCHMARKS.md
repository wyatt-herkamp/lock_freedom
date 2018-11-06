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
CPU MHz:             646.810
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
mean of 507684431.291 r/s (634712064 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 137471490.988 r/s (171867136 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 167608750.739 r/s (209544192 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 1202353821.248 r/s (1503112192 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 199033175.616 r/s (248814592 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 273015850.034 r/s (341306368 rounds in 1.250 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 1200975222.909 r/s (1501554688 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 124831034.260 r/s (156080128 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 275045764.155 r/s (343885824 rounds in 1.250 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 1197984746.134 r/s (1498161152 rounds in 1.251 seconds)
Target 1 (blocking):
mean of 85480173.054 r/s (106906624 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 275141186.521 r/s (344081408 rounds in 1.251 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 1197976452.767 r/s (1499767808 rounds in 1.252 seconds)
Target 1 (blocking):
mean of 38057823.955 r/s (47756288 rounds in 1.255 seconds)
Target 2 (lockfree):
mean of 212250366.434 r/s (265816064 rounds in 1.252 seconds)

```

## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 45837970.507 r/s (57307136 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 29838844.125 r/s (37305344 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 12420118.395 r/s (15527936 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 12091076.960 r/s (15115264 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4120953.783 r/s (5151744 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 3492930.168 r/s (4368384 rounds in 1.251 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 8661376.053 r/s (10828800 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 5085610.766 r/s (6360064 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 3299287.807 r/s (4125696 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8391593.172 r/s (10493952 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4987246.166 r/s (6239232 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 3324582.859 r/s (4162560 rounds in 1.252 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 8369046.328 r/s (10470400 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4962698.946 r/s (6209536 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 3336283.401 r/s (4220928 rounds in 1.265 seconds)

```

## STACK
```

Result for 1 threads:
Target 0 (mutex vector):
mean of 48091110.496 r/s (60125184 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 27322533.317 r/s (34158592 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 19993990.428 r/s (24996864 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 8537679.286 r/s (10673152 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4441046.827 r/s (5552128 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5822190.592 r/s (7278592 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 7730737.748 r/s (9665536 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4969180.892 r/s (6212608 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5610885.776 r/s (7016448 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 7374279.169 r/s (9222144 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4875453.637 r/s (6096896 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 5704397.573 r/s (7136256 rounds in 1.251 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 7356645.322 r/s (9202688 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4858660.011 r/s (6082560 rounds in 1.252 seconds)
Target 2 (lockfree):
mean of 5638199.044 r/s (7122944 rounds in 1.263 seconds)

```

## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 3283917.182 r/s (4520960 rounds in 1.377 seconds)
Target 1 (lockfree insert):
mean of 1927381.647 r/s (2410496 rounds in 1.251 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 1989386.424 r/s (2487296 rounds in 1.250 seconds)
Target 1 (lockfree insert):
mean of 2160268.996 r/s (2701312 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2184249.558 r/s (2732032 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 3472496.687 r/s (4343808 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2405997.284 r/s (3011584 rounds in 1.252 seconds)
Target 1 (lockfree insert):
mean of 3339584.249 r/s (4343808 rounds in 1.301 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 6534876.125 r/s (8169472 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 4631910.602 r/s (5790720 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 2037981.957 r/s (2548736 rounds in 1.251 seconds)
Target 1 (lockfree get):
mean of 7754456.331 r/s (9694208 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2108729.077 r/s (2638848 rounds in 1.251 seconds)
Target 1 (lockfree get):
mean of 11062942.553 r/s (13833216 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2497423.408 r/s (3124224 rounds in 1.251 seconds)
Target 1 (lockfree get):
mean of 10545824.066 r/s (13611008 rounds in 1.291 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 20721925.515 r/s (25907200 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 1494175.610 r/s (2095104 rounds in 1.402 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 12010047.193 r/s (15014912 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 5029782.409 r/s (6289408 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10751254.408 r/s (13442048 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 10861485.433 r/s (13579264 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10692538.787 r/s (13370368 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 10647509.929 r/s (13315072 rounds in 1.251 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 4492921.799 r/s (5617664 rounds in 1.250 seconds)
Target 1 (lockfree mixed):
mean of 1558319.208 r/s (1948672 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1487275.270 r/s (1859584 rounds in 1.250 seconds)
Target 1 (lockfree mixed):
mean of 952495.741 r/s (1191936 rounds in 1.251 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1313527.194 r/s (1644544 rounds in 1.252 seconds)
Target 1 (lockfree mixed):
mean of 1668664.091 r/s (2088960 rounds in 1.252 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1403266.012 r/s (1759232 rounds in 1.254 seconds)
Target 1 (lockfree mixed):
mean of 1616164.633 r/s (2219008 rounds in 1.373 seconds)

```

## REQUEST PROGRAM
```
A program simulating a concurrent server.

Mutexed HashMap and VecDeque with 2 threads total time: 1.268490484s
Mutexed HashMap and LinkedList with 2 threads total time: 1.272911057s
Lockfree structures with 2 threads total time: 872.151931ms

Mutexed HashMap and VecDeque with 4 threads total time: 761.296565ms
Mutexed HashMap and LinkedList with 4 threads total time: 1.898490986s
Lockfree structures with 4 threads total time: 971.036416ms

Mutexed HashMap and VecDeque with 8 threads total time: 1.131371076s
Mutexed HashMap and LinkedList with 8 threads total time: 2.711543437s
Lockfree structures with 8 threads total time: 2.367760568s

Mutexed HashMap and VecDeque with 16 threads total time: 2.819826465s
Mutexed HashMap and LinkedList with 16 threads total time: 4.545691653s
Lockfree structures with 16 threads total time: 3.693290286s
```

## MESSAGE REVERB PROGRAM
```
A program which reverberates messages

Mutexed VecDeque with 2 threads total time: 300.553196ms
Mutexed LinkedList with 2 threads total time: 625.961536ms
Lockfree Queue with 2 threads total time: 307.219865ms

Mutexed VecDeque with 4 threads total time: 231.928856ms
Mutexed LinkedList with 4 threads total time: 460.332671ms
Lockfree Queue with 4 threads total time: 292.280304ms

Mutexed VecDeque with 8 threads total time: 245.625457ms
Mutexed LinkedList with 8 threads total time: 491.879051ms
Lockfree Queue with 8 threads total time: 249.89504ms

Mutexed VecDeque with 16 threads total time: 243.041973ms
Mutexed LinkedList with 16 threads total time: 502.144057ms
Lockfree Queue with 16 threads total time: 259.789856ms
```

## HASH MINING
```
A program simulating a hash miner.

Mutexed structures with 2 threads total time: 563.11084ms
Lockfree structures with 2 threads total time: 471.075502ms

Mutexed structures with 4 threads total time: 312.280863ms
Lockfree structures with 4 threads total time: 323.87518ms

Mutexed structures with 8 threads total time: 252.670661ms
Lockfree structures with 8 threads total time: 262.011915ms

Mutexed structures with 16 threads total time: 253.702651ms
Lockfree structures with 16 threads total time: 246.719194ms

Mutexed structures with 32 threads total time: 249.77473ms
Lockfree structures with 32 threads total time: 257.833279ms

Mutexed structures with 64 threads total time: 261.56769ms
Lockfree structures with 64 threads total time: 271.032044ms

Mutexed structures with 128 threads total time: 331.997606ms
Lockfree structures with 128 threads total time: 350.377434ms
```

