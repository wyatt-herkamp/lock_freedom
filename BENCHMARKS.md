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
CPU MHz:             600.051
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
mean of 549941399.757 r/s (687546368 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 138294840.108 r/s (172882944 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 167825542.268 r/s (209803264 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 1252218936.423 r/s (1565463552 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 167738833.341 r/s (209693696 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 273414679.029 r/s (341815296 rounds in 1.250 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 1253050453.402 r/s (1566669824 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 116727311.432 r/s (145945600 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 279636619.451 r/s (349632512 rounds in 1.250 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 1248836948.984 r/s (1561672704 rounds in 1.251 seconds)
Target 1 (blocking):
mean of 85896394.461 r/s (107427840 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 279364708.247 r/s (349359104 rounds in 1.251 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 1249199977.555 r/s (1563990016 rounds in 1.252 seconds)
Target 1 (blocking):
mean of 37125556.201 r/s (46578688 rounds in 1.255 seconds)
Target 2 (lockfree):
mean of 210385929.081 r/s (263420928 rounds in 1.252 seconds)

```

## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 45686099.307 r/s (57117696 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 28928398.589 r/s (36163584 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 12394584.281 r/s (15496192 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 11088961.222 r/s (13862912 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 2972068.685 r/s (3716096 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 4773430.730 r/s (5967872 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 8660123.796 r/s (10827776 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4950992.578 r/s (6191104 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5348164.374 r/s (6687744 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8193007.548 r/s (10246144 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4905505.769 r/s (6134784 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 5386880.593 r/s (6747136 rounds in 1.253 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 8179184.487 r/s (10232832 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4884361.237 r/s (6113280 rounds in 1.252 seconds)
Target 2 (lockfree):
mean of 5452421.650 r/s (6899712 rounds in 1.265 seconds)

```

## STACK
```

Result for 1 threads:
Target 0 (mutex vector):
mean of 48646091.149 r/s (60816384 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 27644720.597 r/s (34558976 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 20613653.292 r/s (25768960 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 8410186.211 r/s (10514432 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4096945.390 r/s (5122048 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5868255.722 r/s (7336960 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 7800745.588 r/s (9752576 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 5065996.364 r/s (6333440 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 6767737.163 r/s (8462336 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 7524269.394 r/s (9410560 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4960335.880 r/s (6206464 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 6794592.479 r/s (8512512 rounds in 1.253 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 7507670.227 r/s (9392128 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4976901.371 r/s (6228992 rounds in 1.252 seconds)
Target 2 (lockfree):
mean of 6853541.088 r/s (8590336 rounds in 1.253 seconds)

```

## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 3268937.072 r/s (4520960 rounds in 1.383 seconds)
Target 1 (lockfree insert):
mean of 1936663.454 r/s (2421760 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 2003999.977 r/s (2506752 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 2249886.861 r/s (2813952 rounds in 1.251 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2132442.376 r/s (2668544 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 3591954.604 r/s (4492288 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2326343.253 r/s (2913280 rounds in 1.252 seconds)
Target 1 (lockfree insert):
mean of 3555381.872 r/s (4461568 rounds in 1.255 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 6501554.821 r/s (8128512 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 4468252.040 r/s (5586944 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 2056757.000 r/s (2572288 rounds in 1.251 seconds)
Target 1 (lockfree get):
mean of 7169456.091 r/s (8964096 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2301268.845 r/s (2880512 rounds in 1.252 seconds)
Target 1 (lockfree get):
mean of 11389597.387 r/s (14240768 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2357699.532 r/s (2952192 rounds in 1.252 seconds)
Target 1 (lockfree get):
mean of 10731206.678 r/s (13474816 rounds in 1.256 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 20964076.089 r/s (26207232 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 1452026.672 r/s (2044928 rounds in 1.408 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 12222653.486 r/s (15281152 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 5208585.776 r/s (6512640 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10599364.707 r/s (13251584 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 11614675.212 r/s (14522368 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10526915.837 r/s (13161472 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 10968238.220 r/s (13716480 rounds in 1.251 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 4511723.688 r/s (5641216 rounds in 1.250 seconds)
Target 1 (lockfree mixed):
mean of 1623010.810 r/s (2029568 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1491570.381 r/s (1864704 rounds in 1.250 seconds)
Target 1 (lockfree mixed):
mean of 1066464.427 r/s (1335296 rounds in 1.252 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1294519.253 r/s (1619968 rounds in 1.251 seconds)
Target 1 (lockfree mixed):
mean of 1660836.138 r/s (2078720 rounds in 1.252 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1388480.684 r/s (1739776 rounds in 1.253 seconds)
Target 1 (lockfree mixed):
mean of 1677318.132 r/s (2258944 rounds in 1.347 seconds)

```

## MPSC CHANNEL
```
Mutexed VecDeque with 3 threads total time: 194.180677ms
Std's MPSC with 3 threads total time: 159.488215ms
Lockfree MPSC with 3 threads total time: 114.443154ms

Mutexed VecDeque with 5 threads total time: 424.234551ms
Std's MPSC with 5 threads total time: 252.423314ms
Lockfree MPSC with 5 threads total time: 204.853095ms

Mutexed VecDeque with 9 threads total time: 792.618245ms
Std's MPSC with 9 threads total time: 492.884109ms
Lockfree MPSC with 9 threads total time: 385.799289ms

Mutexed VecDeque with 17 threads total time: 1.558790283s
Std's MPSC with 17 threads total time: 1.030869813s
Lockfree MPSC with 17 threads total time: 794.18748ms

Mutexed VecDeque with 33 threads total time: 3.090941338s
Std's MPSC with 33 threads total time: 2.06700007s
Lockfree MPSC with 33 threads total time: 1.612666222s
```

## SPSC CHANNEL
```
Mutexed VecDeque total time: 521.597649ms
Lockfree SPSC total time: 337.320329ms
```

## SPMC CHANNEL
```
Mutexed VecDeque with 3 threads total time: 226.188315ms
Mutexed Std's MPSC (as SPMC) with 3 threads total time: 262.541913ms
Lockfree SPMC with 3 threads total time: 124.024442ms

Mutexed VecDeque with 5 threads total time: 364.920775ms
Mutexed Std's MPSC (as SPMC) with 5 threads total time: 157.159575ms
Lockfree SPMC with 5 threads total time: 129.068428ms

Mutexed VecDeque with 9 threads total time: 654.816801ms
Mutexed Std's MPSC (as SPMC) with 9 threads total time: 162.349709ms
Lockfree SPMC with 9 threads total time: 127.348729ms

Mutexed VecDeque with 17 threads total time: 1.062371642s
Mutexed Std's MPSC (as SPMC) with 17 threads total time: 159.003883ms
Lockfree SPMC with 17 threads total time: 197.730542ms

Mutexed VecDeque with 33 threads total time: 2.405959805s
Mutexed Std's MPSC (as SPMC) with 33 threads total time: 207.252902ms
Lockfree SPMC with 33 threads total time: 568.129691ms
```

## REQUEST PROGRAM
```
A program simulating a concurrent server.

Mutexed HashMap and VecDeque with 2 threads total time: 1.308835775s
Mutexed HashMap and LinkedList with 2 threads total time: 1.187644291s
Lockfree structures with 2 threads total time: 703.142442ms

Mutexed HashMap and VecDeque with 4 threads total time: 844.980004ms
Mutexed HashMap and LinkedList with 4 threads total time: 1.842453933s
Lockfree structures with 4 threads total time: 728.604683ms

Mutexed HashMap and VecDeque with 8 threads total time: 1.025154764s
Mutexed HashMap and LinkedList with 8 threads total time: 2.286303646s
Lockfree structures with 8 threads total time: 800.880899ms

Mutexed HashMap and VecDeque with 16 threads total time: 2.306755917s
Mutexed HashMap and LinkedList with 16 threads total time: 4.650002814s
Lockfree structures with 16 threads total time: 2.162405977s
```

## MESSAGE REVERB PROGRAM
```
A program which reverberates messages through a plain queue channel

Mutexed VecDeque with 2 threads total time: 198.331528ms
Mutexed LinkedList with 2 threads total time: 560.449725ms
Lockfree Queue with 2 threads total time: 199.096649ms

Mutexed VecDeque with 4 threads total time: 235.982417ms
Mutexed LinkedList with 4 threads total time: 443.325976ms
Lockfree Queue with 4 threads total time: 191.422978ms

Mutexed VecDeque with 8 threads total time: 240.597703ms
Mutexed LinkedList with 8 threads total time: 477.775671ms
Lockfree Queue with 8 threads total time: 182.608751ms

Mutexed VecDeque with 16 threads total time: 239.03444ms
Mutexed LinkedList with 16 threads total time: 489.985508ms
Lockfree Queue with 16 threads total time: 171.839714ms
```

## HASH MINING
```
A program simulating a hash miner.

Mutexed structures with 2 threads total time: 467.744585ms
Lockfree structures with 2 threads total time: 464.594466ms

Mutexed structures with 4 threads total time: 295.022038ms
Lockfree structures with 4 threads total time: 316.589134ms

Mutexed structures with 8 threads total time: 254.72871ms
Lockfree structures with 8 threads total time: 258.949557ms

Mutexed structures with 16 threads total time: 249.30597ms
Lockfree structures with 16 threads total time: 243.492016ms

Mutexed structures with 32 threads total time: 250.558026ms
Lockfree structures with 32 threads total time: 262.093357ms

Mutexed structures with 64 threads total time: 259.909881ms
Lockfree structures with 64 threads total time: 269.380561ms

Mutexed structures with 128 threads total time: 330.512394ms
Lockfree structures with 128 threads total time: 344.367923ms
```

