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
CPU MHz:             608.766
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
mean of 503497002.483 r/s (629471232 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 136508523.907 r/s (170661888 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 168584006.073 r/s (210762752 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 1192499423.471 r/s (1490782208 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 127292038.481 r/s (159129600 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 272925880.494 r/s (341188608 rounds in 1.250 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 1233947486.510 r/s (1542950912 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 124743814.055 r/s (155968512 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 274102095.477 r/s (342714368 rounds in 1.250 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 1188764174.766 r/s (1486620672 rounds in 1.251 seconds)
Target 1 (blocking):
mean of 90101289.006 r/s (112686080 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 259842354.996 r/s (324947968 rounds in 1.251 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 1190461788.363 r/s (1490208768 rounds in 1.252 seconds)
Target 1 (blocking):
mean of 37279334.971 r/s (46807040 rounds in 1.256 seconds)
Target 2 (lockfree):
mean of 217821875.934 r/s (272722944 rounds in 1.252 seconds)

```

## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 45486840.251 r/s (56868864 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 28557729.006 r/s (35700736 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 9512226.869 r/s (11892736 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 11836944.398 r/s (14798848 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 3266844.713 r/s (4084736 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5305052.206 r/s (6633472 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 8823645.909 r/s (11032576 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4996524.742 r/s (6248448 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 5294293.447 r/s (6620160 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8382689.412 r/s (10483712 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4923444.037 r/s (6157312 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 4806301.253 r/s (6025216 rounds in 1.254 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 8415216.510 r/s (10531840 rounds in 1.252 seconds)
Target 1 (mutex linked list):
mean of 4884109.175 r/s (6114304 rounds in 1.252 seconds)
Target 2 (lockfree):
mean of 4840694.634 r/s (6062080 rounds in 1.252 seconds)

```

## STACK
```

Result for 1 threads:
Target 0 (mutex vector):
mean of 48040407.434 r/s (60060672 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 27437893.359 r/s (34300928 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 20371248.007 r/s (25466880 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 8759987.057 r/s (10951680 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4477753.974 r/s (5598208 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5795572.729 r/s (7245824 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 7888589.120 r/s (9862144 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 5002548.143 r/s (6257664 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 6735515.965 r/s (8423424 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 7532867.825 r/s (9419776 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4857916.001 r/s (6076416 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 6667589.349 r/s (8363008 rounds in 1.254 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 7535611.647 r/s (9427968 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4844641.986 r/s (6064128 rounds in 1.252 seconds)
Target 2 (lockfree):
mean of 6899481.477 r/s (8687616 rounds in 1.259 seconds)

```

## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 3593678.510 r/s (4494336 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 1907132.554 r/s (2384896 rounds in 1.251 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 1757378.355 r/s (2197504 rounds in 1.250 seconds)
Target 1 (lockfree insert):
mean of 2189142.369 r/s (2738176 rounds in 1.251 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 1817660.638 r/s (2273280 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 3482117.679 r/s (4356096 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 1802484.708 r/s (2256896 rounds in 1.252 seconds)
Target 1 (lockfree insert):
mean of 3455895.720 r/s (4422656 rounds in 1.280 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 4608768.976 r/s (5762048 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 3977119.619 r/s (4972544 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 1797473.173 r/s (2247680 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 7428712.529 r/s (9287680 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2003795.460 r/s (2506752 rounds in 1.251 seconds)
Target 1 (lockfree get):
mean of 11135631.978 r/s (13977600 rounds in 1.255 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 1953665.026 r/s (2447360 rounds in 1.253 seconds)
Target 1 (lockfree get):
mean of 10597581.962 r/s (13257728 rounds in 1.251 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 16302025.119 r/s (20380672 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 1446444.180 r/s (2042880 rounds in 1.412 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 11969750.782 r/s (14964736 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 5357519.210 r/s (6699008 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10612924.599 r/s (13268992 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 10359427.614 r/s (12951552 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10451119.559 r/s (13069312 rounds in 1.251 seconds)
Target 1 (lockfree remove):
mean of 10683475.369 r/s (13361152 rounds in 1.251 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 4034952.016 r/s (5045248 rounds in 1.250 seconds)
Target 1 (lockfree mixed):
mean of 1570475.499 r/s (1964032 rounds in 1.251 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1101281.067 r/s (1377280 rounds in 1.251 seconds)
Target 1 (lockfree mixed):
mean of 1080378.232 r/s (1351680 rounds in 1.251 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1011039.493 r/s (1265664 rounds in 1.252 seconds)
Target 1 (lockfree mixed):
mean of 1742219.979 r/s (2181120 rounds in 1.252 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1008087.833 r/s (1264640 rounds in 1.254 seconds)
Target 1 (lockfree mixed):
mean of 1584794.274 r/s (2111488 rounds in 1.332 seconds)

```

## MPSC CHANNEL
```
Result for 2 threads:
Target 0 (mutex vector):
mean of 9770549.272 r/s (12214272 rounds in 1.250 seconds)
Target 1 (std's mpsc):
mean of 5644312.646 r/s (7057408 rounds in 1.250 seconds)
Target 2 (lockfree mpsc):
mean of 5771709.383 r/s (7216128 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 7335022.022 r/s (9170944 rounds in 1.250 seconds)
Target 1 (std's mpsc):
mean of 7889744.726 r/s (9866240 rounds in 1.251 seconds)
Target 2 (lockfree mpsc):
mean of 6745209.431 r/s (8434688 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 6883893.832 r/s (8609792 rounds in 1.251 seconds)
Target 1 (std's mpsc):
mean of 6390760.634 r/s (7993344 rounds in 1.251 seconds)
Target 2 (lockfree mpsc):
mean of 5847470.061 r/s (7315456 rounds in 1.251 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 6860900.321 r/s (8581120 rounds in 1.251 seconds)
Target 1 (std's mpsc):
mean of 6102847.040 r/s (7639040 rounds in 1.252 seconds)
Target 2 (lockfree mpsc):
mean of 5521186.502 r/s (6912000 rounds in 1.252 seconds)

```

## SPSC CHANNEL
```
Mutexed VecDeque total time: 133.10366ms
Lockfree SPSC total time: 85.696507ms
```

## REQUEST PROGRAM
```
A program simulating a concurrent server.

Mutexed HashMap and VecDeque with 2 threads total time: 1.04244306s
Mutexed HashMap and LinkedList with 2 threads total time: 1.136163127s
Lockfree structures with 2 threads total time: 774.618249ms

Mutexed HashMap and VecDeque with 4 threads total time: 623.609313ms
Mutexed HashMap and LinkedList with 4 threads total time: 2.100949122s
Lockfree structures with 4 threads total time: 705.055226ms

Mutexed HashMap and VecDeque with 8 threads total time: 1.076987166s
Mutexed HashMap and LinkedList with 8 threads total time: 2.636296555s
Lockfree structures with 8 threads total time: 877.769612ms

Mutexed HashMap and VecDeque with 16 threads total time: 1.87311725s
Mutexed HashMap and LinkedList with 16 threads total time: 4.797116504s
Lockfree structures with 16 threads total time: 1.799811074s
```

## MESSAGE REVERB PROGRAM
```
A program which reverberates messages

Mutexed VecDeque with 2 threads total time: 212.787759ms
Mutexed LinkedList with 2 threads total time: 614.707797ms
Lockfree Queue with 2 threads total time: 214.779154ms

Mutexed VecDeque with 4 threads total time: 236.599081ms
Mutexed LinkedList with 4 threads total time: 457.631675ms
Lockfree Queue with 4 threads total time: 187.800223ms

Mutexed VecDeque with 8 threads total time: 244.926809ms
Mutexed LinkedList with 8 threads total time: 493.636563ms
Lockfree Queue with 8 threads total time: 183.437795ms

Mutexed VecDeque with 16 threads total time: 252.598345ms
Mutexed LinkedList with 16 threads total time: 518.491929ms
Lockfree Queue with 16 threads total time: 189.235722ms
```

## HASH MINING
```
A program simulating a hash miner.

Mutexed structures with 2 threads total time: 482.457434ms
Lockfree structures with 2 threads total time: 489.235446ms

Mutexed structures with 4 threads total time: 304.668723ms
Lockfree structures with 4 threads total time: 305.429379ms

Mutexed structures with 8 threads total time: 290.020015ms
Lockfree structures with 8 threads total time: 276.500056ms

Mutexed structures with 16 threads total time: 316.41702ms
Lockfree structures with 16 threads total time: 307.531951ms

Mutexed structures with 32 threads total time: 323.904827ms
Lockfree structures with 32 threads total time: 315.304326ms

Mutexed structures with 64 threads total time: 294.959918ms
Lockfree structures with 64 threads total time: 314.804105ms

Mutexed structures with 128 threads total time: 339.862853ms
Lockfree structures with 128 threads total time: 346.931204ms
```

