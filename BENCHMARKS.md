# CPU info
```
Architecture:        x86_64
CPU op-mode(s):      32-bit, 64-bit
Byte Order:          Little Endian
Address sizes:       39 bits physical, 48 bits virtual
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
CPU MHz:             692.336
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
mean of 438336166.767 r/s (547963904 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 137427384.461 r/s (171797504 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 158474785.264 r/s (198123520 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 1212013380.496 r/s (1515177984 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 170857376.010 r/s (213592064 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 269212672.097 r/s (336546816 rounds in 1.250 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 1212124682.465 r/s (1515582464 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 116904407.345 r/s (146167808 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 280176062.284 r/s (350306304 rounds in 1.250 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 1210283372.195 r/s (1513553920 rounds in 1.251 seconds)
Target 1 (blocking):
mean of 83464161.525 r/s (104390656 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 276676508.849 r/s (346000384 rounds in 1.251 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 1212975558.316 r/s (1518582784 rounds in 1.252 seconds)
Target 1 (blocking):
mean of 37182553.883 r/s (46656512 rounds in 1.255 seconds)
Target 2 (lockfree):
mean of 219622124.347 r/s (275011584 rounds in 1.252 seconds)

```

## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 45590622.353 r/s (56998912 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 28905347.102 r/s (36137984 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 15281582.467 r/s (19105792 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 11866588.557 r/s (14834688 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 3111876.116 r/s (3890176 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5246675.392 r/s (6559744 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 8670461.413 r/s (10841088 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 5114930.326 r/s (6394880 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5965375.490 r/s (7459840 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8372918.921 r/s (10471424 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4926897.986 r/s (6162432 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 5677006.170 r/s (7174144 rounds in 1.264 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 8407240.249 r/s (10516480 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4898953.502 r/s (6130688 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 5735051.914 r/s (7301120 rounds in 1.273 seconds)

```

## STACK
```

Result for 1 threads:
Target 0 (mutex vector):
mean of 48685604.396 r/s (60867584 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 27208289.273 r/s (34014208 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 21605100.466 r/s (27009024 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 8843730.323 r/s (11056128 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4433617.579 r/s (5542912 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 6137539.131 r/s (7672832 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 8056017.581 r/s (10072064 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4964556.577 r/s (6208512 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 6770982.325 r/s (8466432 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 7511691.249 r/s (9393152 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4864774.715 r/s (6085632 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 6653101.478 r/s (8560640 rounds in 1.287 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 7464569.606 r/s (9387008 rounds in 1.258 seconds)
Target 1 (mutex linked list):
mean of 4855653.247 r/s (6082560 rounds in 1.253 seconds)
Target 2 (lockfree):
mean of 6579071.481 r/s (8419328 rounds in 1.280 seconds)

```

## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 3219628.192 r/s (4520960 rounds in 1.404 seconds)
Target 1 (lockfree insert):
mean of 2001448.346 r/s (2502656 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 1971336.082 r/s (2465792 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 2467906.566 r/s (3086336 rounds in 1.251 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2218933.725 r/s (2775040 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 3437360.873 r/s (4298752 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2386600.598 r/s (2985984 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 3597114.280 r/s (4661248 rounds in 1.296 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 6207082.918 r/s (7759872 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 4118900.649 r/s (5149696 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 2020195.873 r/s (2526208 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 7101356.631 r/s (8878080 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2202191.737 r/s (2755584 rounds in 1.251 seconds)
Target 1 (lockfree get):
mean of 11136177.468 r/s (13922304 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2295742.261 r/s (2873344 rounds in 1.252 seconds)
Target 1 (lockfree get):
mean of 10676265.398 r/s (13351936 rounds in 1.251 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 19746942.039 r/s (24686592 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 3132242.039 r/s (3915776 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 12129532.446 r/s (15163392 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 7178924.026 r/s (8976384 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10385577.808 r/s (12985344 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 9719687.700 r/s (14257152 rounds in 1.467 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10159512.088 r/s (12703744 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 11376333.469 r/s (14227456 rounds in 1.251 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 4484369.069 r/s (5606400 rounds in 1.250 seconds)
Target 1 (lockfree mixed):
mean of 1534301.535 r/s (1918976 rounds in 1.251 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1491172.592 r/s (1865728 rounds in 1.251 seconds)
Target 1 (lockfree mixed):
mean of 999969.356 r/s (1251328 rounds in 1.251 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1309324.002 r/s (1638400 rounds in 1.251 seconds)
Target 1 (lockfree mixed):
mean of 1709962.811 r/s (2141184 rounds in 1.252 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1397358.071 r/s (1751040 rounds in 1.253 seconds)
Target 1 (lockfree mixed):
mean of 1980455.070 r/s (2794496 rounds in 1.411 seconds)

```

## MPSC CHANNEL
```
Mutexed VecDeque with 3 threads total time: 213.989557ms
Std's MPSC with 3 threads total time: 142.790771ms
Lockfree MPSC with 3 threads total time: 112.616631ms

Mutexed VecDeque with 5 threads total time: 406.039728ms
Std's MPSC with 5 threads total time: 244.310377ms
Lockfree MPSC with 5 threads total time: 198.789775ms

Mutexed VecDeque with 9 threads total time: 782.491586ms
Std's MPSC with 9 threads total time: 497.223562ms
Lockfree MPSC with 9 threads total time: 380.153447ms

Mutexed VecDeque with 17 threads total time: 1.574944957s
Std's MPSC with 17 threads total time: 1.038915292s
Lockfree MPSC with 17 threads total time: 788.963588ms

Mutexed VecDeque with 33 threads total time: 3.002891732s
Std's MPSC with 33 threads total time: 2.05421932s
Lockfree MPSC with 33 threads total time: 1.657342755s
```

## SPSC CHANNEL
```
Mutexed VecDeque total time: 666.34521ms
Std's MPSC (as SPSC) total time: 117.510235ms
Lockfree SPSC total time: 392.270341ms
```

## SPMC CHANNEL
```
Mutexed VecDeque with 3 threads total time: 228.68586ms
Mutexed Std's MPSC (as SPMC) with 3 threads total time: 295.099229ms
Lockfree SPMC with 3 threads total time: 110.277453ms

Mutexed VecDeque with 5 threads total time: 320.900652ms
Mutexed Std's MPSC (as SPMC) with 5 threads total time: 151.060762ms
Lockfree SPMC with 5 threads total time: 93.697871ms

Mutexed VecDeque with 9 threads total time: 671.189792ms
Mutexed Std's MPSC (as SPMC) with 9 threads total time: 159.973235ms
Lockfree SPMC with 9 threads total time: 101.741895ms

Mutexed VecDeque with 17 threads total time: 1.191164497s
Mutexed Std's MPSC (as SPMC) with 17 threads total time: 160.394873ms
Lockfree SPMC with 17 threads total time: 146.836951ms

Mutexed VecDeque with 33 threads total time: 2.329613064s
Mutexed Std's MPSC (as SPMC) with 33 threads total time: 218.34555ms
Lockfree SPMC with 33 threads total time: 486.383693ms
```

## MPMC CHANNEL
```
Mutexed VecDeque with 4 threads total time: 82.272005ms
Mutexed Std's MPSC (as MPMC)  with 4 threads total time: 98.358398ms
Lockfree MPMC with 4 threads total time: 47.500141ms

Mutexed VecDeque with 8 threads total time: 192.371333ms
Mutexed Std's MPSC (as MPMC)  with 8 threads total time: 194.791757ms
Lockfree MPMC with 8 threads total time: 94.161514ms

Mutexed VecDeque with 16 threads total time: 403.483605ms
Mutexed Std's MPSC (as MPMC)  with 16 threads total time: 421.84929ms
Lockfree MPMC with 16 threads total time: 362.751925ms
```

## REQUEST PROGRAM
```
A program simulating a concurrent server.

Mutexed HashMap and Std's MPSC with 2 threads total time: 972.818498ms
Lockfree structures with 2 threads total time: 484.363973ms

Mutexed HashMap and Std's MPSC with 4 threads total time: 370.364597ms
Lockfree structures with 4 threads total time: 394.637043ms

Mutexed HashMap and Std's MPSC with 8 threads total time: 355.236079ms
Lockfree structures with 8 threads total time: 384.019877ms

Mutexed HashMap and Std's MPSC with 16 threads total time: 382.945099ms
Lockfree structures with 16 threads total time: 416.485863ms
```

## MESSAGE REVERB PROGRAM
```
A program which reverberates messages through a plain queue channel

Mutexed VecDeque with 2 threads total time: 209.035328ms
Mutexed LinkedList with 2 threads total time: 604.804ms
Lockfree Queue with 2 threads total time: 197.690289ms

Mutexed VecDeque with 4 threads total time: 217.49984ms
Mutexed LinkedList with 4 threads total time: 449.328914ms
Lockfree Queue with 4 threads total time: 160.041583ms

Mutexed VecDeque with 8 threads total time: 240.474789ms
Mutexed LinkedList with 8 threads total time: 482.899673ms
Lockfree Queue with 8 threads total time: 160.354703ms

Mutexed VecDeque with 16 threads total time: 238.060083ms
Mutexed LinkedList with 16 threads total time: 497.319762ms
Lockfree Queue with 16 threads total time: 155.875498ms
```

## HASH MINING
```
A program simulating a hash miner.

Mutexed structures with 2 threads total time: 470.63554ms
Lockfree structures with 2 threads total time: 468.397783ms

Mutexed structures with 4 threads total time: 300.848644ms
Lockfree structures with 4 threads total time: 311.814368ms

Mutexed structures with 8 threads total time: 250.709601ms
Lockfree structures with 8 threads total time: 253.655119ms

Mutexed structures with 16 threads total time: 244.85033ms
Lockfree structures with 16 threads total time: 255.840211ms

Mutexed structures with 32 threads total time: 261.866771ms
Lockfree structures with 32 threads total time: 250.629204ms

Mutexed structures with 64 threads total time: 261.329732ms
Lockfree structures with 64 threads total time: 273.478164ms

Mutexed structures with 128 threads total time: 328.503614ms
Lockfree structures with 128 threads total time: 342.649508ms
```

