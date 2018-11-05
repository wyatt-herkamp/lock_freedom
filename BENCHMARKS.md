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
CPU MHz:             700.021
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
mean of 510037312.878 r/s (637632512 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 138400411.316 r/s (173010944 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 168659895.670 r/s (210856960 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 1210680173.903 r/s (1513499648 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 200139112.980 r/s (250207232 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 277335978.236 r/s (346701824 rounds in 1.250 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 1211069430.364 r/s (1514138624 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 126837207.121 r/s (158587904 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 277104736.607 r/s (346454016 rounds in 1.250 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 1210588972.191 r/s (1513852928 rounds in 1.251 seconds)
Target 1 (blocking):
mean of 90051551.570 r/s (112628736 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 247460232.522 r/s (309450752 rounds in 1.251 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 1209960464.848 r/s (1514790912 rounds in 1.252 seconds)
Target 1 (blocking):
mean of 37771824.160 r/s (47383552 rounds in 1.254 seconds)
Target 2 (lockfree):
mean of 206148582.497 r/s (258108416 rounds in 1.252 seconds)

```

## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 45994533.567 r/s (57503744 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 29983267.765 r/s (37484544 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 12486111.604 r/s (15610880 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 11966894.200 r/s (14960640 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 3894590.945 r/s (4869120 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 3397589.946 r/s (4247552 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 8535322.035 r/s (10671104 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4984659.848 r/s (6233088 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 3269365.821 r/s (4087808 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8223287.232 r/s (10281984 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4956947.032 r/s (6200320 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 3329207.641 r/s (4166656 rounds in 1.252 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 8207917.552 r/s (10268672 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4932889.822 r/s (6172672 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 3381613.236 r/s (4250624 rounds in 1.257 seconds)

```

## STACK
```

Result for 1 threads:
Target 0 (mutex vector):
mean of 48985891.046 r/s (61242368 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 26699603.026 r/s (33379328 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 20100908.851 r/s (25127936 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 8320804.501 r/s (10401792 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 3780214.241 r/s (4725760 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5847010.953 r/s (7309312 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 7812026.322 r/s (9766912 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4948121.544 r/s (6187008 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5810420.268 r/s (7264256 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 7407643.880 r/s (9263104 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4896362.464 r/s (6123520 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 5784101.088 r/s (7245824 rounds in 1.253 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 7414127.874 r/s (9275392 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4864625.569 r/s (6087680 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 5851751.805 r/s (7328768 rounds in 1.252 seconds)

```

## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 3305540.832 r/s (4520960 rounds in 1.368 seconds)
Target 1 (lockfree insert):
mean of 1901473.269 r/s (2377728 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 2033531.740 r/s (2542592 rounds in 1.250 seconds)
Target 1 (lockfree insert):
mean of 2146781.936 r/s (2684928 rounds in 1.251 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2274502.321 r/s (2846720 rounds in 1.252 seconds)
Target 1 (lockfree insert):
mean of 2575052.259 r/s (3221504 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2397717.335 r/s (3002368 rounds in 1.252 seconds)
Target 1 (lockfree insert):
mean of 2434677.101 r/s (3052544 rounds in 1.254 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 6620138.552 r/s (8276992 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 4125150.167 r/s (5157888 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 2080300.808 r/s (2600960 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 3389536.113 r/s (4239360 rounds in 1.251 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2346240.648 r/s (2934784 rounds in 1.251 seconds)
Target 1 (lockfree get):
mean of 5071317.291 r/s (6341632 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2575465.994 r/s (3224576 rounds in 1.252 seconds)
Target 1 (lockfree get):
mean of 5159881.481 r/s (6456320 rounds in 1.251 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 23085200.996 r/s (28861440 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 1802501.914 r/s (2253824 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 12801005.444 r/s (16003072 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 4366962.229 r/s (5459968 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10866167.888 r/s (13586432 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 8001188.489 r/s (10002432 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10745528.223 r/s (13436928 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 8094516.523 r/s (10124288 rounds in 1.251 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 4553852.033 r/s (5694464 rounds in 1.250 seconds)
Target 1 (lockfree mixed):
mean of 1450067.983 r/s (1813504 rounds in 1.251 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1533691.608 r/s (1918976 rounds in 1.251 seconds)
Target 1 (lockfree mixed):
mean of 928293.621 r/s (1163264 rounds in 1.253 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1327524.746 r/s (1661952 rounds in 1.252 seconds)
Target 1 (lockfree mixed):
mean of 1019792.845 r/s (1277952 rounds in 1.253 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1435172.994 r/s (1799168 rounds in 1.254 seconds)
Target 1 (lockfree mixed):
mean of 961216.090 r/s (1253376 rounds in 1.304 seconds)

```

## REQUEST PROGRAM
```
A program simulating a concurrent server.

Mutexed HashMap and VecDeque with 2 threads total time: 1.307733299s
Mutexed HashMap and LinkedList with 2 threads total time: 1.192109554s
Lockfree structures with 2 threads total time: 701.292178ms

Mutexed HashMap and VecDeque with 4 threads total time: 751.81623ms
Mutexed HashMap and LinkedList with 4 threads total time: 1.918179635s
Lockfree structures with 4 threads total time: 855.664422ms

Mutexed HashMap and VecDeque with 8 threads total time: 950.22813ms
Mutexed HashMap and LinkedList with 8 threads total time: 2.18572619s
Lockfree structures with 8 threads total time: 1.128883307s

Mutexed HashMap and VecDeque with 16 threads total time: 2.285765566s
Mutexed HashMap and LinkedList with 16 threads total time: 4.484809329s
Lockfree structures with 16 threads total time: 2.565418957s
```

## MESSAGE REVERB PROGRAM
```
A program which reverberates messages

Mutexed VecDeque with 2 threads total time: 221.496398ms
Mutexed LinkedList with 2 threads total time: 621.002132ms
Lockfree Queue with 2 threads total time: 312.045661ms

Mutexed VecDeque with 4 threads total time: 237.634028ms
Mutexed LinkedList with 4 threads total time: 473.06379ms
Lockfree Queue with 4 threads total time: 304.805327ms

Mutexed VecDeque with 8 threads total time: 244.504738ms
Mutexed LinkedList with 8 threads total time: 482.283936ms
Lockfree Queue with 8 threads total time: 299.363579ms

Mutexed VecDeque with 16 threads total time: 240.650337ms
Mutexed LinkedList with 16 threads total time: 507.695217ms
Lockfree Queue with 16 threads total time: 270.323292ms
```

## HASH MINING
```
A program simulating a hash miner.

Mutexed structures with 2 threads total time: 457.399248ms
Lockfree structures with 2 threads total time: 456.455757ms

Mutexed structures with 4 threads total time: 294.110325ms
Lockfree structures with 4 threads total time: 314.990311ms

Mutexed structures with 8 threads total time: 260.374151ms
Lockfree structures with 8 threads total time: 260.084629ms

Mutexed structures with 16 threads total time: 249.630957ms
Lockfree structures with 16 threads total time: 241.979862ms

Mutexed structures with 32 threads total time: 249.944092ms
Lockfree structures with 32 threads total time: 242.833186ms

Mutexed structures with 64 threads total time: 254.084572ms
Lockfree structures with 64 threads total time: 263.936673ms

Mutexed structures with 128 threads total time: 325.826922ms
Lockfree structures with 128 threads total time: 339.216896ms
```

