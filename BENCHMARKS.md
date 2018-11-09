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
CPU MHz:             2726.324
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
mean of 549687365.414 r/s (687218688 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 137923847.190 r/s (172421120 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 167281146.915 r/s (209117184 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 1249744245.126 r/s (1562330112 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 191920809.250 r/s (239924224 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 279090523.798 r/s (348896256 rounds in 1.250 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 1248904896.912 r/s (1561549824 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 124385072.398 r/s (155522048 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 281310131.993 r/s (351723520 rounds in 1.250 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 1245335387.953 r/s (1557284864 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 90575330.543 r/s (113282048 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 281113208.115 r/s (351542272 rounds in 1.251 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 1245206121.731 r/s (1559058432 rounds in 1.252 seconds)
Target 1 (blocking):
mean of 37360902.630 r/s (46876672 rounds in 1.255 seconds)
Target 2 (lockfree):
mean of 220235719.862 r/s (275763200 rounds in 1.252 seconds)

```

## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 45820246.996 r/s (57280512 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 29025301.590 r/s (36284416 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 15481662.578 r/s (19355648 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 11797777.926 r/s (14748672 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 3143141.116 r/s (3930112 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5352924.516 r/s (6691840 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 8712497.836 r/s (10893312 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 5137199.098 r/s (6423552 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 6124156.147 r/s (7658496 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8361956.748 r/s (10457088 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4942922.874 r/s (6183936 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 5487553.788 r/s (7033856 rounds in 1.282 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 8425437.169 r/s (10541056 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4929592.943 r/s (6169600 rounds in 1.252 seconds)
Target 2 (lockfree):
mean of 5746963.870 r/s (7349248 rounds in 1.279 seconds)

```

## STACK
```

Result for 1 threads:
Target 0 (mutex vector):
mean of 48287775.019 r/s (60364800 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 23042735.228 r/s (28806144 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 21526238.115 r/s (26909696 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 8350234.966 r/s (10439680 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 3751087.476 r/s (4690944 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 6234985.826 r/s (7794688 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 7679103.516 r/s (9602048 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4988308.424 r/s (6237184 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 6854575.912 r/s (8570880 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 7423648.172 r/s (9282560 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4862201.155 r/s (6082560 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 6731662.621 r/s (8560640 rounds in 1.272 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 7415934.719 r/s (9287680 rounds in 1.252 seconds)
Target 1 (mutex linked list):
mean of 4851626.864 r/s (6072320 rounds in 1.252 seconds)
Target 2 (lockfree):
mean of 6746181.086 r/s (8601600 rounds in 1.275 seconds)

```

## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 3277319.407 r/s (4520960 rounds in 1.379 seconds)
Target 1 (lockfree insert):
mean of 2010515.428 r/s (2513920 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 1975169.951 r/s (2469888 rounds in 1.250 seconds)
Target 1 (lockfree insert):
mean of 2539448.802 r/s (3176448 rounds in 1.251 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2199596.372 r/s (2750464 rounds in 1.250 seconds)
Target 1 (lockfree insert):
mean of 3631129.449 r/s (4543488 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2312650.016 r/s (2894848 rounds in 1.252 seconds)
Target 1 (lockfree insert):
mean of 3671811.850 r/s (4667392 rounds in 1.271 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 6404608.088 r/s (8006656 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 4617575.349 r/s (5773312 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 2031905.143 r/s (2540544 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 7126982.779 r/s (8911872 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2114923.809 r/s (2646016 rounds in 1.251 seconds)
Target 1 (lockfree get):
mean of 11326685.630 r/s (14182400 rounds in 1.252 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2451381.341 r/s (3069952 rounds in 1.252 seconds)
Target 1 (lockfree get):
mean of 11161378.948 r/s (13958144 rounds in 1.251 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 20959583.093 r/s (26204160 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 3297956.737 r/s (4123648 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 12287549.749 r/s (15361024 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 7323527.406 r/s (9156608 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 9173840.576 r/s (13047808 rounds in 1.422 seconds)
Target 1 (lockfree remove):
mean of 11354959.862 r/s (14198784 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10296824.856 r/s (12876800 rounds in 1.251 seconds)
Target 1 (lockfree remove):
mean of 11440752.914 r/s (14307328 rounds in 1.251 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 4500630.510 r/s (5626880 rounds in 1.250 seconds)
Target 1 (lockfree mixed):
mean of 1770262.062 r/s (2213888 rounds in 1.251 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1514651.598 r/s (1894400 rounds in 1.251 seconds)
Target 1 (lockfree mixed):
mean of 1163164.826 r/s (1456128 rounds in 1.252 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1295397.508 r/s (1622016 rounds in 1.252 seconds)
Target 1 (lockfree mixed):
mean of 1782592.276 r/s (2231296 rounds in 1.252 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1405281.265 r/s (1761280 rounds in 1.253 seconds)
Target 1 (lockfree mixed):
mean of 2128020.500 r/s (2910208 rounds in 1.368 seconds)

```

## MPSC CHANNEL
```
Mutexed VecDeque with 3 threads total time: 186.704975ms
Std's MPSC with 3 threads total time: 154.779724ms
Lockfree MPSC with 3 threads total time: 115.073683ms

Mutexed VecDeque with 5 threads total time: 451.980202ms
Std's MPSC with 5 threads total time: 255.353557ms
Lockfree MPSC with 5 threads total time: 196.244002ms

Mutexed VecDeque with 9 threads total time: 818.566102ms
Std's MPSC with 9 threads total time: 488.052613ms
Lockfree MPSC with 9 threads total time: 392.484694ms

Mutexed VecDeque with 17 threads total time: 1.666775743s
Std's MPSC with 17 threads total time: 981.020549ms
Lockfree MPSC with 17 threads total time: 740.592194ms

Mutexed VecDeque with 33 threads total time: 3.204210583s
Std's MPSC with 33 threads total time: 1.954984682s
Lockfree MPSC with 33 threads total time: 1.498478385s
```

## SPSC CHANNEL
```
Mutexed VecDeque total time: 560.975686ms
Lockfree SPSC total time: 371.035817ms
```

## SPMC CHANNEL
```
Mutexed VecDeque with 3 threads total time: 233.924558ms
Mutexed Std's MPSC (as SPMC) with 3 threads total time: 302.590365ms
Lockfree SPMC with 3 threads total time: 112.109708ms

Mutexed VecDeque with 5 threads total time: 351.265059ms
Mutexed Std's MPSC (as SPMC) with 5 threads total time: 154.962577ms
Lockfree SPMC with 5 threads total time: 102.275617ms

Mutexed VecDeque with 9 threads total time: 717.685753ms
Mutexed Std's MPSC (as SPMC) with 9 threads total time: 159.164441ms
Lockfree SPMC with 9 threads total time: 96.057705ms

Mutexed VecDeque with 17 threads total time: 1.194136926s
Mutexed Std's MPSC (as SPMC) with 17 threads total time: 166.250695ms
Lockfree SPMC with 17 threads total time: 218.314979ms

Mutexed VecDeque with 33 threads total time: 2.255403999s
Mutexed Std's MPSC (as SPMC) with 33 threads total time: 214.840607ms
Lockfree SPMC with 33 threads total time: 503.806924ms
```

## REQUEST PROGRAM
```
A program simulating a concurrent server.

Mutexed HashMap and VecDeque with 2 threads total time: 1.313416907s
Mutexed HashMap and LinkedList with 2 threads total time: 1.204615488s
Lockfree structures with 2 threads total time: 662.605373ms

Mutexed HashMap and VecDeque with 4 threads total time: 662.401274ms
Mutexed HashMap and LinkedList with 4 threads total time: 1.744702882s
Lockfree structures with 4 threads total time: 514.932636ms

Mutexed HashMap and VecDeque with 8 threads total time: 771.794807ms
Mutexed HashMap and LinkedList with 8 threads total time: 2.282478687s
Lockfree structures with 8 threads total time: 713.76397ms

Mutexed HashMap and VecDeque with 16 threads total time: 2.023066509s
Mutexed HashMap and LinkedList with 16 threads total time: 4.267969543s
Lockfree structures with 16 threads total time: 2.371225131s
```

## MESSAGE REVERB PROGRAM
```
A program which reverberates messages through a plain queue channel

Mutexed VecDeque with 2 threads total time: 185.982493ms
Mutexed LinkedList with 2 threads total time: 561.210787ms
Lockfree Queue with 2 threads total time: 203.646699ms

Mutexed VecDeque with 4 threads total time: 234.83501ms
Mutexed LinkedList with 4 threads total time: 456.900947ms
Lockfree Queue with 4 threads total time: 162.125759ms

Mutexed VecDeque with 8 threads total time: 235.183836ms
Mutexed LinkedList with 8 threads total time: 477.569274ms
Lockfree Queue with 8 threads total time: 157.417179ms

Mutexed VecDeque with 16 threads total time: 241.270436ms
Mutexed LinkedList with 16 threads total time: 478.632839ms
Lockfree Queue with 16 threads total time: 154.44212ms
```

## HASH MINING
```
A program simulating a hash miner.

Mutexed structures with 2 threads total time: 471.552033ms
Lockfree structures with 2 threads total time: 472.473517ms

Mutexed structures with 4 threads total time: 299.901006ms
Lockfree structures with 4 threads total time: 297.427489ms

Mutexed structures with 8 threads total time: 255.024035ms
Lockfree structures with 8 threads total time: 261.761415ms

Mutexed structures with 16 threads total time: 250.234684ms
Lockfree structures with 16 threads total time: 245.062467ms

Mutexed structures with 32 threads total time: 250.919829ms
Lockfree structures with 32 threads total time: 252.814198ms

Mutexed structures with 64 threads total time: 257.252161ms
Lockfree structures with 64 threads total time: 272.663432ms

Mutexed structures with 128 threads total time: 329.328633ms
Lockfree structures with 128 threads total time: 344.276903ms
```

