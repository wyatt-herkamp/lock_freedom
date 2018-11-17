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
CPU MHz:             2720.314
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
mean of 439645192.770 r/s (549639168 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 127572248.390 r/s (159489024 rounds in 1.250 seconds)
Target 2 (blocking with cached access):
mean of 311227029.036 r/s (389093376 rounds in 1.250 seconds)
Target 3 (lockfree):
mean of 144818945.936 r/s (181036032 rounds in 1.250 seconds)
Target 4 (lockfree with cached id):
mean of 192063794.105 r/s (240095232 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 1223957855.846 r/s (1530082304 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 201030680.596 r/s (251311104 rounds in 1.250 seconds)
Target 2 (blocking with cached access):
mean of 341790876.815 r/s (427270144 rounds in 1.250 seconds)
Target 3 (lockfree):
mean of 292911482.019 r/s (366168064 rounds in 1.250 seconds)
Target 4 (lockfree with cached id):
mean of 406536148.630 r/s (508211200 rounds in 1.250 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 1224831135.726 r/s (1531313152 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 147946321.942 r/s (184969216 rounds in 1.250 seconds)
Target 2 (blocking with cached access):
mean of 192633565.903 r/s (240843776 rounds in 1.250 seconds)
Target 3 (lockfree):
mean of 292985933.538 r/s (366297088 rounds in 1.250 seconds)
Target 4 (lockfree with cached id):
mean of 409361318.449 r/s (511794176 rounds in 1.250 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 1224647453.766 r/s (1531260928 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 115911185.972 r/s (144956416 rounds in 1.251 seconds)
Target 2 (blocking with cached access):
mean of 132108568.983 r/s (165215232 rounds in 1.251 seconds)
Target 3 (lockfree):
mean of 292727562.619 r/s (366053376 rounds in 1.250 seconds)
Target 4 (lockfree with cached id):
mean of 409465120.628 r/s (512010240 rounds in 1.250 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 1223840145.962 r/s (1531779072 rounds in 1.252 seconds)
Target 1 (blocking):
mean of 56520331.785 r/s (70829056 rounds in 1.253 seconds)
Target 2 (blocking with cached access):
mean of 61601168.248 r/s (77193216 rounds in 1.253 seconds)
Target 3 (lockfree):
mean of 292873912.554 r/s (366572544 rounds in 1.252 seconds)
Target 4 (lockfree with cached id):
mean of 409345887.189 r/s (512361472 rounds in 1.252 seconds)

```

## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 46008034.137 r/s (57519104 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 29662212.739 r/s (37083136 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 14520751.154 r/s (18153472 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 11565284.708 r/s (14457856 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 3757342.354 r/s (4697088 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5118925.648 r/s (6400000 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 8824871.996 r/s (11033600 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4998322.776 r/s (6250496 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 6141029.594 r/s (7680000 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8467591.857 r/s (10589184 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4913925.513 r/s (6147072 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 5871593.914 r/s (7414784 rounds in 1.263 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 8441577.782 r/s (10559488 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4899400.648 r/s (6130688 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 6037621.298 r/s (7752704 rounds in 1.284 seconds)

```

## STACK
```

Result for 1 threads:
Target 0 (mutex vector):
mean of 48633018.931 r/s (60801024 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 29566494.929 r/s (36961280 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 19576060.906 r/s (24473600 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 7786161.403 r/s (9734144 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4513158.230 r/s (5642240 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5846570.650 r/s (7309312 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 7841766.105 r/s (9803776 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 5155637.460 r/s (6447104 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 6353401.627 r/s (7944192 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 7529036.041 r/s (9415680 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 5145428.983 r/s (6436864 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 6422169.547 r/s (8187904 rounds in 1.275 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 7529110.848 r/s (9433088 rounds in 1.253 seconds)
Target 1 (mutex linked list):
mean of 5137016.572 r/s (6428672 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 6430637.874 r/s (8188928 rounds in 1.273 seconds)

```

## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 3263364.043 r/s (4520960 rounds in 1.385 seconds)
Target 1 (lockfree insert):
mean of 2012092.859 r/s (2515968 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 2020681.334 r/s (2526208 rounds in 1.250 seconds)
Target 1 (lockfree insert):
mean of 2524058.480 r/s (3155968 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2301094.318 r/s (2879488 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 3760455.755 r/s (4702208 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2467795.079 r/s (3090432 rounds in 1.252 seconds)
Target 1 (lockfree insert):
mean of 3863077.822 r/s (5137408 rounds in 1.330 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 6540012.154 r/s (8176640 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 4693118.518 r/s (5867520 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 2061532.939 r/s (2578432 rounds in 1.251 seconds)
Target 1 (lockfree get):
mean of 7082048.543 r/s (8853504 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2405508.463 r/s (3010560 rounds in 1.252 seconds)
Target 1 (lockfree get):
mean of 10793598.439 r/s (13496320 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2490676.394 r/s (3117056 rounds in 1.251 seconds)
Target 1 (lockfree get):
mean of 10274394.348 r/s (12850176 rounds in 1.251 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 21379055.308 r/s (26728448 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 3033575.329 r/s (3792896 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 12261385.106 r/s (15328256 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 8924479.641 r/s (11157504 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10964522.649 r/s (13707264 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 11709830.937 r/s (14639104 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10882074.075 r/s (13608960 rounds in 1.251 seconds)
Target 1 (lockfree remove):
mean of 12118440.219 r/s (15154176 rounds in 1.251 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 4530506.783 r/s (5664768 rounds in 1.250 seconds)
Target 1 (lockfree mixed):
mean of 1998433.825 r/s (2498560 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1545946.141 r/s (1933312 rounds in 1.251 seconds)
Target 1 (lockfree mixed):
mean of 1139799.378 r/s (1427456 rounds in 1.252 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1303669.923 r/s (1631232 rounds in 1.251 seconds)
Target 1 (lockfree mixed):
mean of 1587801.987 r/s (1986560 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1411358.456 r/s (1768448 rounds in 1.253 seconds)
Target 1 (lockfree mixed):
mean of 2272391.124 r/s (2984960 rounds in 1.314 seconds)

```

## MPSC CHANNEL
```
Mutexed VecDeque with 3 threads total time: 198.361552ms
Std's MPSC with 3 threads total time: 142.750497ms
Lockfree MPSC with 3 threads total time: 109.820408ms

Mutexed VecDeque with 5 threads total time: 452.112589ms
Std's MPSC with 5 threads total time: 245.36246ms
Lockfree MPSC with 5 threads total time: 200.132915ms

Mutexed VecDeque with 9 threads total time: 845.043886ms
Std's MPSC with 9 threads total time: 483.207737ms
Lockfree MPSC with 9 threads total time: 379.172909ms

Mutexed VecDeque with 17 threads total time: 1.635998081s
Std's MPSC with 17 threads total time: 998.572514ms
Lockfree MPSC with 17 threads total time: 766.748923ms

Mutexed VecDeque with 33 threads total time: 3.139739494s
Std's MPSC with 33 threads total time: 1.950121912s
Lockfree MPSC with 33 threads total time: 1.565820478s
```

## SPSC CHANNEL
```
Mutexed VecDeque total time: 552.694358ms
Std's MPSC (as SPSC) total time: 73.120241ms
Lockfree SPSC total time: 337.312057ms
```

## SPMC CHANNEL
```
Mutexed VecDeque with 3 threads total time: 238.186823ms
Mutexed Std's MPSC (as SPMC) with 3 threads total time: 264.777994ms
Lockfree SPMC with 3 threads total time: 113.80611ms

Mutexed VecDeque with 5 threads total time: 354.711929ms
Mutexed Std's MPSC (as SPMC) with 5 threads total time: 159.856592ms
Lockfree SPMC with 5 threads total time: 92.494851ms

Mutexed VecDeque with 9 threads total time: 664.160033ms
Mutexed Std's MPSC (as SPMC) with 9 threads total time: 161.485165ms
Lockfree SPMC with 9 threads total time: 92.819085ms

Mutexed VecDeque with 17 threads total time: 1.123390345s
Mutexed Std's MPSC (as SPMC) with 17 threads total time: 173.323065ms
Lockfree SPMC with 17 threads total time: 180.423884ms

Mutexed VecDeque with 33 threads total time: 2.382446963s
Mutexed Std's MPSC (as SPMC) with 33 threads total time: 188.944963ms
Lockfree SPMC with 33 threads total time: 476.985645ms
```

## MPMC CHANNEL
```
Mutexed VecDeque with 4 threads total time: 83.437172ms
Mutexed Std's MPSC (as MPMC)  with 4 threads total time: 141.620542ms
Lockfree MPMC with 4 threads total time: 44.71398ms

Mutexed VecDeque with 8 threads total time: 178.852085ms
Mutexed Std's MPSC (as MPMC)  with 8 threads total time: 177.67061ms
Lockfree MPMC with 8 threads total time: 96.309621ms

Mutexed VecDeque with 16 threads total time: 406.463208ms
Mutexed Std's MPSC (as MPMC)  with 16 threads total time: 413.746805ms
Lockfree MPMC with 16 threads total time: 278.775144ms
```

## REQUEST PROGRAM
```
A program simulating a concurrent server.

Mutexed HashMap and Std's MPSC with 2 threads total time: 787.916789ms
Lockfree structures with 2 threads total time: 459.022772ms

Mutexed HashMap and Std's MPSC with 4 threads total time: 350.87238ms
Lockfree structures with 4 threads total time: 426.717735ms

Mutexed HashMap and Std's MPSC with 8 threads total time: 350.858761ms
Lockfree structures with 8 threads total time: 349.094082ms

Mutexed HashMap and Std's MPSC with 16 threads total time: 379.910996ms
Lockfree structures with 16 threads total time: 419.586687ms
```

## MESSAGE REVERB PROGRAM
```
A program which reverberates messages through a plain queue channel

Mutexed VecDeque with 2 threads total time: 190.870653ms
Mutexed LinkedList with 2 threads total time: 621.114091ms
Lockfree Queue with 2 threads total time: 199.2706ms

Mutexed VecDeque with 4 threads total time: 236.334975ms
Mutexed LinkedList with 4 threads total time: 457.31927ms
Lockfree Queue with 4 threads total time: 161.088467ms

Mutexed VecDeque with 8 threads total time: 254.613069ms
Mutexed LinkedList with 8 threads total time: 479.513823ms
Lockfree Queue with 8 threads total time: 152.476832ms

Mutexed VecDeque with 16 threads total time: 248.407052ms
Mutexed LinkedList with 16 threads total time: 485.27814ms
Lockfree Queue with 16 threads total time: 146.291211ms
```

## HASH MINING
```
A program simulating a hash miner.

Mutexed structures with 2 threads total time: 455.002795ms
Lockfree structures with 2 threads total time: 459.938592ms

Mutexed structures with 4 threads total time: 313.208754ms
Lockfree structures with 4 threads total time: 315.818463ms

Mutexed structures with 8 threads total time: 262.358454ms
Lockfree structures with 8 threads total time: 249.799204ms

Mutexed structures with 16 threads total time: 259.726634ms
Lockfree structures with 16 threads total time: 247.317981ms

Mutexed structures with 32 threads total time: 252.027951ms
Lockfree structures with 32 threads total time: 249.552583ms

Mutexed structures with 64 threads total time: 249.962936ms
Lockfree structures with 64 threads total time: 265.49984ms

Mutexed structures with 128 threads total time: 323.723959ms
Lockfree structures with 128 threads total time: 334.856954ms
```

