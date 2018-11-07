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
CPU MHz:             2733.962
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
mean of 506536819.718 r/s (633275392 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 136932153.456 r/s (171179008 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 167526675.374 r/s (209440768 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 1196534464.819 r/s (1495838720 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 195337284.646 r/s (244194304 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 270994260.453 r/s (338778112 rounds in 1.250 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 1196157055.668 r/s (1495584768 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 124807236.905 r/s (156048384 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 273182931.497 r/s (341566464 rounds in 1.250 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 1195172496.078 r/s (1494583296 rounds in 1.251 seconds)
Target 1 (blocking):
mean of 89867310.473 r/s (112394240 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 273879812.563 r/s (342494208 rounds in 1.251 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 1195486667.768 r/s (1496662016 rounds in 1.252 seconds)
Target 1 (blocking):
mean of 37912826.332 r/s (47576064 rounds in 1.255 seconds)
Target 2 (lockfree):
mean of 206954438.166 r/s (259161088 rounds in 1.252 seconds)

```

## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 45917376.487 r/s (57406464 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 29765141.001 r/s (37212160 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 12432531.853 r/s (15543296 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 11615106.170 r/s (14520320 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 3512020.017 r/s (4390912 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 3540008.111 r/s (4426752 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 8602126.946 r/s (10755072 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 5226877.380 r/s (6536192 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 3508209.740 r/s (4387840 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8352335.183 r/s (10443776 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 5084162.691 r/s (6359040 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 3496521.486 r/s (4392960 rounds in 1.256 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 8297308.112 r/s (10379264 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 5065606.413 r/s (6339584 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 3558368.502 r/s (4457472 rounds in 1.253 seconds)

```

## STACK
```

Result for 1 threads:
Target 0 (mutex vector):
mean of 48726350.104 r/s (60913664 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 27262230.983 r/s (34080768 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 19987351.425 r/s (24986624 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 8282999.052 r/s (10354688 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4548610.354 r/s (5686272 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5814224.568 r/s (7269376 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 7732174.821 r/s (9667584 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 5097893.274 r/s (6374400 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5708013.299 r/s (7138304 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 7364876.427 r/s (9210880 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4929030.152 r/s (6163456 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5633750.837 r/s (7060480 rounds in 1.253 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 7321938.890 r/s (9160704 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4886641.050 r/s (6117376 rounds in 1.252 seconds)
Target 2 (lockfree):
mean of 5682123.391 r/s (7135232 rounds in 1.256 seconds)

```

## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 3267182.898 r/s (4520960 rounds in 1.384 seconds)
Target 1 (lockfree insert):
mean of 1928983.535 r/s (2412544 rounds in 1.251 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 1996869.304 r/s (2497536 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 2217316.788 r/s (2772992 rounds in 1.251 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2122341.816 r/s (2655232 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 3618794.889 r/s (4526080 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2240628.599 r/s (2804736 rounds in 1.252 seconds)
Target 1 (lockfree insert):
mean of 3771605.444 r/s (4772864 rounds in 1.265 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 6499228.907 r/s (8125440 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 4579210.103 r/s (5725184 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 2038481.973 r/s (2548736 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 7621391.536 r/s (9529344 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2347492.028 r/s (2936832 rounds in 1.251 seconds)
Target 1 (lockfree get):
mean of 11181176.668 r/s (13983744 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2433835.344 r/s (3046400 rounds in 1.252 seconds)
Target 1 (lockfree get):
mean of 10455018.659 r/s (13076480 rounds in 1.251 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 20929738.864 r/s (26166272 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 1690514.188 r/s (2116608 rounds in 1.252 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 12179404.417 r/s (15226880 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 3932625.757 r/s (4917248 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10606670.486 r/s (13261824 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 9943221.761 r/s (12431360 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10531253.858 r/s (13169664 rounds in 1.251 seconds)
Target 1 (lockfree remove):
mean of 10565569.962 r/s (13214720 rounds in 1.251 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 4514263.308 r/s (5644288 rounds in 1.250 seconds)
Target 1 (lockfree mixed):
mean of 1642837.973 r/s (2054144 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1492036.725 r/s (1865728 rounds in 1.250 seconds)
Target 1 (lockfree mixed):
mean of 1080746.750 r/s (1351680 rounds in 1.251 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1286443.714 r/s (1609728 rounds in 1.251 seconds)
Target 1 (lockfree mixed):
mean of 1743738.915 r/s (2181120 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1400748.683 r/s (1754112 rounds in 1.252 seconds)
Target 1 (lockfree mixed):
mean of 1703377.847 r/s (2291712 rounds in 1.345 seconds)

```

## REQUEST PROGRAM
```
A program simulating a concurrent server.

Mutexed HashMap and VecDeque with 2 threads total time: 1.286702708s
Mutexed HashMap and LinkedList with 2 threads total time: 1.082673732s
Lockfree structures with 2 threads total time: 774.275883ms

Mutexed HashMap and VecDeque with 4 threads total time: 722.572799ms
Mutexed HashMap and LinkedList with 4 threads total time: 1.80335551s
Lockfree structures with 4 threads total time: 862.058002ms

Mutexed HashMap and VecDeque with 8 threads total time: 923.789316ms
Mutexed HashMap and LinkedList with 8 threads total time: 2.175552943s
Lockfree structures with 8 threads total time: 1.037737345s

Mutexed HashMap and VecDeque with 16 threads total time: 2.157493879s
Mutexed HashMap and LinkedList with 16 threads total time: 4.395953192s
Lockfree structures with 16 threads total time: 2.330237716s
```

## MESSAGE REVERB PROGRAM
```
A program which reverberates messages

Mutexed VecDeque with 2 threads total time: 221.1694ms
Mutexed LinkedList with 2 threads total time: 597.23209ms
Lockfree Queue with 2 threads total time: 301.342738ms

Mutexed VecDeque with 4 threads total time: 236.545399ms
Mutexed LinkedList with 4 threads total time: 468.859768ms
Lockfree Queue with 4 threads total time: 297.518841ms

Mutexed VecDeque with 8 threads total time: 244.947416ms
Mutexed LinkedList with 8 threads total time: 482.612872ms
Lockfree Queue with 8 threads total time: 290.167168ms

Mutexed VecDeque with 16 threads total time: 240.453909ms
Mutexed LinkedList with 16 threads total time: 495.736421ms
Lockfree Queue with 16 threads total time: 287.839634ms
```

## HASH MINING
```
A program simulating a hash miner.

Mutexed structures with 2 threads total time: 473.004487ms
Lockfree structures with 2 threads total time: 471.535073ms

Mutexed structures with 4 threads total time: 321.877187ms
Lockfree structures with 4 threads total time: 316.915663ms

Mutexed structures with 8 threads total time: 262.457855ms
Lockfree structures with 8 threads total time: 247.672911ms

Mutexed structures with 16 threads total time: 254.399398ms
Lockfree structures with 16 threads total time: 256.56411ms

Mutexed structures with 32 threads total time: 253.679584ms
Lockfree structures with 32 threads total time: 253.324831ms

Mutexed structures with 64 threads total time: 262.056944ms
Lockfree structures with 64 threads total time: 275.206843ms

Mutexed structures with 128 threads total time: 328.624333ms
Lockfree structures with 128 threads total time: 344.630277ms
```

