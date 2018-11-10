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
CPU MHz:             2879.778
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
mean of 551038946.434 r/s (688855040 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 137460323.793 r/s (171836416 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 166730350.648 r/s (208444416 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 1179844901.052 r/s (1474995200 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 199501097.032 r/s (249400320 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 281054236.432 r/s (351350784 rounds in 1.250 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 1264434845.307 r/s (1581046784 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 125619686.000 r/s (157066240 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 281072059.933 r/s (351425536 rounds in 1.250 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 1252023926.988 r/s (1565717504 rounds in 1.251 seconds)
Target 1 (blocking):
mean of 90130577.642 r/s (112727040 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 280763617.037 r/s (351090688 rounds in 1.250 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 1248642835.371 r/s (1563399168 rounds in 1.252 seconds)
Target 1 (blocking):
mean of 37614694.521 r/s (47205376 rounds in 1.255 seconds)
Target 2 (lockfree):
mean of 220380548.773 r/s (275982336 rounds in 1.252 seconds)

```

## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 45941353.484 r/s (57436160 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 29230898.448 r/s (36542464 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 15541483.468 r/s (19430400 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 11166144.536 r/s (13959168 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 3055823.539 r/s (3820544 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5276752.442 r/s (6597632 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 8601453.816 r/s (10755072 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4914263.283 r/s (6145024 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 6104323.479 r/s (7633920 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8245343.852 r/s (10310656 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4835903.853 r/s (6050816 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 5683900.898 r/s (7183360 rounds in 1.264 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 8267511.345 r/s (10340352 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4853813.523 r/s (6073344 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 5681695.127 r/s (7201792 rounds in 1.268 seconds)

```

## STACK
```

Result for 1 threads:
Target 0 (mutex vector):
mean of 48211333.273 r/s (60275712 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 27694297.369 r/s (34624512 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 21501584.491 r/s (26882048 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 8204450.791 r/s (10256384 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4420645.202 r/s (5526528 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 6154866.866 r/s (7695360 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 7906296.712 r/s (9884672 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 5131875.789 r/s (6417408 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 6715705.588 r/s (8397824 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 7550664.002 r/s (9442304 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4980905.855 r/s (6231040 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 6740757.726 r/s (8615936 rounds in 1.278 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 7519137.562 r/s (9405440 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4966799.241 r/s (6216704 rounds in 1.252 seconds)
Target 2 (lockfree):
mean of 6668105.336 r/s (8519680 rounds in 1.278 seconds)

```

## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 3269983.193 r/s (4520960 rounds in 1.383 seconds)
Target 1 (lockfree insert):
mean of 2024727.134 r/s (2531328 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 1987896.782 r/s (2485248 rounds in 1.250 seconds)
Target 1 (lockfree insert):
mean of 2520597.907 r/s (3154944 rounds in 1.252 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2079941.045 r/s (2601984 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 3468769.942 r/s (4338688 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2278793.277 r/s (2853888 rounds in 1.252 seconds)
Target 1 (lockfree insert):
mean of 3628822.083 r/s (4659200 rounds in 1.284 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 6436152.511 r/s (8047616 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 4319907.589 r/s (5400576 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 2034670.521 r/s (2543616 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 7024827.196 r/s (8781824 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2316877.950 r/s (2897920 rounds in 1.251 seconds)
Target 1 (lockfree get):
mean of 10918045.644 r/s (13651968 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2386718.941 r/s (2988032 rounds in 1.252 seconds)
Target 1 (lockfree get):
mean of 10609925.422 r/s (13329408 rounds in 1.256 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 20886133.303 r/s (26112000 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 2814385.778 r/s (4080640 rounds in 1.450 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 12093551.888 r/s (15118336 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 7214720.114 r/s (9019392 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10594411.076 r/s (13246464 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 11131573.699 r/s (13918208 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10562510.195 r/s (13208576 rounds in 1.251 seconds)
Target 1 (lockfree remove):
mean of 11222261.328 r/s (14033920 rounds in 1.251 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 4493885.780 r/s (5618688 rounds in 1.250 seconds)
Target 1 (lockfree mixed):
mean of 1788337.080 r/s (2236416 rounds in 1.251 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1493581.197 r/s (1891328 rounds in 1.266 seconds)
Target 1 (lockfree mixed):
mean of 1080870.914 r/s (1353728 rounds in 1.252 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1293207.487 r/s (1617920 rounds in 1.251 seconds)
Target 1 (lockfree mixed):
mean of 1709497.397 r/s (2140160 rounds in 1.252 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1393518.052 r/s (1745920 rounds in 1.253 seconds)
Target 1 (lockfree mixed):
mean of 2014529.479 r/s (2734080 rounds in 1.357 seconds)

```

## MPSC CHANNEL
```
Mutexed VecDeque with 3 threads total time: 198.106502ms
Std's MPSC with 3 threads total time: 149.801294ms
Lockfree MPSC with 3 threads total time: 113.538334ms

Mutexed VecDeque with 5 threads total time: 433.67735ms
Std's MPSC with 5 threads total time: 252.353575ms
Lockfree MPSC with 5 threads total time: 195.544918ms

Mutexed VecDeque with 9 threads total time: 841.790683ms
Std's MPSC with 9 threads total time: 491.176214ms
Lockfree MPSC with 9 threads total time: 387.042572ms

Mutexed VecDeque with 17 threads total time: 1.653781937s
Std's MPSC with 17 threads total time: 1.006511124s
Lockfree MPSC with 17 threads total time: 749.495519ms

Mutexed VecDeque with 33 threads total time: 3.248423073s
Std's MPSC with 33 threads total time: 2.026259666s
Lockfree MPSC with 33 threads total time: 1.570055475s
```

## SPSC CHANNEL
```
Mutexed VecDeque total time: 619.285969ms
Lockfree SPSC total time: 390.903281ms
```

## SPMC CHANNEL
```
Mutexed VecDeque with 3 threads total time: 230.746543ms
Mutexed Std's MPSC (as SPMC) with 3 threads total time: 269.891147ms
Lockfree SPMC with 3 threads total time: 113.226344ms

Mutexed VecDeque with 5 threads total time: 406.515659ms
Mutexed Std's MPSC (as SPMC) with 5 threads total time: 160.164525ms
Lockfree SPMC with 5 threads total time: 94.903038ms

Mutexed VecDeque with 9 threads total time: 653.91891ms
Mutexed Std's MPSC (as SPMC) with 9 threads total time: 156.964223ms
Lockfree SPMC with 9 threads total time: 94.818746ms

Mutexed VecDeque with 17 threads total time: 1.186609781s
Mutexed Std's MPSC (as SPMC) with 17 threads total time: 155.874604ms
Lockfree SPMC with 17 threads total time: 242.17052ms

Mutexed VecDeque with 33 threads total time: 2.467442422s
Mutexed Std's MPSC (as SPMC) with 33 threads total time: 180.388409ms
Lockfree SPMC with 33 threads total time: 525.326388ms
```

## MPMC CHANNEL
```
Mutexed VecDeque with 4 threads total time: 84.21336ms
Mutexed Std's MPSC (as MPMC)  with 4 threads total time: 142.701664ms
Lockfree MPMC with 4 threads total time: 47.685639ms

Mutexed VecDeque with 8 threads total time: 182.338484ms
Mutexed Std's MPSC (as MPMC)  with 8 threads total time: 190.055618ms
Lockfree MPMC with 8 threads total time: 96.749202ms

Mutexed VecDeque with 16 threads total time: 382.163374ms
Mutexed Std's MPSC (as MPMC)  with 16 threads total time: 414.182347ms
Lockfree MPMC with 16 threads total time: 340.021299ms
```

## REQUEST PROGRAM
```
A program simulating a concurrent server.

Mutexed HashMap and VecDeque with 2 threads total time: 1.283410863s
Mutexed HashMap and LinkedList with 2 threads total time: 1.134299832s
Lockfree structures with 2 threads total time: 600.173656ms

Mutexed HashMap and VecDeque with 4 threads total time: 768.733059ms
Mutexed HashMap and LinkedList with 4 threads total time: 1.776658514s
Lockfree structures with 4 threads total time: 472.288832ms

Mutexed HashMap and VecDeque with 8 threads total time: 842.392831ms
Mutexed HashMap and LinkedList with 8 threads total time: 2.463649075s
Lockfree structures with 8 threads total time: 763.356276ms

Mutexed HashMap and VecDeque with 16 threads total time: 2.130563165s
Mutexed HashMap and LinkedList with 16 threads total time: 3.687384574s
Lockfree structures with 16 threads total time: 1.771189867s
```

## MESSAGE REVERB PROGRAM
```
A program which reverberates messages through a plain queue channel

Mutexed VecDeque with 2 threads total time: 194.002854ms
Mutexed LinkedList with 2 threads total time: 613.208912ms
Lockfree Queue with 2 threads total time: 204.005115ms

Mutexed VecDeque with 4 threads total time: 228.898782ms
Mutexed LinkedList with 4 threads total time: 449.747119ms
Lockfree Queue with 4 threads total time: 163.172391ms

Mutexed VecDeque with 8 threads total time: 245.707204ms
Mutexed LinkedList with 8 threads total time: 468.791247ms
Lockfree Queue with 8 threads total time: 149.317099ms

Mutexed VecDeque with 16 threads total time: 250.328974ms
Mutexed LinkedList with 16 threads total time: 481.238139ms
Lockfree Queue with 16 threads total time: 155.465321ms
```

## HASH MINING
```
A program simulating a hash miner.

Mutexed structures with 2 threads total time: 458.126585ms
Lockfree structures with 2 threads total time: 463.985156ms

Mutexed structures with 4 threads total time: 303.309338ms
Lockfree structures with 4 threads total time: 303.315052ms

Mutexed structures with 8 threads total time: 248.746389ms
Lockfree structures with 8 threads total time: 247.93606ms

Mutexed structures with 16 threads total time: 246.22988ms
Lockfree structures with 16 threads total time: 245.460501ms

Mutexed structures with 32 threads total time: 251.06499ms
Lockfree structures with 32 threads total time: 250.031345ms

Mutexed structures with 64 threads total time: 259.528016ms
Lockfree structures with 64 threads total time: 269.426201ms

Mutexed structures with 128 threads total time: 330.359664ms
Lockfree structures with 128 threads total time: 347.360986ms
```

