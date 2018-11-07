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
CPU MHz:             2325.429
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
mean of 506384898.779 r/s (633082880 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 137622563.139 r/s (172041216 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 167542907.832 r/s (209460224 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 1202230920.752 r/s (1503006720 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 199177499.515 r/s (248996864 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 274812322.476 r/s (343548928 rounds in 1.250 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 1213129818.451 r/s (1516875776 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 125247309.327 r/s (156601344 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 272738632.185 r/s (341009408 rounds in 1.250 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 1199130714.050 r/s (1499491328 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 89911953.414 r/s (112456704 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 273157099.338 r/s (341598208 rounds in 1.251 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 1199395916.786 r/s (1501563904 rounds in 1.252 seconds)
Target 1 (blocking):
mean of 37783639.658 r/s (47410176 rounds in 1.255 seconds)
Target 2 (lockfree):
mean of 216497799.056 r/s (271086592 rounds in 1.252 seconds)

```

## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 45929850.750 r/s (57422848 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 29915138.909 r/s (37397504 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 9579032.781 r/s (11975680 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 12605510.669 r/s (15759360 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 3573019.696 r/s (4466688 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 4812700.941 r/s (6017024 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 8791505.592 r/s (10991616 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 5105184.752 r/s (6383616 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 4645713.358 r/s (5809152 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8286000.590 r/s (10361856 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4964026.473 r/s (6209536 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 4322598.168 r/s (5476352 rounds in 1.267 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 8296002.086 r/s (10381312 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4940980.618 r/s (6183936 rounds in 1.252 seconds)
Target 2 (lockfree):
mean of 4591983.090 r/s (5827584 rounds in 1.269 seconds)

```

## STACK
```

Result for 1 threads:
Target 0 (mutex vector):
mean of 48456842.983 r/s (60581888 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 27295789.327 r/s (34124800 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 19980153.074 r/s (24977408 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 8256901.120 r/s (10322944 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4573221.234 r/s (5718016 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5819759.699 r/s (7275520 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 7949849.676 r/s (9938944 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 5001153.436 r/s (6253568 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5669997.636 r/s (7090176 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 7545928.214 r/s (9435136 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4939050.908 r/s (6178816 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 5695202.260 r/s (7130112 rounds in 1.252 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 7531502.720 r/s (9420800 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4931388.214 r/s (6172672 rounds in 1.252 seconds)
Target 2 (lockfree):
mean of 5762473.542 r/s (7217152 rounds in 1.252 seconds)

```

## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 3247121.679 r/s (4520960 rounds in 1.392 seconds)
Target 1 (lockfree insert):
mean of 1901117.628 r/s (2377728 rounds in 1.251 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 1998761.866 r/s (2499584 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 2230340.843 r/s (2789376 rounds in 1.251 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2181377.689 r/s (2727936 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 3611492.159 r/s (4517888 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2283706.415 r/s (2859008 rounds in 1.252 seconds)
Target 1 (lockfree insert):
mean of 3677974.346 r/s (4632576 rounds in 1.260 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 6467433.875 r/s (8128512 rounds in 1.257 seconds)
Target 1 (lockfree get):
mean of 4535338.195 r/s (5670912 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 2044958.211 r/s (2556928 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 7275283.573 r/s (9096192 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2275439.021 r/s (2846720 rounds in 1.251 seconds)
Target 1 (lockfree get):
mean of 11348839.388 r/s (14189568 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2434078.796 r/s (3046400 rounds in 1.252 seconds)
Target 1 (lockfree get):
mean of 10686683.719 r/s (13547520 rounds in 1.268 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 20883498.463 r/s (26108928 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 1493665.580 r/s (2097152 rounds in 1.404 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 12321529.152 r/s (15404032 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 5305474.957 r/s (6633472 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10653034.538 r/s (13319168 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 9882128.788 r/s (12355584 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10543294.941 r/s (13184000 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 10644231.864 r/s (13310976 rounds in 1.251 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 4517196.779 r/s (5647360 rounds in 1.250 seconds)
Target 1 (lockfree mixed):
mean of 1616072.198 r/s (2021376 rounds in 1.251 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1523102.158 r/s (1904640 rounds in 1.251 seconds)
Target 1 (lockfree mixed):
mean of 1069648.738 r/s (1339392 rounds in 1.252 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1285826.638 r/s (1609728 rounds in 1.252 seconds)
Target 1 (lockfree mixed):
mean of 1650972.290 r/s (2066432 rounds in 1.252 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1408428.508 r/s (1764352 rounds in 1.253 seconds)
Target 1 (lockfree mixed):
mean of 1515383.265 r/s (2184192 rounds in 1.441 seconds)

```

## REQUEST PROGRAM
```
A program simulating a concurrent server.

Mutexed HashMap and VecDeque with 2 threads total time: 1.052718793s
Mutexed HashMap and LinkedList with 2 threads total time: 1.170706101s
Lockfree structures with 2 threads total time: 720.198625ms

Mutexed HashMap and VecDeque with 4 threads total time: 768.553116ms
Mutexed HashMap and LinkedList with 4 threads total time: 1.83794862s
Lockfree structures with 4 threads total time: 858.142468ms

Mutexed HashMap and VecDeque with 8 threads total time: 1.240324394s
Mutexed HashMap and LinkedList with 8 threads total time: 2.180575693s
Lockfree structures with 8 threads total time: 734.540386ms

Mutexed HashMap and VecDeque with 16 threads total time: 1.938417272s
Mutexed HashMap and LinkedList with 16 threads total time: 4.240822806s
Lockfree structures with 16 threads total time: 1.641062488s
```

## MESSAGE REVERB PROGRAM
```
A program which reverberates messages

Mutexed VecDeque with 2 threads total time: 196.031037ms
Mutexed LinkedList with 2 threads total time: 561.069336ms
Lockfree Queue with 2 threads total time: 199.863388ms

Mutexed VecDeque with 4 threads total time: 235.187245ms
Mutexed LinkedList with 4 threads total time: 458.923928ms
Lockfree Queue with 4 threads total time: 207.93559ms

Mutexed VecDeque with 8 threads total time: 243.865999ms
Mutexed LinkedList with 8 threads total time: 478.871162ms
Lockfree Queue with 8 threads total time: 212.000033ms

Mutexed VecDeque with 16 threads total time: 252.749905ms
Mutexed LinkedList with 16 threads total time: 484.609162ms
Lockfree Queue with 16 threads total time: 194.03816ms
```

## HASH MINING
```
A program simulating a hash miner.

Mutexed structures with 2 threads total time: 474.39992ms
Lockfree structures with 2 threads total time: 475.61357ms

Mutexed structures with 4 threads total time: 302.402177ms
Lockfree structures with 4 threads total time: 311.631188ms

Mutexed structures with 8 threads total time: 254.404926ms
Lockfree structures with 8 threads total time: 265.386781ms

Mutexed structures with 16 threads total time: 244.182947ms
Lockfree structures with 16 threads total time: 251.273425ms

Mutexed structures with 32 threads total time: 253.296031ms
Lockfree structures with 32 threads total time: 252.436322ms

Mutexed structures with 64 threads total time: 259.672123ms
Lockfree structures with 64 threads total time: 276.421097ms

Mutexed structures with 128 threads total time: 328.819506ms
Lockfree structures with 128 threads total time: 339.17546ms
```

