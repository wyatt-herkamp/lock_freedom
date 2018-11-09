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
CPU MHz:             2719.149
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
mean of 550736270.088 r/s (688533504 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 137420123.081 r/s (171787264 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 166797592.332 r/s (208513024 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 1252300433.180 r/s (1565544448 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 194936464.872 r/s (243695616 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 275518169.469 r/s (344431616 rounds in 1.250 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 1251268320.553 r/s (1564509184 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 124898638.658 r/s (156162048 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 274480247.147 r/s (343179264 rounds in 1.250 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 1249676427.453 r/s (1562758144 rounds in 1.251 seconds)
Target 1 (blocking):
mean of 86963678.096 r/s (108764160 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 280703802.247 r/s (351019008 rounds in 1.250 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 1249629811.919 r/s (1564439552 rounds in 1.252 seconds)
Target 1 (blocking):
mean of 39122098.489 r/s (49090560 rounds in 1.255 seconds)
Target 2 (lockfree):
mean of 213131223.330 r/s (266858496 rounds in 1.252 seconds)

```

## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 45407010.785 r/s (56763392 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 29232475.071 r/s (36543488 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 9643786.481 r/s (12056576 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 11922283.263 r/s (14905344 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 3112450.228 r/s (3891200 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5217515.304 r/s (6522880 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 8771048.647 r/s (10966016 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4940647.057 r/s (6178816 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 5285428.013 r/s (6608896 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8375958.130 r/s (10474496 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4838798.050 r/s (6050816 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 4791133.443 r/s (6051840 rounds in 1.263 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 8402730.300 r/s (10512384 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4849832.279 r/s (6069248 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 5114366.080 r/s (6480896 rounds in 1.267 seconds)

```

## STACK
```

Result for 1 threads:
Target 0 (mutex vector):
mean of 48645331.094 r/s (60818432 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 27703264.426 r/s (34634752 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 20615705.944 r/s (25774080 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 8376163.859 r/s (10471424 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4179837.201 r/s (5225472 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5798922.543 r/s (7249920 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 7910447.310 r/s (9890816 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 5094791.224 r/s (6371328 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 6714644.634 r/s (8396800 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 7533731.787 r/s (9420800 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 5001510.368 r/s (6256640 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 6813254.366 r/s (8526848 rounds in 1.252 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 7531617.558 r/s (9422848 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4976811.783 r/s (6230016 rounds in 1.252 seconds)
Target 2 (lockfree):
mean of 6764608.724 r/s (8514560 rounds in 1.259 seconds)

```

## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 3285046.847 r/s (4520960 rounds in 1.376 seconds)
Target 1 (lockfree insert):
mean of 1937194.025 r/s (2422784 rounds in 1.251 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 1996279.990 r/s (2496512 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 2232149.138 r/s (2791424 rounds in 1.251 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2026310.605 r/s (2534400 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 3590079.975 r/s (4489216 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2340674.525 r/s (2931712 rounds in 1.253 seconds)
Target 1 (lockfree insert):
mean of 3548806.180 r/s (4455424 rounds in 1.255 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 6512548.990 r/s (8142848 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 4614822.285 r/s (5769216 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 2034102.528 r/s (2543616 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 7253207.981 r/s (9068544 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2033670.606 r/s (2545664 rounds in 1.252 seconds)
Target 1 (lockfree get):
mean of 10132936.642 r/s (12692480 rounds in 1.253 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2469446.628 r/s (3090432 rounds in 1.251 seconds)
Target 1 (lockfree get):
mean of 10957499.554 r/s (13734912 rounds in 1.253 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 21024768.493 r/s (26286080 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 1512931.509 r/s (2126848 rounds in 1.406 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 12159807.134 r/s (15201280 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 5636035.473 r/s (7047168 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10774330.601 r/s (13471744 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 10415202.415 r/s (13030400 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10739684.646 r/s (13428736 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 10921761.007 r/s (13660160 rounds in 1.251 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 4508349.089 r/s (5637120 rounds in 1.250 seconds)
Target 1 (lockfree mixed):
mean of 1626383.152 r/s (2034688 rounds in 1.251 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1513476.620 r/s (1892352 rounds in 1.250 seconds)
Target 1 (lockfree mixed):
mean of 1089318.088 r/s (1363968 rounds in 1.252 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1283408.116 r/s (1606656 rounds in 1.252 seconds)
Target 1 (lockfree mixed):
mean of 1801448.482 r/s (2252800 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1385621.047 r/s (1736704 rounds in 1.253 seconds)
Target 1 (lockfree mixed):
mean of 1584581.964 r/s (2145280 rounds in 1.354 seconds)

```

## MPSC CHANNEL
```
Mutexed VecDeque with 2 threads total time: 213.117776ms
Std's MPSC with 2 threads total time: 143.642016ms
Lockfree MPSC with 2 threads total time: 112.512402ms

Mutexed VecDeque with 4 threads total time: 437.781345ms
Std's MPSC with 4 threads total time: 255.023538ms
Lockfree MPSC with 4 threads total time: 195.19434ms

Mutexed VecDeque with 8 threads total time: 782.063676ms
Std's MPSC with 8 threads total time: 482.142987ms
Lockfree MPSC with 8 threads total time: 390.354092ms

Mutexed VecDeque with 16 threads total time: 1.603557774s
Std's MPSC with 16 threads total time: 1.033397252s
Lockfree MPSC with 16 threads total time: 810.351382ms

Mutexed VecDeque with 32 threads total time: 3.055130835s
Std's MPSC with 32 threads total time: 1.937340757s
Lockfree MPSC with 32 threads total time: 1.485801395s
```

## SPSC CHANNEL
```
Mutexed VecDeque total time: 138.179049ms
Lockfree SPSC total time: 93.300085ms
```

## REQUEST PROGRAM
```
A program simulating a concurrent server.

Mutexed HashMap and VecDeque with 2 threads total time: 1.037979317s
Mutexed HashMap and LinkedList with 2 threads total time: 1.178164118s
Lockfree structures with 2 threads total time: 716.242912ms

Mutexed HashMap and VecDeque with 4 threads total time: 648.561311ms
Mutexed HashMap and LinkedList with 4 threads total time: 1.773466401s
Lockfree structures with 4 threads total time: 612.962325ms

Mutexed HashMap and VecDeque with 8 threads total time: 808.438263ms
Mutexed HashMap and LinkedList with 8 threads total time: 2.142263964s
Lockfree structures with 8 threads total time: 724.830043ms

Mutexed HashMap and VecDeque with 16 threads total time: 2.20471053s
Mutexed HashMap and LinkedList with 16 threads total time: 4.350239883s
Lockfree structures with 16 threads total time: 1.389524294s
```

## MESSAGE REVERB PROGRAM
```
A program which reverberates messages

Mutexed VecDeque with 2 threads total time: 215.862814ms
Mutexed LinkedList with 2 threads total time: 550.440662ms
Lockfree Queue with 2 threads total time: 214.818663ms

Mutexed VecDeque with 4 threads total time: 230.460383ms
Mutexed LinkedList with 4 threads total time: 441.698687ms
Lockfree Queue with 4 threads total time: 186.162849ms

Mutexed VecDeque with 8 threads total time: 243.548779ms
Mutexed LinkedList with 8 threads total time: 486.230109ms
Lockfree Queue with 8 threads total time: 176.135548ms

Mutexed VecDeque with 16 threads total time: 242.065692ms
Mutexed LinkedList with 16 threads total time: 485.398551ms
Lockfree Queue with 16 threads total time: 171.684351ms
```

## HASH MINING
```
A program simulating a hash miner.

Mutexed structures with 2 threads total time: 470.181902ms
Lockfree structures with 2 threads total time: 464.902824ms

Mutexed structures with 4 threads total time: 309.194947ms
Lockfree structures with 4 threads total time: 304.760251ms

Mutexed structures with 8 threads total time: 254.56414ms
Lockfree structures with 8 threads total time: 256.269443ms

Mutexed structures with 16 threads total time: 253.461905ms
Lockfree structures with 16 threads total time: 243.698398ms

Mutexed structures with 32 threads total time: 254.395376ms
Lockfree structures with 32 threads total time: 245.013882ms

Mutexed structures with 64 threads total time: 256.810224ms
Lockfree structures with 64 threads total time: 270.721447ms

Mutexed structures with 128 threads total time: 332.479257ms
Lockfree structures with 128 threads total time: 343.356426ms
```

