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
CPU MHz:             600.020
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
mean of 550314100.068 r/s (688011264 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 137448139.412 r/s (171824128 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 166690453.258 r/s (208395264 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 1249037584.392 r/s (1561468928 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 190042784.254 r/s (237575168 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 280105347.942 r/s (350174208 rounds in 1.250 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 1260614795.183 r/s (1576158208 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 122168514.603 r/s (152751104 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 281152427.057 r/s (351526912 rounds in 1.250 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 1247882916.524 r/s (1560463360 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 84239965.304 r/s (105361408 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 280362446.827 r/s (350604288 rounds in 1.251 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 1247437959.126 r/s (1561871360 rounds in 1.252 seconds)
Target 1 (blocking):
mean of 37533405.337 r/s (47099904 rounds in 1.255 seconds)
Target 2 (lockfree):
mean of 248332400.976 r/s (311204864 rounds in 1.253 seconds)

```

## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 45552880.308 r/s (56951808 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 29275313.698 r/s (36600832 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 12418402.534 r/s (15525888 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 12607979.992 r/s (15761408 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 2896184.358 r/s (3620864 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 4902087.645 r/s (6128640 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 8760794.554 r/s (10952704 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 5009021.805 r/s (6263808 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 5443439.774 r/s (6806528 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8383112.058 r/s (10482688 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4923812.508 r/s (6158336 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 5578679.098 r/s (6980608 rounds in 1.251 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 8336555.466 r/s (10430464 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4880735.746 r/s (6108160 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 5389242.248 r/s (6804480 rounds in 1.263 seconds)

```

## STACK
```

Result for 1 threads:
Target 0 (mutex vector):
mean of 48577047.825 r/s (60732416 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 27744577.917 r/s (34685952 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 20639569.574 r/s (25804800 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 8607419.849 r/s (10761216 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 3698171.729 r/s (4623360 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5771744.376 r/s (7216128 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 8096787.952 r/s (10124288 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 5044733.376 r/s (6308864 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 6549565.198 r/s (8189952 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 7674340.696 r/s (9596928 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4927836.633 r/s (6164480 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 6554204.060 r/s (8203264 rounds in 1.252 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 7679492.012 r/s (9608192 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4898336.355 r/s (6132736 rounds in 1.252 seconds)
Target 2 (lockfree):
mean of 6628308.486 r/s (8321024 rounds in 1.255 seconds)

```

## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 3264892.474 r/s (4520960 rounds in 1.385 seconds)
Target 1 (lockfree insert):
mean of 1925447.291 r/s (2407424 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 2000106.026 r/s (2501632 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 2231860.460 r/s (2791424 rounds in 1.251 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2038633.053 r/s (2549760 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 3611478.285 r/s (4517888 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2337748.720 r/s (2925568 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 3363683.255 r/s (4574208 rounds in 1.360 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 6459717.641 r/s (8076288 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 4634553.553 r/s (5794816 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 2030013.192 r/s (2538496 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 7971898.817 r/s (9966592 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2188807.788 r/s (2738176 rounds in 1.251 seconds)
Target 1 (lockfree get):
mean of 11295260.340 r/s (14124032 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2449805.367 r/s (3083264 rounds in 1.259 seconds)
Target 1 (lockfree get):
mean of 10862984.669 r/s (13583360 rounds in 1.250 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 21000161.526 r/s (26252288 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 1621467.114 r/s (2027520 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 12146657.513 r/s (15185920 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 3878320.435 r/s (4849664 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10447343.310 r/s (13062144 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 10555211.667 r/s (13197312 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10356864.966 r/s (12950528 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 11057368.067 r/s (13827072 rounds in 1.250 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 4526699.816 r/s (5659648 rounds in 1.250 seconds)
Target 1 (lockfree mixed):
mean of 1642990.059 r/s (2055168 rounds in 1.251 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1512593.277 r/s (1891328 rounds in 1.250 seconds)
Target 1 (lockfree mixed):
mean of 1068043.220 r/s (1337344 rounds in 1.252 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1296320.862 r/s (1622016 rounds in 1.251 seconds)
Target 1 (lockfree mixed):
mean of 1761700.171 r/s (2204672 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1394915.412 r/s (1747968 rounds in 1.253 seconds)
Target 1 (lockfree mixed):
mean of 1636045.971 r/s (2382848 rounds in 1.456 seconds)

```

## MPSC CHANNEL
```
Mutexed VecDeque with 3 threads total time: 227.859073ms
Std's MPSC with 3 threads total time: 157.97358ms
Lockfree MPSC with 3 threads total time: 117.150722ms

Mutexed VecDeque with 5 threads total time: 430.402011ms
Std's MPSC with 5 threads total time: 246.193938ms
Lockfree MPSC with 5 threads total time: 205.278326ms

Mutexed VecDeque with 9 threads total time: 805.887431ms
Std's MPSC with 9 threads total time: 475.614051ms
Lockfree MPSC with 9 threads total time: 383.841494ms

Mutexed VecDeque with 17 threads total time: 1.525882484s
Std's MPSC with 17 threads total time: 1.017100947s
Lockfree MPSC with 17 threads total time: 824.999418ms

Mutexed VecDeque with 33 threads total time: 3.123336868s
Std's MPSC with 33 threads total time: 2.051523703s
Lockfree MPSC with 33 threads total time: 1.572551171s
```

## SPSC CHANNEL
```
Mutexed VecDeque total time: 518.955144ms
Lockfree SPSC total time: 353.111947ms
```

## SPMC CHANNEL
```
Mutexed VecDeque with 3 threads total time: 237.113022ms
Mutexed Std's MPSC (as SPMC) with 3 threads total time: 292.187214ms
Lockfree MPSC with 3 threads total time: 126.785641ms

Mutexed VecDeque with 5 threads total time: 292.224841ms
Mutexed Std's MPSC (as SPMC) with 5 threads total time: 159.608685ms
Lockfree MPSC with 5 threads total time: 119.143623ms

Mutexed VecDeque with 9 threads total time: 700.447752ms
Mutexed Std's MPSC (as SPMC) with 9 threads total time: 157.52326ms
Lockfree MPSC with 9 threads total time: 122.275017ms

Mutexed VecDeque with 17 threads total time: 1.201897824s
Mutexed Std's MPSC (as SPMC) with 17 threads total time: 162.660037ms
Lockfree MPSC with 17 threads total time: 286.911461ms

Mutexed VecDeque with 33 threads total time: 2.30292555s
Mutexed Std's MPSC (as SPMC) with 33 threads total time: 196.766663ms
Lockfree MPSC with 33 threads total time: 535.988106ms
```

## REQUEST PROGRAM
```
A program simulating a concurrent server.

Mutexed HashMap and VecDeque with 2 threads total time: 1.293299263s
Mutexed HashMap and LinkedList with 2 threads total time: 1.203508856s
Lockfree structures with 2 threads total time: 737.702389ms

Mutexed HashMap and VecDeque with 4 threads total time: 792.961634ms
Mutexed HashMap and LinkedList with 4 threads total time: 2.069625115s
Lockfree structures with 4 threads total time: 768.700934ms

Mutexed HashMap and VecDeque with 8 threads total time: 929.3105ms
Mutexed HashMap and LinkedList with 8 threads total time: 2.316328106s
Lockfree structures with 8 threads total time: 1.089739579s

Mutexed HashMap and VecDeque with 16 threads total time: 2.394597715s
Mutexed HashMap and LinkedList with 16 threads total time: 4.708013296s
Lockfree structures with 16 threads total time: 2.270930803s
```

## MESSAGE REVERB PROGRAM
```
A program which reverberates messages through a plain queue channel

Mutexed VecDeque with 2 threads total time: 310.695653ms
Mutexed LinkedList with 2 threads total time: 567.607805ms
Lockfree Queue with 2 threads total time: 197.024749ms

Mutexed VecDeque with 4 threads total time: 228.097571ms
Mutexed LinkedList with 4 threads total time: 440.886408ms
Lockfree Queue with 4 threads total time: 187.035188ms

Mutexed VecDeque with 8 threads total time: 242.723549ms
Mutexed LinkedList with 8 threads total time: 466.649083ms
Lockfree Queue with 8 threads total time: 187.765736ms

Mutexed VecDeque with 16 threads total time: 248.289151ms
Mutexed LinkedList with 16 threads total time: 486.993132ms
Lockfree Queue with 16 threads total time: 180.975722ms
```

## HASH MINING
```
A program simulating a hash miner.

Mutexed structures with 2 threads total time: 465.359102ms
Lockfree structures with 2 threads total time: 466.429334ms

Mutexed structures with 4 threads total time: 306.294297ms
Lockfree structures with 4 threads total time: 298.698132ms

Mutexed structures with 8 threads total time: 246.271886ms
Lockfree structures with 8 threads total time: 245.935169ms

Mutexed structures with 16 threads total time: 257.367011ms
Lockfree structures with 16 threads total time: 245.139593ms

Mutexed structures with 32 threads total time: 247.941298ms
Lockfree structures with 32 threads total time: 245.173492ms

Mutexed structures with 64 threads total time: 261.171563ms
Lockfree structures with 64 threads total time: 272.955523ms

Mutexed structures with 128 threads total time: 331.354231ms
Lockfree structures with 128 threads total time: 343.126615ms
```

