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
CPU MHz:             2537.544
CPU max MHz:         3100.0000
CPU min MHz:         400.0000
BogoMIPS:            5426.00
Virtualization:      VT-x
L1d cache:           32K
L1i cache:           32K
L2 cache:            256K
L3 cache:            3072K
NUMA node0 CPU(s):   0-3
Flags:               fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx pdpe1gb rdtscp lm constant_tsc art arch_perfmon pebs bts rep_good nopl xtopology nonstop_tsc cpuid aperfmperf tsc_known_freq pni pclmulqdq dtes64 monitor ds_cpl vmx est tm2 ssse3 sdbg fma cx16 xtpr pdcm pcid sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave avx f16c rdrand lahf_lm abm 3dnowprefetch cpuid_fault epb invpcid_single pti ssbd ibrs ibpb stibp tpr_shadow vnmi flexpriority ept vpid ept_ad fsgsbase tsc_adjust bmi1 avx2 smep bmi2 erms invpcid mpx rdseed adx smap clflushopt intel_pt xsaveopt xsavec xgetbv1 xsaves dtherm ida arat pln pts hwp hwp_notify hwp_act_window hwp_epp flush_l1d
```
Running on Linux 4.19.1-arch1-1-ARCH #1 SMP PREEMPT Sun Nov 4 16:49:26 UTC 2018 x86_64 GNU/Linux

# Benchmarks
Benchmark code under [benchmark](benchmark) directory.
More rounds per seconds is better.

As you can see, there is a lot to improve!


## THREAD-LOCAL STORAGE
```
Result for 1 threads:
Target 0 (std/global):
mean of 439694851.879 r/s (549701632 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 127601994.001 r/s (159513600 rounds in 1.250 seconds)
Target 2 (blocking with cached access):
mean of 314439070.559 r/s (393110528 rounds in 1.250 seconds)
Target 3 (lockfree):
mean of 144533817.072 r/s (180692992 rounds in 1.250 seconds)
Target 4 (lockfree with cached id):
mean of 192086023.080 r/s (240143360 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 1222160449.130 r/s (1527842816 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 197805416.433 r/s (247277568 rounds in 1.250 seconds)
Target 2 (blocking with cached access):
mean of 341557380.276 r/s (426981376 rounds in 1.250 seconds)
Target 3 (lockfree):
mean of 292982151.962 r/s (366255104 rounds in 1.250 seconds)
Target 4 (lockfree with cached id):
mean of 406548068.451 r/s (508222464 rounds in 1.250 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 1005018791.564 r/s (1256529920 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 125354446.252 r/s (156727296 rounds in 1.250 seconds)
Target 2 (blocking with cached access):
mean of 181159936.730 r/s (226498560 rounds in 1.250 seconds)
Target 3 (lockfree):
mean of 292993993.579 r/s (366305280 rounds in 1.250 seconds)
Target 4 (lockfree with cached id):
mean of 401067223.585 r/s (501451776 rounds in 1.250 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 987346882.457 r/s (1234705408 rounds in 1.251 seconds)
Target 1 (blocking):
mean of 104253940.930 r/s (130371584 rounds in 1.251 seconds)
Target 2 (blocking with cached access):
mean of 138927404.552 r/s (173749248 rounds in 1.251 seconds)
Target 3 (lockfree):
mean of 292939515.045 r/s (366307328 rounds in 1.250 seconds)
Target 4 (lockfree with cached id):
mean of 386659113.883 r/s (483523584 rounds in 1.251 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 987193581.666 r/s (1235953664 rounds in 1.252 seconds)
Target 1 (blocking):
mean of 52544802.709 r/s (65835008 rounds in 1.253 seconds)
Target 2 (blocking with cached access):
mean of 77650906.252 r/s (97311744 rounds in 1.253 seconds)
Target 3 (lockfree):
mean of 292893498.232 r/s (366617600 rounds in 1.252 seconds)
Target 4 (lockfree with cached id):
mean of 370094020.047 r/s (463286272 rounds in 1.252 seconds)

```

## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 37093774.971 r/s (46374912 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 29320495.999 r/s (36656128 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 15519767.989 r/s (19402752 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 12935023.167 r/s (16171008 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 3402312.677 r/s (4253696 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5339125.884 r/s (6675456 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 8702212.454 r/s (10880000 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4501034.392 r/s (5627904 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 4835418.368 r/s (6045696 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8075547.439 r/s (10099712 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4660968.434 r/s (5828608 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 4699503.558 r/s (5888000 rounds in 1.253 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 7680608.292 r/s (9609216 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4859263.141 r/s (6083584 rounds in 1.252 seconds)
Target 2 (lockfree):
mean of 4821584.344 r/s (6056960 rounds in 1.256 seconds)

```

## STACK
```

Result for 1 threads:
Target 0 (mutex vector):
mean of 43973403.477 r/s (54976512 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 29523689.251 r/s (36911104 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 20598593.673 r/s (25750528 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 8117065.765 r/s (10147840 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 3491986.231 r/s (4366336 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 6046861.294 r/s (7559168 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 7806153.341 r/s (9759744 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4858199.457 r/s (6074368 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 6525058.409 r/s (8159232 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 7576704.042 r/s (9475072 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 3988235.318 r/s (4988928 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 5505099.175 r/s (6907904 rounds in 1.255 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 7550467.304 r/s (9444352 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4172579.119 r/s (5222400 rounds in 1.252 seconds)
Target 2 (lockfree):
mean of 5166852.378 r/s (6648832 rounds in 1.287 seconds)

```

## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 3240939.024 r/s (4520960 rounds in 1.395 seconds)
Target 1 (lockfree insert):
mean of 2023899.519 r/s (2531328 rounds in 1.251 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 2009133.608 r/s (2511872 rounds in 1.250 seconds)
Target 1 (lockfree insert):
mean of 2514822.029 r/s (3145728 rounds in 1.251 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2107826.778 r/s (2637824 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 3152136.327 r/s (3944448 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2400519.283 r/s (3005440 rounds in 1.252 seconds)
Target 1 (lockfree insert):
mean of 3773559.828 r/s (4775936 rounds in 1.266 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 6536142.187 r/s (8172544 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 4710204.840 r/s (5889024 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 2053893.318 r/s (2568192 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 7974205.766 r/s (9969664 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2202964.486 r/s (2755584 rounds in 1.251 seconds)
Target 1 (lockfree get):
mean of 9859591.973 r/s (12328960 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2537600.703 r/s (3175424 rounds in 1.251 seconds)
Target 1 (lockfree get):
mean of 12139675.155 r/s (15184896 rounds in 1.251 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 21181259.050 r/s (26480640 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 2851263.140 r/s (4281344 rounds in 1.502 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 12285040.231 r/s (15357952 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 9533361.344 r/s (11919360 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10775773.299 r/s (13472768 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 12944432.926 r/s (16184320 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10744492.646 r/s (13435904 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 13716786.495 r/s (17153024 rounds in 1.251 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 4543358.888 r/s (5680128 rounds in 1.250 seconds)
Target 1 (lockfree mixed):
mean of 2066888.748 r/s (2584576 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1551175.696 r/s (1940480 rounds in 1.251 seconds)
Target 1 (lockfree mixed):
mean of 1274032.781 r/s (1593344 rounds in 1.251 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1172311.162 r/s (1466368 rounds in 1.251 seconds)
Target 1 (lockfree mixed):
mean of 1578745.788 r/s (1976320 rounds in 1.252 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1403952.139 r/s (1758208 rounds in 1.252 seconds)
Target 1 (lockfree mixed):
mean of 1802034.004 r/s (2263040 rounds in 1.256 seconds)

```

## MPSC CHANNEL
```
Mutexed VecDeque with 3 threads total time: 193.717732ms
Std's MPSC with 3 threads total time: 154.818302ms
Lockfree MPSC with 3 threads total time: 113.282819ms

Mutexed VecDeque with 5 threads total time: 427.977927ms
Std's MPSC with 5 threads total time: 249.697637ms
Lockfree MPSC with 5 threads total time: 194.820021ms

Mutexed VecDeque with 9 threads total time: 799.947835ms
Std's MPSC with 9 threads total time: 485.332648ms
Lockfree MPSC with 9 threads total time: 359.27469ms

Mutexed VecDeque with 17 threads total time: 1.577752555s
Std's MPSC with 17 threads total time: 963.690757ms
Lockfree MPSC with 17 threads total time: 766.104542ms

Mutexed VecDeque with 33 threads total time: 3.156558927s
Std's MPSC with 33 threads total time: 1.954219541s
Lockfree MPSC with 33 threads total time: 1.543029925s
```

## SPSC CHANNEL
```
Mutexed VecDeque total time: 600.244559ms
Std's MPSC (as SPSC) total time: 86.180935ms
Lockfree SPSC total time: 369.088217ms
```

## SPMC CHANNEL
```
Mutexed VecDeque with 3 threads total time: 237.96873ms
Mutexed Std's MPSC (as SPMC) with 3 threads total time: 259.893993ms
Lockfree SPMC with 3 threads total time: 120.591703ms

Mutexed VecDeque with 5 threads total time: 373.803155ms
Mutexed Std's MPSC (as SPMC) with 5 threads total time: 158.954184ms
Lockfree SPMC with 5 threads total time: 119.680745ms

Mutexed VecDeque with 9 threads total time: 702.703564ms
Mutexed Std's MPSC (as SPMC) with 9 threads total time: 172.479452ms
Lockfree SPMC with 9 threads total time: 107.895003ms

Mutexed VecDeque with 17 threads total time: 1.357197227s
Mutexed Std's MPSC (as SPMC) with 17 threads total time: 206.995845ms
Lockfree SPMC with 17 threads total time: 305.180432ms

Mutexed VecDeque with 33 threads total time: 2.386541853s
Mutexed Std's MPSC (as SPMC) with 33 threads total time: 222.811347ms
Lockfree SPMC with 33 threads total time: 690.688504ms
```

## MPMC CHANNEL
```
Mutexed VecDeque with 4 threads total time: 104.98589ms
Mutexed Std's MPSC (as MPMC)  with 4 threads total time: 181.621319ms
Lockfree MPMC with 4 threads total time: 65.918396ms

Mutexed VecDeque with 8 threads total time: 226.572447ms
Mutexed Std's MPSC (as MPMC)  with 8 threads total time: 245.975143ms
Lockfree MPMC with 8 threads total time: 142.854197ms

Mutexed VecDeque with 16 threads total time: 375.368896ms
Mutexed Std's MPSC (as MPMC)  with 16 threads total time: 416.156079ms
Lockfree MPMC with 16 threads total time: 306.302107ms
```

## REQUEST PROGRAM
```
A program simulating a concurrent server.

Mutexed HashMap and Std's MPSC with 2 threads total time: 841.481869ms
Lockfree structures with 2 threads total time: 479.010452ms

Mutexed HashMap and Std's MPSC with 4 threads total time: 421.174654ms
Lockfree structures with 4 threads total time: 440.204058ms

Mutexed HashMap and Std's MPSC with 8 threads total time: 413.581221ms
Lockfree structures with 8 threads total time: 471.595807ms

Mutexed HashMap and Std's MPSC with 16 threads total time: 387.942711ms
Lockfree structures with 16 threads total time: 460.72267ms
```

## MESSAGE REVERB PROGRAM
```
A program which reverberates messages through a plain queue channel

Mutexed VecDeque with 2 threads total time: 210.932019ms
Mutexed LinkedList with 2 threads total time: 606.480712ms
Lockfree Queue with 2 threads total time: 203.519744ms

Mutexed VecDeque with 4 threads total time: 236.426856ms
Mutexed LinkedList with 4 threads total time: 446.870642ms
Lockfree Queue with 4 threads total time: 164.237813ms

Mutexed VecDeque with 8 threads total time: 243.018704ms
Mutexed LinkedList with 8 threads total time: 482.348473ms
Lockfree Queue with 8 threads total time: 149.421492ms

Mutexed VecDeque with 16 threads total time: 298.417683ms
Mutexed LinkedList with 16 threads total time: 613.642776ms
Lockfree Queue with 16 threads total time: 186.038338ms
```

## HASH MINING
```
A program simulating a hash miner.

Mutexed structures with 2 threads total time: 552.075626ms
Lockfree structures with 2 threads total time: 540.977078ms

Mutexed structures with 4 threads total time: 309.57107ms
Lockfree structures with 4 threads total time: 307.912658ms

Mutexed structures with 8 threads total time: 260.045722ms
Lockfree structures with 8 threads total time: 244.678767ms

Mutexed structures with 16 threads total time: 248.233044ms
Lockfree structures with 16 threads total time: 242.869477ms

Mutexed structures with 32 threads total time: 302.493516ms
Lockfree structures with 32 threads total time: 305.526021ms

Mutexed structures with 64 threads total time: 313.503112ms
Lockfree structures with 64 threads total time: 327.561084ms

Mutexed structures with 128 threads total time: 390.7458ms
Lockfree structures with 128 threads total time: 391.902957ms
```

