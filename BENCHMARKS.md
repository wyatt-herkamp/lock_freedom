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
CPU MHz:             800.180
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
mean of 431588950.125 r/s (539569152 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 127625990.530 r/s (159555584 rounds in 1.250 seconds)
Target 2 (blocking with cached access):
mean of 312220364.851 r/s (390301696 rounds in 1.250 seconds)
Target 3 (lockfree):
mean of 144608658.358 r/s (180787200 rounds in 1.250 seconds)
Target 4 (lockfree with cached id):
mean of 191692216.717 r/s (239650816 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 1167261870.816 r/s (1459196928 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 201381752.631 r/s (251764736 rounds in 1.250 seconds)
Target 2 (blocking with cached access):
mean of 341172714.926 r/s (426504192 rounds in 1.250 seconds)
Target 3 (lockfree):
mean of 292943768.231 r/s (366209024 rounds in 1.250 seconds)
Target 4 (lockfree with cached id):
mean of 406633570.932 r/s (508328960 rounds in 1.250 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 1221684371.386 r/s (1527369728 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 149489751.528 r/s (186897408 rounds in 1.250 seconds)
Target 2 (blocking with cached access):
mean of 179739505.670 r/s (224719872 rounds in 1.250 seconds)
Target 3 (lockfree):
mean of 292911007.342 r/s (366200832 rounds in 1.250 seconds)
Target 4 (lockfree with cached id):
mean of 409465540.712 r/s (511925248 rounds in 1.250 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 1224070290.515 r/s (1530544128 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 118428183.766 r/s (148102144 rounds in 1.251 seconds)
Target 2 (blocking with cached access):
mean of 132472748.451 r/s (165660672 rounds in 1.251 seconds)
Target 3 (lockfree):
mean of 292817494.229 r/s (366164992 rounds in 1.250 seconds)
Target 4 (lockfree with cached id):
mean of 409359234.536 r/s (511880192 rounds in 1.250 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 1222474759.591 r/s (1529976832 rounds in 1.252 seconds)
Target 1 (blocking):
mean of 57450207.886 r/s (71995392 rounds in 1.253 seconds)
Target 2 (blocking with cached access):
mean of 61348301.775 r/s (76881920 rounds in 1.253 seconds)
Target 3 (lockfree):
mean of 292841103.441 r/s (366601216 rounds in 1.252 seconds)
Target 4 (lockfree with cached id):
mean of 409246071.417 r/s (512243712 rounds in 1.252 seconds)

```

## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 45029088.061 r/s (56291328 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 29987978.456 r/s (37487616 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 15492227.439 r/s (19368960 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 11648442.405 r/s (14562304 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 3591955.688 r/s (4491264 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5428106.239 r/s (6787072 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 8676068.627 r/s (10847232 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 5061044.459 r/s (6327296 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 6123383.368 r/s (7656448 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8357926.753 r/s (10451968 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4993742.342 r/s (6245376 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 5932319.714 r/s (7462912 rounds in 1.258 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 8346459.804 r/s (10442752 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4970480.001 r/s (6220800 rounds in 1.252 seconds)
Target 2 (lockfree):
mean of 5554866.699 r/s (7083008 rounds in 1.275 seconds)

```

## STACK
```

Result for 1 threads:
Target 0 (mutex vector):
mean of 47379921.752 r/s (59235328 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 29478468.368 r/s (36854784 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 20576810.231 r/s (25724928 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 8054816.544 r/s (10070016 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 3629537.412 r/s (4537344 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 6017024.115 r/s (7522304 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 7794579.840 r/s (9745408 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4916129.234 r/s (6147072 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 6498619.977 r/s (8125440 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 7483989.574 r/s (9358336 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4848574.748 r/s (6066176 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 6471126.200 r/s (8146944 rounds in 1.259 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 7484094.663 r/s (9363456 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4843951.456 r/s (6061056 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 6395472.988 r/s (8251392 rounds in 1.290 seconds)

```

## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 3253553.661 r/s (4520960 rounds in 1.390 seconds)
Target 1 (lockfree insert):
mean of 2028221.336 r/s (2536448 rounds in 1.251 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 2019016.034 r/s (2525184 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 2503941.603 r/s (3131392 rounds in 1.251 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2315475.712 r/s (2895872 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 3792081.785 r/s (4743168 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2504575.331 r/s (3134464 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 3640440.160 r/s (4634624 rounds in 1.273 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 6539901.359 r/s (8176640 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 4760705.388 r/s (5952512 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 2052973.059 r/s (2567168 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 7391833.101 r/s (9242624 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2397423.651 r/s (3016704 rounds in 1.258 seconds)
Target 1 (lockfree get):
mean of 12208304.129 r/s (15386624 rounds in 1.260 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2536631.544 r/s (3176448 rounds in 1.252 seconds)
Target 1 (lockfree get):
mean of 12060428.088 r/s (15081472 rounds in 1.250 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 21231228.673 r/s (26543104 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 3741167.061 r/s (4677632 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 12172236.100 r/s (15217664 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 7218554.932 r/s (9025536 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10702819.232 r/s (13381632 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 12455243.705 r/s (15571968 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 9363968.655 r/s (13359104 rounds in 1.427 seconds)
Target 1 (lockfree remove):
mean of 11910561.246 r/s (14896128 rounds in 1.251 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 4550772.226 r/s (5689344 rounds in 1.250 seconds)
Target 1 (lockfree mixed):
mean of 1884923.506 r/s (2357248 rounds in 1.251 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1552749.595 r/s (1941504 rounds in 1.250 seconds)
Target 1 (lockfree mixed):
mean of 1294768.527 r/s (1619968 rounds in 1.251 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1302788.548 r/s (1630208 rounds in 1.251 seconds)
Target 1 (lockfree mixed):
mean of 1866459.277 r/s (2334720 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1417831.129 r/s (1776640 rounds in 1.253 seconds)
Target 1 (lockfree mixed):
mean of 1647170.899 r/s (2360320 rounds in 1.433 seconds)

```

## MPSC CHANNEL
```
Mutexed VecDeque with 3 threads total time: 217.096114ms
Std's MPSC with 3 threads total time: 158.555172ms
Lockfree MPSC with 3 threads total time: 112.5009ms

Mutexed VecDeque with 5 threads total time: 427.441763ms
Std's MPSC with 5 threads total time: 254.071412ms
Lockfree MPSC with 5 threads total time: 188.851161ms

Mutexed VecDeque with 9 threads total time: 828.648994ms
Std's MPSC with 9 threads total time: 495.932602ms
Lockfree MPSC with 9 threads total time: 375.025288ms

Mutexed VecDeque with 17 threads total time: 1.664373026s
Std's MPSC with 17 threads total time: 1.008476483s
Lockfree MPSC with 17 threads total time: 750.481657ms

Mutexed VecDeque with 33 threads total time: 3.204942575s
Std's MPSC with 33 threads total time: 1.983592098s
Lockfree MPSC with 33 threads total time: 1.52262644s
```

## SPSC CHANNEL
```
Mutexed VecDeque total time: 693.329644ms
Std's MPSC (as SPSC) total time: 95.70451ms
Lockfree SPSC total time: 369.255192ms
```

## SPMC CHANNEL
```
Mutexed VecDeque with 3 threads total time: 237.702211ms
Mutexed Std's MPSC (as SPMC) with 3 threads total time: 298.260844ms
Lockfree SPMC with 3 threads total time: 124.842105ms

Mutexed VecDeque with 5 threads total time: 389.09159ms
Mutexed Std's MPSC (as SPMC) with 5 threads total time: 161.016268ms
Lockfree SPMC with 5 threads total time: 124.830331ms

Mutexed VecDeque with 9 threads total time: 660.208909ms
Mutexed Std's MPSC (as SPMC) with 9 threads total time: 168.732966ms
Lockfree SPMC with 9 threads total time: 112.976193ms

Mutexed VecDeque with 17 threads total time: 1.084651462s
Mutexed Std's MPSC (as SPMC) with 17 threads total time: 178.014993ms
Lockfree SPMC with 17 threads total time: 171.221207ms

Mutexed VecDeque with 33 threads total time: 2.229750031s
Mutexed Std's MPSC (as SPMC) with 33 threads total time: 198.823554ms
Lockfree SPMC with 33 threads total time: 608.840693ms
```

## MPMC CHANNEL
```
Mutexed VecDeque with 4 threads total time: 81.549383ms
Mutexed Std's MPSC (as MPMC)  with 4 threads total time: 139.662304ms
Lockfree MPMC with 4 threads total time: 52.892947ms

Mutexed VecDeque with 8 threads total time: 185.548713ms
Mutexed Std's MPSC (as MPMC)  with 8 threads total time: 184.709444ms
Lockfree MPMC with 8 threads total time: 100.958789ms

Mutexed VecDeque with 16 threads total time: 388.793245ms
Mutexed Std's MPSC (as MPMC)  with 16 threads total time: 418.191397ms
Lockfree MPMC with 16 threads total time: 350.3493ms
```

## REQUEST PROGRAM
```
A program simulating a concurrent server.

Mutexed HashMap and Std's MPSC with 2 threads total time: 799.521487ms
Lockfree structures with 2 threads total time: 516.453484ms

Mutexed HashMap and Std's MPSC with 4 threads total time: 387.831395ms
Lockfree structures with 4 threads total time: 394.20069ms

Mutexed HashMap and Std's MPSC with 8 threads total time: 391.78227ms
Lockfree structures with 8 threads total time: 397.745513ms

Mutexed HashMap and Std's MPSC with 16 threads total time: 354.907846ms
Lockfree structures with 16 threads total time: 365.321654ms
```

## MESSAGE REVERB PROGRAM
```
A program which reverberates messages through a plain queue channel

Mutexed VecDeque with 2 threads total time: 217.824639ms
Mutexed LinkedList with 2 threads total time: 573.624218ms
Lockfree Queue with 2 threads total time: 202.8205ms

Mutexed VecDeque with 4 threads total time: 238.153249ms
Mutexed LinkedList with 4 threads total time: 479.385588ms
Lockfree Queue with 4 threads total time: 162.056346ms

Mutexed VecDeque with 8 threads total time: 249.550548ms
Mutexed LinkedList with 8 threads total time: 489.274461ms
Lockfree Queue with 8 threads total time: 154.400015ms

Mutexed VecDeque with 16 threads total time: 241.968781ms
Mutexed LinkedList with 16 threads total time: 501.281671ms
Lockfree Queue with 16 threads total time: 150.8722ms
```

## HASH MINING
```
A program simulating a hash miner.

Mutexed structures with 2 threads total time: 448.07129ms
Lockfree structures with 2 threads total time: 453.713597ms

Mutexed structures with 4 threads total time: 310.135534ms
Lockfree structures with 4 threads total time: 309.631367ms

Mutexed structures with 8 threads total time: 252.969277ms
Lockfree structures with 8 threads total time: 260.894533ms

Mutexed structures with 16 threads total time: 247.005546ms
Lockfree structures with 16 threads total time: 244.91421ms

Mutexed structures with 32 threads total time: 251.322232ms
Lockfree structures with 32 threads total time: 243.96609ms

Mutexed structures with 64 threads total time: 252.674054ms
Lockfree structures with 64 threads total time: 262.014743ms

Mutexed structures with 128 threads total time: 321.475201ms
Lockfree structures with 128 threads total time: 335.928792ms
```

