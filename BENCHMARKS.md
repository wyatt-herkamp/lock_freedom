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
CPU MHz:             2754.716
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
mean of 439931475.266 r/s (549999616 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 127727769.072 r/s (159670272 rounds in 1.250 seconds)
Target 2 (blocking with cached access):
mean of 303670840.235 r/s (379644928 rounds in 1.250 seconds)
Target 3 (lockfree):
mean of 181404834.870 r/s (226789376 rounds in 1.250 seconds)
Target 4 (lockfree with cached id):
mean of 219727453.410 r/s (274716672 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 1224136219.654 r/s (1530316800 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 198207209.447 r/s (247783424 rounds in 1.250 seconds)
Target 2 (blocking with cached access):
mean of 337728047.408 r/s (422178816 rounds in 1.250 seconds)
Target 3 (lockfree):
mean of 362241952.158 r/s (452845568 rounds in 1.250 seconds)
Target 4 (lockfree with cached id):
mean of 425592602.244 r/s (532030464 rounds in 1.250 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 1225358597.414 r/s (1532000256 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 150566224.246 r/s (188249088 rounds in 1.250 seconds)
Target 2 (blocking with cached access):
mean of 199342011.356 r/s (249238528 rounds in 1.250 seconds)
Target 3 (lockfree):
mean of 328944266.169 r/s (411259904 rounds in 1.250 seconds)
Target 4 (lockfree with cached id):
mean of 394966622.869 r/s (493808640 rounds in 1.250 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 1225548685.734 r/s (1532512256 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 115422931.293 r/s (144347136 rounds in 1.251 seconds)
Target 2 (blocking with cached access):
mean of 152091311.863 r/s (190204928 rounds in 1.251 seconds)
Target 3 (lockfree):
mean of 307625468.617 r/s (384692224 rounds in 1.251 seconds)
Target 4 (lockfree with cached id):
mean of 364701154.030 r/s (456032256 rounds in 1.250 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 1224406677.268 r/s (1532834816 rounds in 1.252 seconds)
Target 1 (blocking):
mean of 58528117.432 r/s (73352192 rounds in 1.253 seconds)
Target 2 (blocking with cached access):
mean of 61288407.174 r/s (76822528 rounds in 1.253 seconds)
Target 3 (lockfree):
mean of 256092543.615 r/s (320616448 rounds in 1.252 seconds)
Target 4 (lockfree with cached id):
mean of 296707281.796 r/s (371459072 rounds in 1.252 seconds)

```

## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 45618346.219 r/s (57031680 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 29993720.164 r/s (37497856 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 15757441.167 r/s (19699712 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 12539383.900 r/s (15675392 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4028871.856 r/s (5037056 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5280751.400 r/s (6602752 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 8296939.434 r/s (10374144 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4969355.813 r/s (6212608 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 6031760.982 r/s (7542784 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8073497.524 r/s (10096640 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4900091.582 r/s (6128640 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 5799468.308 r/s (7396352 rounds in 1.275 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 8034819.069 r/s (10050560 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4905151.077 r/s (6138880 rounds in 1.252 seconds)
Target 2 (lockfree):
mean of 5809043.004 r/s (7347200 rounds in 1.265 seconds)

```

## STACK
```

Result for 1 threads:
Target 0 (mutex vector):
mean of 48980722.970 r/s (61236224 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 29273939.415 r/s (36597760 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 21267637.358 r/s (26589184 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 7517133.819 r/s (9397248 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4903074.961 r/s (6129664 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 6207464.859 r/s (7760896 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 7763214.522 r/s (9706496 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4920401.186 r/s (6152192 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 6646290.801 r/s (8309760 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 7436814.860 r/s (9299968 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4883057.927 r/s (6108160 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 6737735.339 r/s (8536064 rounds in 1.267 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 7454379.635 r/s (9325568 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4861737.029 r/s (6086656 rounds in 1.252 seconds)
Target 2 (lockfree):
mean of 6750515.983 r/s (8574976 rounds in 1.270 seconds)

```

## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 3266411.392 r/s (4520960 rounds in 1.384 seconds)
Target 1 (lockfree insert):
mean of 2027926.579 r/s (2536448 rounds in 1.251 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 2002356.682 r/s (2503680 rounds in 1.250 seconds)
Target 1 (lockfree insert):
mean of 2570607.551 r/s (3215360 rounds in 1.251 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2302816.818 r/s (2879488 rounds in 1.250 seconds)
Target 1 (lockfree insert):
mean of 3830796.511 r/s (4792320 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2507057.854 r/s (3138560 rounds in 1.252 seconds)
Target 1 (lockfree insert):
mean of 3924293.423 r/s (5056512 rounds in 1.289 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 6523171.133 r/s (8156160 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 5072463.514 r/s (6341632 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 2031851.490 r/s (2540544 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 7940770.615 r/s (9927680 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2325072.078 r/s (2940928 rounds in 1.265 seconds)
Target 1 (lockfree get):
mean of 11153774.758 r/s (14395392 rounds in 1.291 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2630553.645 r/s (3293184 rounds in 1.252 seconds)
Target 1 (lockfree get):
mean of 11535754.434 r/s (14592000 rounds in 1.265 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 21129965.487 r/s (26417152 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 3167575.191 r/s (4493312 rounds in 1.419 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 12195075.510 r/s (15246336 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 7242906.182 r/s (9056256 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10810149.323 r/s (13515776 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 11446988.906 r/s (14311424 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10727059.054 r/s (13414400 rounds in 1.251 seconds)
Target 1 (lockfree remove):
mean of 11589376.360 r/s (14492672 rounds in 1.251 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 4540358.727 r/s (5677056 rounds in 1.250 seconds)
Target 1 (lockfree mixed):
mean of 1888429.566 r/s (2361344 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1546695.282 r/s (1935360 rounds in 1.251 seconds)
Target 1 (lockfree mixed):
mean of 1172581.401 r/s (1468416 rounds in 1.252 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1316778.930 r/s (1648640 rounds in 1.252 seconds)
Target 1 (lockfree mixed):
mean of 1869245.395 r/s (2338816 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1425762.564 r/s (1783808 rounds in 1.251 seconds)
Target 1 (lockfree mixed):
mean of 1868992.837 r/s (2623488 rounds in 1.404 seconds)

```

## MPSC CHANNEL
```
Mutexed VecDeque with 3 threads total time: 200.333147ms
Std's MPSC with 3 threads total time: 144.006513ms
Lockfree MPSC with 3 threads total time: 115.120885ms

Mutexed VecDeque with 5 threads total time: 428.717704ms
Std's MPSC with 5 threads total time: 257.948787ms
Lockfree MPSC with 5 threads total time: 200.414987ms

Mutexed VecDeque with 9 threads total time: 844.824703ms
Std's MPSC with 9 threads total time: 490.235268ms
Lockfree MPSC with 9 threads total time: 392.578682ms

Mutexed VecDeque with 17 threads total time: 1.65963477s
Std's MPSC with 17 threads total time: 982.825303ms
Lockfree MPSC with 17 threads total time: 758.474964ms

Mutexed VecDeque with 33 threads total time: 3.268112487s
Std's MPSC with 33 threads total time: 2.034911661s
Lockfree MPSC with 33 threads total time: 1.632807642s
```

## SPSC CHANNEL
```
Mutexed VecDeque total time: 781.044289ms
Std's MPSC (as SPSC) total time: 104.731403ms
Lockfree SPSC total time: 374.180293ms
```

## SPMC CHANNEL
```
Mutexed VecDeque with 3 threads total time: 226.163631ms
Mutexed Std's MPSC (as SPMC) with 3 threads total time: 308.076533ms
Lockfree SPMC with 3 threads total time: 111.00929ms

Mutexed VecDeque with 5 threads total time: 330.176627ms
Mutexed Std's MPSC (as SPMC) with 5 threads total time: 157.038995ms
Lockfree SPMC with 5 threads total time: 110.675001ms

Mutexed VecDeque with 9 threads total time: 706.84683ms
Mutexed Std's MPSC (as SPMC) with 9 threads total time: 170.836373ms
Lockfree SPMC with 9 threads total time: 99.52314ms

Mutexed VecDeque with 17 threads total time: 1.100380191s
Mutexed Std's MPSC (as SPMC) with 17 threads total time: 169.108907ms
Lockfree SPMC with 17 threads total time: 246.592692ms

Mutexed VecDeque with 33 threads total time: 2.201868371s
Mutexed Std's MPSC (as SPMC) with 33 threads total time: 265.56603ms
Lockfree SPMC with 33 threads total time: 466.509421ms
```

## MPMC CHANNEL
```
Mutexed VecDeque with 4 threads total time: 84.823881ms
Mutexed Std's MPSC (as MPMC)  with 4 threads total time: 127.813253ms
Lockfree MPMC with 4 threads total time: 53.432497ms

Mutexed VecDeque with 8 threads total time: 183.250031ms
Mutexed Std's MPSC (as MPMC)  with 8 threads total time: 198.296799ms
Lockfree MPMC with 8 threads total time: 103.716406ms

Mutexed VecDeque with 16 threads total time: 391.374687ms
Mutexed Std's MPSC (as MPMC)  with 16 threads total time: 422.227762ms
Lockfree MPMC with 16 threads total time: 353.769535ms
```

## REQUEST PROGRAM
```
A program simulating a concurrent server.

Mutexed HashMap and Std's MPSC with 2 threads total time: 843.455533ms
Lockfree structures with 2 threads total time: 465.319853ms

Mutexed HashMap and Std's MPSC with 4 threads total time: 375.562276ms
Lockfree structures with 4 threads total time: 388.560535ms

Mutexed HashMap and Std's MPSC with 8 threads total time: 399.367066ms
Lockfree structures with 8 threads total time: 396.940674ms

Mutexed HashMap and Std's MPSC with 16 threads total time: 380.445258ms
Lockfree structures with 16 threads total time: 359.028539ms
```

## MESSAGE REVERB PROGRAM
```
A program which reverberates messages through a plain queue channel

Mutexed VecDeque with 2 threads total time: 205.993123ms
Mutexed LinkedList with 2 threads total time: 534.761405ms
Lockfree Queue with 2 threads total time: 189.622453ms

Mutexed VecDeque with 4 threads total time: 236.763ms
Mutexed LinkedList with 4 threads total time: 459.672301ms
Lockfree Queue with 4 threads total time: 162.816376ms

Mutexed VecDeque with 8 threads total time: 245.048533ms
Mutexed LinkedList with 8 threads total time: 485.007637ms
Lockfree Queue with 8 threads total time: 153.793501ms

Mutexed VecDeque with 16 threads total time: 241.895906ms
Mutexed LinkedList with 16 threads total time: 486.872974ms
Lockfree Queue with 16 threads total time: 152.179073ms
```

## HASH MINING
```
A program simulating a hash miner.

Mutexed structures with 2 threads total time: 471.309232ms
Lockfree structures with 2 threads total time: 459.030602ms

Mutexed structures with 4 threads total time: 327.793323ms
Lockfree structures with 4 threads total time: 314.350874ms

Mutexed structures with 8 threads total time: 263.449119ms
Lockfree structures with 8 threads total time: 258.892959ms

Mutexed structures with 16 threads total time: 246.58946ms
Lockfree structures with 16 threads total time: 242.66801ms

Mutexed structures with 32 threads total time: 255.998852ms
Lockfree structures with 32 threads total time: 246.931385ms

Mutexed structures with 64 threads total time: 250.61187ms
Lockfree structures with 64 threads total time: 267.688039ms

Mutexed structures with 128 threads total time: 329.051589ms
Lockfree structures with 128 threads total time: 338.430017ms
```

