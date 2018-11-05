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
CPU MHz:             2334.445
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
mean of 506884944.121 r/s (633703424 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 137799983.929 r/s (172274688 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 167807665.124 r/s (209792000 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 1203331742.619 r/s (1504316416 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 198949850.188 r/s (248714240 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 266567761.914 r/s (333255680 rounds in 1.250 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 1203133935.408 r/s (1504259072 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 125908426.344 r/s (157447168 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 248178313.991 r/s (310289408 rounds in 1.250 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 1200564890.654 r/s (1501357056 rounds in 1.251 seconds)
Target 1 (blocking):
mean of 93022696.357 r/s (116344832 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 260516417.876 r/s (325999616 rounds in 1.251 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 1200225288.782 r/s (1502545920 rounds in 1.252 seconds)
Target 1 (blocking):
mean of 37593092.543 r/s (47168512 rounds in 1.255 seconds)
Target 2 (lockfree):
mean of 219741055.085 r/s (275136512 rounds in 1.252 seconds)

```

## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 45398249.659 r/s (56754176 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 29766528.835 r/s (37211136 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 12381327.410 r/s (15477760 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 12565347.732 r/s (15708160 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4283962.507 r/s (5355520 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 3536474.418 r/s (4421632 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 8626015.002 r/s (10783744 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 5003238.780 r/s (6255616 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 3416715.166 r/s (4273152 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8284311.021 r/s (10359808 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4882225.770 r/s (6105088 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 3429705.930 r/s (4300800 rounds in 1.254 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 8241267.171 r/s (10310656 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4858995.067 r/s (6081536 rounds in 1.252 seconds)
Target 2 (lockfree):
mean of 3480216.397 r/s (4361216 rounds in 1.253 seconds)

```

## STACK
```

Result for 1 threads:
Target 0 (mutex vector):
mean of 48753228.275 r/s (60952576 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 27167976.583 r/s (33963008 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 19981038.950 r/s (24980480 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 8855877.300 r/s (11071488 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 3942019.695 r/s (4928512 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5751273.059 r/s (7191552 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 7938387.016 r/s (9925632 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4932135.594 r/s (6168576 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 5465595.150 r/s (6834176 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 7374400.347 r/s (9223168 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4817279.619 r/s (6026240 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 5581419.752 r/s (6982656 rounds in 1.251 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 7365666.708 r/s (9211904 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4790261.419 r/s (5996544 rounds in 1.252 seconds)
Target 2 (lockfree):
mean of 5423705.124 r/s (6832128 rounds in 1.260 seconds)

```

## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 3226153.608 r/s (4520960 rounds in 1.401 seconds)
Target 1 (lockfree insert):
mean of 1857553.950 r/s (2322432 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 1948044.676 r/s (2436096 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 2123018.801 r/s (2654208 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2239540.478 r/s (2801664 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 2585673.203 r/s (3233792 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2276246.644 r/s (2847744 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 2303928.468 r/s (2900992 rounds in 1.259 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 6117621.264 r/s (7649280 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 3788978.447 r/s (4737024 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 2060405.538 r/s (2576384 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 3444461.924 r/s (4308992 rounds in 1.251 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2360470.657 r/s (2953216 rounds in 1.251 seconds)
Target 1 (lockfree get):
mean of 5122837.447 r/s (6406144 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2424804.214 r/s (3034112 rounds in 1.251 seconds)
Target 1 (lockfree get):
mean of 5182354.406 r/s (6488064 rounds in 1.252 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 20915812.933 r/s (26146816 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 1792057.786 r/s (2240512 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 13133522.776 r/s (16418816 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 3607679.916 r/s (4521984 rounds in 1.253 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10705921.422 r/s (13384704 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 7345251.058 r/s (9191424 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10576299.708 r/s (13224960 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 8060931.027 r/s (10081280 rounds in 1.251 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 4423628.347 r/s (5531648 rounds in 1.250 seconds)
Target 1 (lockfree mixed):
mean of 1432956.467 r/s (1792000 rounds in 1.251 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1498069.306 r/s (1873920 rounds in 1.251 seconds)
Target 1 (lockfree mixed):
mean of 901219.117 r/s (1128448 rounds in 1.252 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1287416.785 r/s (1610752 rounds in 1.251 seconds)
Target 1 (lockfree mixed):
mean of 1082015.727 r/s (1355776 rounds in 1.253 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1398615.577 r/s (1753088 rounds in 1.253 seconds)
Target 1 (lockfree mixed):
mean of 999872.649 r/s (1274880 rounds in 1.275 seconds)

```

## REQUEST PROGRAM
```
A program simulating a concurrent server.

Mutexed HashMap and VecDeque with 2 threads total time: 795.596324ms
Mutexed HashMap and LinkedList with 2 threads total time: 806.622719ms
Lockfree structures with 2 threads total time: 759.07448ms

Mutexed HashMap and VecDeque with 4 threads total time: 890.34346ms
Mutexed HashMap and LinkedList with 4 threads total time: 899.928908ms
Lockfree structures with 4 threads total time: 997.570793ms

Mutexed HashMap and VecDeque with 8 threads total time: 930.45526ms
Mutexed HashMap and LinkedList with 8 threads total time: 940.526473ms
Lockfree structures with 8 threads total time: 1.275565827s

Mutexed HashMap and VecDeque with 16 threads total time: 1.499977349s
Mutexed HashMap and LinkedList with 16 threads total time: 1.977173103s
Lockfree structures with 16 threads total time: 1.953965361s
```

## MESSAGE REVERB PROGRAM
```
A program which reverberates messages

Mutexed VecDeque with 2 threads total time: 147.783089ms
Mutexed LinkedList with 2 threads total time: 389.952621ms
Lockfree Queue with 2 threads total time: 194.377615ms

Mutexed VecDeque with 4 threads total time: 316.629785ms
Mutexed LinkedList with 4 threads total time: 595.138708ms
Lockfree Queue with 4 threads total time: 402.449956ms

Mutexed VecDeque with 8 threads total time: 655.918567ms
Mutexed LinkedList with 8 threads total time: 1.251513045s
Lockfree Queue with 8 threads total time: 809.194516ms

Mutexed VecDeque with 16 threads total time: 1.33880518s
Mutexed LinkedList with 16 threads total time: 2.617906757s
Lockfree Queue with 16 threads total time: 1.549800882s
```

