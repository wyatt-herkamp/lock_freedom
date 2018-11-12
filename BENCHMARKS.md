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
CPU MHz:             2688.379
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
mean of 439996701.704 r/s (550082560 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 127756533.831 r/s (159719424 rounds in 1.250 seconds)
Target 2 (blocking with cached access):
mean of 304921751.037 r/s (381210624 rounds in 1.250 seconds)
Target 3 (lockfree):
mean of 181350149.755 r/s (226720768 rounds in 1.250 seconds)
Target 4 (lockfree with cached id):
mean of 219770182.644 r/s (274729984 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 1224945884.325 r/s (1531323392 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 201229023.767 r/s (251586560 rounds in 1.250 seconds)
Target 2 (blocking with cached access):
mean of 341223729.511 r/s (426566656 rounds in 1.250 seconds)
Target 3 (lockfree):
mean of 297674075.950 r/s (372123648 rounds in 1.250 seconds)
Target 4 (lockfree with cached id):
mean of 352223047.353 r/s (440310784 rounds in 1.250 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 1225270530.924 r/s (1531914240 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 147757644.032 r/s (184740864 rounds in 1.250 seconds)
Target 2 (blocking with cached access):
mean of 200447918.131 r/s (250615808 rounds in 1.250 seconds)
Target 3 (lockfree):
mean of 297154990.744 r/s (371517440 rounds in 1.250 seconds)
Target 4 (lockfree with cached id):
mean of 354258071.640 r/s (442916864 rounds in 1.250 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 1225048605.567 r/s (1531884544 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 116848596.772 r/s (146136064 rounds in 1.251 seconds)
Target 2 (blocking with cached access):
mean of 147370955.762 r/s (184308736 rounds in 1.251 seconds)
Target 3 (lockfree):
mean of 297478089.858 r/s (372004864 rounds in 1.251 seconds)
Target 4 (lockfree with cached id):
mean of 351598284.478 r/s (439672832 rounds in 1.250 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 1225343141.606 r/s (1533903872 rounds in 1.252 seconds)
Target 1 (blocking):
mean of 57933618.599 r/s (72608768 rounds in 1.253 seconds)
Target 2 (blocking with cached access):
mean of 61029201.246 r/s (76510208 rounds in 1.254 seconds)
Target 3 (lockfree):
mean of 222842979.244 r/s (279026688 rounds in 1.252 seconds)
Target 4 (lockfree with cached id):
mean of 250437700.095 r/s (313570304 rounds in 1.252 seconds)

```

## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 46015655.325 r/s (57528320 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 29814419.810 r/s (37273600 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 16140601.079 r/s (20177920 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 13094609.327 r/s (16369664 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 3277649.297 r/s (4099072 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 5307029.878 r/s (6634496 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 8619090.988 r/s (10775552 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4920427.564 r/s (6152192 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 6010690.509 r/s (7517184 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8289746.076 r/s (10366976 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4896669.034 r/s (6123520 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 5770556.077 r/s (7325696 rounds in 1.269 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 8266261.190 r/s (10342400 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4888936.160 r/s (6119424 rounds in 1.252 seconds)
Target 2 (lockfree):
mean of 5805451.876 r/s (7414784 rounds in 1.277 seconds)

```

## STACK
```

Result for 1 threads:
Target 0 (mutex vector):
mean of 48986873.919 r/s (61244416 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 29675237.547 r/s (37100544 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 21691739.095 r/s (27116544 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 7412286.562 r/s (9266176 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4042805.425 r/s (5054464 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 6139324.119 r/s (7674880 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 7921003.623 r/s (9904128 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4892358.952 r/s (6117376 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 6543231.807 r/s (8180736 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 7536600.754 r/s (9423872 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4834714.703 r/s (6048768 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 6511226.745 r/s (8275968 rounds in 1.271 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 7526847.529 r/s (9414656 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4829790.237 r/s (6043648 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 6597524.706 r/s (8470528 rounds in 1.284 seconds)

```

## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 3250982.528 r/s (4520960 rounds in 1.391 seconds)
Target 1 (lockfree insert):
mean of 2033016.349 r/s (2542592 rounds in 1.251 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 2013402.746 r/s (2516992 rounds in 1.250 seconds)
Target 1 (lockfree insert):
mean of 2565858.110 r/s (3209216 rounds in 1.251 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2267267.796 r/s (2836480 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 3822905.746 r/s (4780032 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2474173.120 r/s (3097600 rounds in 1.252 seconds)
Target 1 (lockfree insert):
mean of 4038179.479 r/s (5111808 rounds in 1.266 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 6506281.408 r/s (8133632 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 4868434.356 r/s (6086656 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 2066407.149 r/s (2583552 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 8007253.694 r/s (10010624 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2210771.648 r/s (2765824 rounds in 1.251 seconds)
Target 1 (lockfree get):
mean of 10968202.899 r/s (13712384 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2643066.939 r/s (3308544 rounds in 1.252 seconds)
Target 1 (lockfree get):
mean of 11394788.961 r/s (14249984 rounds in 1.251 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 21359733.163 r/s (26701824 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 3471921.441 r/s (4340736 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 12195805.695 r/s (15246336 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 7329738.927 r/s (9164800 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10649608.678 r/s (13314048 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 11239535.279 r/s (14053376 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10525881.416 r/s (13161472 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 11565047.151 r/s (14461952 rounds in 1.250 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 4537476.746 r/s (5673984 rounds in 1.250 seconds)
Target 1 (lockfree mixed):
mean of 1906871.465 r/s (2384896 rounds in 1.251 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1493854.733 r/s (1868800 rounds in 1.251 seconds)
Target 1 (lockfree mixed):
mean of 1181886.322 r/s (1478656 rounds in 1.251 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1304850.144 r/s (1633280 rounds in 1.252 seconds)
Target 1 (lockfree mixed):
mean of 1839983.331 r/s (2302976 rounds in 1.252 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1420402.399 r/s (1778688 rounds in 1.252 seconds)
Target 1 (lockfree mixed):
mean of 2388197.548 r/s (3117056 rounds in 1.305 seconds)

```

## MPSC CHANNEL
```
Mutexed VecDeque with 3 threads total time: 197.079469ms
Std's MPSC with 3 threads total time: 144.212637ms
Lockfree MPSC with 3 threads total time: 113.81897ms

Mutexed VecDeque with 5 threads total time: 454.925931ms
Std's MPSC with 5 threads total time: 244.289176ms
Lockfree MPSC with 5 threads total time: 202.734832ms

Mutexed VecDeque with 9 threads total time: 792.820312ms
Std's MPSC with 9 threads total time: 477.359992ms
Lockfree MPSC with 9 threads total time: 384.951569ms

Mutexed VecDeque with 17 threads total time: 1.611498162s
Std's MPSC with 17 threads total time: 993.83529ms
Lockfree MPSC with 17 threads total time: 768.035555ms

Mutexed VecDeque with 33 threads total time: 3.205025185s
Std's MPSC with 33 threads total time: 1.959206533s
Lockfree MPSC with 33 threads total time: 1.659302342s
```

## SPSC CHANNEL
```
Mutexed VecDeque total time: 670.12071ms
Std's MPSC (as SPSC) total time: 86.914052ms
Lockfree SPSC total time: 390.730588ms
```

## SPMC CHANNEL
```
Mutexed VecDeque with 3 threads total time: 254.390952ms
Mutexed Std's MPSC (as SPMC) with 3 threads total time: 303.356186ms
Lockfree SPMC with 3 threads total time: 111.200147ms

Mutexed VecDeque with 5 threads total time: 338.718028ms
Mutexed Std's MPSC (as SPMC) with 5 threads total time: 155.657281ms
Lockfree SPMC with 5 threads total time: 92.951321ms

Mutexed VecDeque with 9 threads total time: 733.008737ms
Mutexed Std's MPSC (as SPMC) with 9 threads total time: 170.675857ms
Lockfree SPMC with 9 threads total time: 96.502531ms

Mutexed VecDeque with 17 threads total time: 1.139339538s
Mutexed Std's MPSC (as SPMC) with 17 threads total time: 180.856715ms
Lockfree SPMC with 17 threads total time: 206.57024ms

Mutexed VecDeque with 33 threads total time: 2.313293422s
Mutexed Std's MPSC (as SPMC) with 33 threads total time: 229.593564ms
Lockfree SPMC with 33 threads total time: 306.615097ms
```

## MPMC CHANNEL
```
Mutexed VecDeque with 4 threads total time: 83.372872ms
Mutexed Std's MPSC (as MPMC)  with 4 threads total time: 142.997602ms
Lockfree MPMC with 4 threads total time: 42.667375ms

Mutexed VecDeque with 8 threads total time: 175.741776ms
Mutexed Std's MPSC (as MPMC)  with 8 threads total time: 197.728613ms
Lockfree MPMC with 8 threads total time: 87.067142ms

Mutexed VecDeque with 16 threads total time: 395.062862ms
Mutexed Std's MPSC (as MPMC)  with 16 threads total time: 420.86186ms
Lockfree MPMC with 16 threads total time: 274.093888ms
```

## REQUEST PROGRAM
```
A program simulating a concurrent server.

Mutexed HashMap and Std's MPSC with 2 threads total time: 816.253703ms
Lockfree structures with 2 threads total time: 480.213339ms

Mutexed HashMap and Std's MPSC with 4 threads total time: 366.846145ms
Lockfree structures with 4 threads total time: 393.596918ms

Mutexed HashMap and Std's MPSC with 8 threads total time: 367.480964ms
Lockfree structures with 8 threads total time: 347.391214ms

Mutexed HashMap and Std's MPSC with 16 threads total time: 334.420038ms
Lockfree structures with 16 threads total time: 405.705754ms
```

## MESSAGE REVERB PROGRAM
```
A program which reverberates messages through a plain queue channel

Mutexed VecDeque with 2 threads total time: 213.789081ms
Mutexed LinkedList with 2 threads total time: 600.177552ms
Lockfree Queue with 2 threads total time: 200.311577ms

Mutexed VecDeque with 4 threads total time: 237.96143ms
Mutexed LinkedList with 4 threads total time: 460.737892ms
Lockfree Queue with 4 threads total time: 162.206144ms

Mutexed VecDeque with 8 threads total time: 249.180902ms
Mutexed LinkedList with 8 threads total time: 481.548234ms
Lockfree Queue with 8 threads total time: 152.765921ms

Mutexed VecDeque with 16 threads total time: 243.548249ms
Mutexed LinkedList with 16 threads total time: 496.646642ms
Lockfree Queue with 16 threads total time: 150.581607ms
```

## HASH MINING
```
A program simulating a hash miner.

Mutexed structures with 2 threads total time: 475.381656ms
Lockfree structures with 2 threads total time: 463.806961ms

Mutexed structures with 4 threads total time: 311.415609ms
Lockfree structures with 4 threads total time: 303.829533ms

Mutexed structures with 8 threads total time: 258.355101ms
Lockfree structures with 8 threads total time: 241.733968ms

Mutexed structures with 16 threads total time: 250.028158ms
Lockfree structures with 16 threads total time: 248.885735ms

Mutexed structures with 32 threads total time: 246.846169ms
Lockfree structures with 32 threads total time: 253.05098ms

Mutexed structures with 64 threads total time: 254.661183ms
Lockfree structures with 64 threads total time: 264.923939ms

Mutexed structures with 128 threads total time: 320.986229ms
Lockfree structures with 128 threads total time: 336.612203ms
```

