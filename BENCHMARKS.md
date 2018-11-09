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
CPU MHz:             2839.136
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
mean of 507741899.578 r/s (634748928 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 137650458.171 r/s (172081152 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 169274358.243 r/s (211609600 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 1198219637.830 r/s (1497925632 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 198401878.770 r/s (248027136 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 273308504.037 r/s (341667840 rounds in 1.250 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 1196630654.942 r/s (1496177664 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 125051713.578 r/s (156357632 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 273997429.090 r/s (342577152 rounds in 1.250 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 1197388258.178 r/s (1497389056 rounds in 1.251 seconds)
Target 1 (blocking):
mean of 89084144.779 r/s (111492096 rounds in 1.252 seconds)
Target 2 (lockfree):
mean of 274377618.877 r/s (343131136 rounds in 1.251 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 1195703977.814 r/s (1497011200 rounds in 1.252 seconds)
Target 1 (blocking):
mean of 38351609.894 r/s (48116736 rounds in 1.255 seconds)
Target 2 (lockfree):
mean of 223173395.728 r/s (279477248 rounds in 1.252 seconds)

```

## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 45883921.321 r/s (57360384 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 29228773.321 r/s (36539392 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 9544722.255 r/s (11932672 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 11744277.092 r/s (14682112 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 3167508.970 r/s (3960832 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5270494.996 r/s (6589440 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 8731735.213 r/s (10916864 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4994136.491 r/s (6244352 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5303562.895 r/s (6632448 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8429409.148 r/s (10541056 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4946563.928 r/s (6184960 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 4738954.957 r/s (5958656 rounds in 1.257 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 8421163.498 r/s (10534912 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4885458.498 r/s (6114304 rounds in 1.252 seconds)
Target 2 (lockfree):
mean of 5300298.090 r/s (6653952 rounds in 1.255 seconds)

```

## STACK
```

Result for 1 threads:
Target 0 (mutex vector):
mean of 48398493.834 r/s (60503040 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 27601726.502 r/s (34504704 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 20498707.272 r/s (25625600 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 8157491.198 r/s (10199040 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4267619.729 r/s (5336064 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5876428.051 r/s (7346176 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 7910989.973 r/s (9890816 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 5053801.683 r/s (6320128 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 6653860.784 r/s (8320000 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 7554924.560 r/s (9448448 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4914819.172 r/s (6146048 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 6728119.084 r/s (8421376 rounds in 1.252 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 7576117.892 r/s (9477120 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4896364.229 r/s (6128640 rounds in 1.252 seconds)
Target 2 (lockfree):
mean of 6661393.945 r/s (8399872 rounds in 1.261 seconds)

```

## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 3592404.451 r/s (4493312 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 1938605.364 r/s (2423808 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 1785611.263 r/s (2232320 rounds in 1.250 seconds)
Target 1 (lockfree insert):
mean of 2200442.631 r/s (2752512 rounds in 1.251 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 1863756.069 r/s (2332672 rounds in 1.252 seconds)
Target 1 (lockfree insert):
mean of 3621594.445 r/s (4530176 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 1984211.085 r/s (2486272 rounds in 1.253 seconds)
Target 1 (lockfree insert):
mean of 3525555.200 r/s (4502528 rounds in 1.277 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 5126918.830 r/s (6409216 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 4743514.723 r/s (5929984 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 1827978.561 r/s (2285568 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 7350724.334 r/s (9189376 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2074740.267 r/s (2596864 rounds in 1.252 seconds)
Target 1 (lockfree get):
mean of 11173538.423 r/s (13970432 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2139110.397 r/s (2677760 rounds in 1.252 seconds)
Target 1 (lockfree get):
mean of 10490992.471 r/s (13120512 rounds in 1.251 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 16980631.780 r/s (21229568 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 1761480.062 r/s (2202624 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 11997833.757 r/s (14999552 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 4669331.506 r/s (5838848 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10599611.043 r/s (13252608 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 10462928.096 r/s (13082624 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10494155.365 r/s (13122560 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 10986705.340 r/s (13739008 rounds in 1.251 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 4081935.222 r/s (5104640 rounds in 1.251 seconds)
Target 1 (lockfree mixed):
mean of 1672619.699 r/s (2092032 rounds in 1.251 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1142294.776 r/s (1429504 rounds in 1.251 seconds)
Target 1 (lockfree mixed):
mean of 1085313.201 r/s (1357824 rounds in 1.251 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1014132.139 r/s (1269760 rounds in 1.252 seconds)
Target 1 (lockfree mixed):
mean of 1734836.859 r/s (2170880 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1019999.535 r/s (1280000 rounds in 1.255 seconds)
Target 1 (lockfree mixed):
mean of 1827872.075 r/s (2778112 rounds in 1.520 seconds)

```

## MPSC CHANNEL
```
Result for 2 threads:
Target 0 (mutex vector):
mean of 9034737.094 r/s (11295744 rounds in 1.250 seconds)
Target 1 (std's mpsc):
mean of 5423061.901 r/s (6779904 rounds in 1.250 seconds)
Target 2 (lockfree mpsc):
mean of 5782158.881 r/s (7228416 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 7338991.447 r/s (9176064 rounds in 1.250 seconds)
Target 1 (std's mpsc):
mean of 7899528.910 r/s (9877504 rounds in 1.250 seconds)
Target 2 (lockfree mpsc):
mean of 6831609.191 r/s (8542208 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 6829625.560 r/s (8541184 rounds in 1.251 seconds)
Target 1 (std's mpsc):
mean of 6369473.098 r/s (7966720 rounds in 1.251 seconds)
Target 2 (lockfree mpsc):
mean of 5954265.650 r/s (7446528 rounds in 1.251 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 6812557.917 r/s (8523776 rounds in 1.251 seconds)
Target 1 (std's mpsc):
mean of 6145458.824 r/s (7691264 rounds in 1.252 seconds)
Target 2 (lockfree mpsc):
mean of 5626798.977 r/s (7047168 rounds in 1.252 seconds)

```

## REQUEST PROGRAM
```
A program simulating a concurrent server.

Mutexed HashMap and VecDeque with 2 threads total time: 1.2447158s
Mutexed HashMap and LinkedList with 2 threads total time: 1.212344498s
Lockfree structures with 2 threads total time: 831.9816ms

Mutexed HashMap and VecDeque with 4 threads total time: 728.373929ms
Mutexed HashMap and LinkedList with 4 threads total time: 1.908262451s
Lockfree structures with 4 threads total time: 933.26584ms

Mutexed HashMap and VecDeque with 8 threads total time: 1.063502s
Mutexed HashMap and LinkedList with 8 threads total time: 2.037281546s
Lockfree structures with 8 threads total time: 728.173381ms

Mutexed HashMap and VecDeque with 16 threads total time: 2.271891585s
Mutexed HashMap and LinkedList with 16 threads total time: 4.53943494s
Lockfree structures with 16 threads total time: 1.534898153s
```

## MESSAGE REVERB PROGRAM
```
A program which reverberates messages

Mutexed VecDeque with 2 threads total time: 218.431849ms
Mutexed LinkedList with 2 threads total time: 610.325279ms
Lockfree Queue with 2 threads total time: 214.392365ms

Mutexed VecDeque with 4 threads total time: 232.118439ms
Mutexed LinkedList with 4 threads total time: 455.584408ms
Lockfree Queue with 4 threads total time: 188.387874ms

Mutexed VecDeque with 8 threads total time: 246.378047ms
Mutexed LinkedList with 8 threads total time: 481.735487ms
Lockfree Queue with 8 threads total time: 186.827413ms

Mutexed VecDeque with 16 threads total time: 243.991941ms
Mutexed LinkedList with 16 threads total time: 485.234486ms
Lockfree Queue with 16 threads total time: 182.034745ms
```

## HASH MINING
```
A program simulating a hash miner.

Mutexed structures with 2 threads total time: 480.534487ms
Lockfree structures with 2 threads total time: 472.414102ms

Mutexed structures with 4 threads total time: 312.471868ms
Lockfree structures with 4 threads total time: 319.537237ms

Mutexed structures with 8 threads total time: 259.721725ms
Lockfree structures with 8 threads total time: 250.186689ms

Mutexed structures with 16 threads total time: 250.431915ms
Lockfree structures with 16 threads total time: 243.995025ms

Mutexed structures with 32 threads total time: 255.514953ms
Lockfree structures with 32 threads total time: 247.570803ms

Mutexed structures with 64 threads total time: 262.844963ms
Lockfree structures with 64 threads total time: 272.703694ms

Mutexed structures with 128 threads total time: 332.279768ms
Lockfree structures with 128 threads total time: 350.052461ms
```

