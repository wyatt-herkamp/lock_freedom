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
CPU MHz:             600.053
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
mean of 512139544.673 r/s (640275456 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 127709699.625 r/s (159646720 rounds in 1.250 seconds)
Target 2 (blocking with cached access):
mean of 319391947.975 r/s (399298560 rounds in 1.250 seconds)
Target 3 (lockfree):
mean of 143329876.152 r/s (179173376 rounds in 1.250 seconds)
Target 4 (lockfree with cached id):
mean of 192056433.840 r/s (240101376 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 1224972587.419 r/s (1531353088 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 201916879.806 r/s (252417024 rounds in 1.250 seconds)
Target 2 (blocking with cached access):
mean of 341535671.022 r/s (426953728 rounds in 1.250 seconds)
Target 3 (lockfree):
mean of 293260833.276 r/s (366610432 rounds in 1.250 seconds)
Target 4 (lockfree with cached id):
mean of 407323340.206 r/s (509195264 rounds in 1.250 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 1224501434.025 r/s (1530923008 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 150776418.500 r/s (188513280 rounds in 1.250 seconds)
Target 2 (blocking with cached access):
mean of 181609816.982 r/s (227062784 rounds in 1.250 seconds)
Target 3 (lockfree):
mean of 293164998.513 r/s (366534656 rounds in 1.250 seconds)
Target 4 (lockfree with cached id):
mean of 409897296.919 r/s (512483328 rounds in 1.250 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 1222923877.606 r/s (1529277440 rounds in 1.251 seconds)
Target 1 (blocking):
mean of 114432441.984 r/s (143108096 rounds in 1.251 seconds)
Target 2 (blocking with cached access):
mean of 147235153.382 r/s (184129536 rounds in 1.251 seconds)
Target 3 (lockfree):
mean of 293127646.125 r/s (366566400 rounds in 1.251 seconds)
Target 4 (lockfree with cached id):
mean of 409951416.129 r/s (512611328 rounds in 1.250 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 1221367754.618 r/s (1529025536 rounds in 1.252 seconds)
Target 1 (blocking):
mean of 57932682.138 r/s (72626176 rounds in 1.254 seconds)
Target 2 (blocking with cached access):
mean of 61572701.707 r/s (77186048 rounds in 1.254 seconds)
Target 3 (lockfree):
mean of 292909665.493 r/s (366750720 rounds in 1.252 seconds)
Target 4 (lockfree with cached id):
mean of 409716335.129 r/s (512984064 rounds in 1.252 seconds)

```

## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 46048473.563 r/s (57565184 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 29690911.627 r/s (37118976 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 14753269.199 r/s (18444288 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 11299725.747 r/s (14126080 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 3944054.410 r/s (4931584 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5326491.570 r/s (6659072 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 8416399.560 r/s (10523648 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 5019818.755 r/s (6276096 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 6087256.870 r/s (7611392 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8118255.857 r/s (10152960 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4939717.925 r/s (6178816 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 5860964.634 r/s (7444480 rounds in 1.270 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 8172190.930 r/s (10222592 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4922707.129 r/s (6162432 rounds in 1.252 seconds)
Target 2 (lockfree):
mean of 5865378.853 r/s (7488512 rounds in 1.277 seconds)

```

## STACK
```

Result for 1 threads:
Target 0 (mutex vector):
mean of 48611164.557 r/s (60774400 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 29589799.735 r/s (36993024 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 19014944.227 r/s (23772160 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 8297557.599 r/s (10373120 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4366001.662 r/s (5457920 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 6045083.609 r/s (7557120 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 7879030.973 r/s (9850880 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4930568.492 r/s (6164480 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 6414959.076 r/s (8020992 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 7726984.757 r/s (9662464 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4836459.416 r/s (6050816 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 6578805.687 r/s (8397824 rounds in 1.276 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 7519142.401 r/s (9405440 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4821306.166 r/s (6034432 rounds in 1.252 seconds)
Target 2 (lockfree):
mean of 6260926.210 r/s (8185856 rounds in 1.307 seconds)

```

## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 3238107.139 r/s (4520960 rounds in 1.396 seconds)
Target 1 (lockfree insert):
mean of 2025093.662 r/s (2532352 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 1998383.723 r/s (2498560 rounds in 1.250 seconds)
Target 1 (lockfree insert):
mean of 2552267.722 r/s (3190784 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2303851.655 r/s (2881536 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 3736966.690 r/s (4673536 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2435558.902 r/s (3048448 rounds in 1.252 seconds)
Target 1 (lockfree insert):
mean of 3778903.316 r/s (4796416 rounds in 1.269 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 6509402.531 r/s (8138752 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 5043106.336 r/s (6304768 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 2040989.076 r/s (2551808 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 8099934.659 r/s (10127360 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2376136.538 r/s (2971648 rounds in 1.251 seconds)
Target 1 (lockfree get):
mean of 11793737.569 r/s (14745600 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2622804.377 r/s (3284992 rounds in 1.252 seconds)
Target 1 (lockfree get):
mean of 11623563.628 r/s (14535680 rounds in 1.251 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 21332565.852 r/s (26670080 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 3484241.121 r/s (4356096 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 12338803.928 r/s (15425536 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 7265057.845 r/s (9082880 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10573924.486 r/s (13219840 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 11823106.244 r/s (14783488 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10517551.510 r/s (13150208 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 11850734.650 r/s (14820352 rounds in 1.251 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 4532106.208 r/s (5665792 rounds in 1.250 seconds)
Target 1 (lockfree mixed):
mean of 1891664.690 r/s (2365440 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1532775.396 r/s (1916928 rounds in 1.251 seconds)
Target 1 (lockfree mixed):
mean of 1105999.898 r/s (1384448 rounds in 1.252 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1284532.880 r/s (1607680 rounds in 1.252 seconds)
Target 1 (lockfree mixed):
mean of 1801849.896 r/s (2255872 rounds in 1.252 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1430066.336 r/s (1790976 rounds in 1.252 seconds)
Target 1 (lockfree mixed):
mean of 2229312.954 r/s (2830336 rounds in 1.270 seconds)

```

## MPSC CHANNEL
```
Mutexed VecDeque with 3 threads total time: 185.189303ms
Std's MPSC with 3 threads total time: 143.969618ms
Lockfree MPSC with 3 threads total time: 113.960842ms

Mutexed VecDeque with 5 threads total time: 412.285397ms
Std's MPSC with 5 threads total time: 253.788246ms
Lockfree MPSC with 5 threads total time: 205.905003ms

Mutexed VecDeque with 9 threads total time: 796.049182ms
Std's MPSC with 9 threads total time: 488.886024ms
Lockfree MPSC with 9 threads total time: 381.563423ms

Mutexed VecDeque with 17 threads total time: 1.565540211s
Std's MPSC with 17 threads total time: 975.174664ms
Lockfree MPSC with 17 threads total time: 778.958957ms

Mutexed VecDeque with 33 threads total time: 3.043269774s
Std's MPSC with 33 threads total time: 2.020977961s
Lockfree MPSC with 33 threads total time: 1.596467021s
```

## SPSC CHANNEL
```
Mutexed VecDeque total time: 720.896882ms
Std's MPSC (as SPSC) total time: 102.649848ms
Lockfree SPSC total time: 344.578959ms
```

## SPMC CHANNEL
```
Mutexed VecDeque with 3 threads total time: 235.261687ms
Mutexed Std's MPSC (as SPMC) with 3 threads total time: 309.134961ms
Lockfree SPMC with 3 threads total time: 109.883979ms

Mutexed VecDeque with 5 threads total time: 338.168495ms
Mutexed Std's MPSC (as SPMC) with 5 threads total time: 154.388882ms
Lockfree SPMC with 5 threads total time: 92.935627ms

Mutexed VecDeque with 9 threads total time: 760.613029ms
Mutexed Std's MPSC (as SPMC) with 9 threads total time: 161.215158ms
Lockfree SPMC with 9 threads total time: 96.045893ms

Mutexed VecDeque with 17 threads total time: 1.117225065s
Mutexed Std's MPSC (as SPMC) with 17 threads total time: 170.246325ms
Lockfree SPMC with 17 threads total time: 190.20428ms

Mutexed VecDeque with 33 threads total time: 2.178239315s
Mutexed Std's MPSC (as SPMC) with 33 threads total time: 189.099692ms
Lockfree SPMC with 33 threads total time: 367.032635ms
```

## MPMC CHANNEL
```
Mutexed VecDeque with 4 threads total time: 80.681871ms
Mutexed Std's MPSC (as MPMC)  with 4 threads total time: 149.070259ms
Lockfree MPMC with 4 threads total time: 54.256853ms

Mutexed VecDeque with 8 threads total time: 190.167113ms
Mutexed Std's MPSC (as MPMC)  with 8 threads total time: 190.849166ms
Lockfree MPMC with 8 threads total time: 98.339171ms

Mutexed VecDeque with 16 threads total time: 392.888833ms
Mutexed Std's MPSC (as MPMC)  with 16 threads total time: 410.274314ms
Lockfree MPMC with 16 threads total time: 347.902178ms
```

## REQUEST PROGRAM
```
A program simulating a concurrent server.

Mutexed HashMap and Std's MPSC with 2 threads total time: 888.370187ms
Lockfree structures with 2 threads total time: 480.961161ms

Mutexed HashMap and Std's MPSC with 4 threads total time: 350.756452ms
Lockfree structures with 4 threads total time: 417.262342ms

Mutexed HashMap and Std's MPSC with 8 threads total time: 368.132417ms
Lockfree structures with 8 threads total time: 361.023548ms

Mutexed HashMap and Std's MPSC with 16 threads total time: 344.60952ms
Lockfree structures with 16 threads total time: 396.267218ms
```

## MESSAGE REVERB PROGRAM
```
A program which reverberates messages through a plain queue channel

Mutexed VecDeque with 2 threads total time: 212.95505ms
Mutexed LinkedList with 2 threads total time: 616.048431ms
Lockfree Queue with 2 threads total time: 202.782665ms

Mutexed VecDeque with 4 threads total time: 230.206368ms
Mutexed LinkedList with 4 threads total time: 450.223036ms
Lockfree Queue with 4 threads total time: 153.37729ms

Mutexed VecDeque with 8 threads total time: 245.975862ms
Mutexed LinkedList with 8 threads total time: 474.660115ms
Lockfree Queue with 8 threads total time: 153.65495ms

Mutexed VecDeque with 16 threads total time: 245.687481ms
Mutexed LinkedList with 16 threads total time: 489.669259ms
Lockfree Queue with 16 threads total time: 151.112758ms
```

## HASH MINING
```
A program simulating a hash miner.

Mutexed structures with 2 threads total time: 462.878694ms
Lockfree structures with 2 threads total time: 471.778866ms

Mutexed structures with 4 threads total time: 300.675913ms
Lockfree structures with 4 threads total time: 309.105324ms

Mutexed structures with 8 threads total time: 251.474398ms
Lockfree structures with 8 threads total time: 252.031354ms

Mutexed structures with 16 threads total time: 243.071479ms
Lockfree structures with 16 threads total time: 246.860797ms

Mutexed structures with 32 threads total time: 252.089704ms
Lockfree structures with 32 threads total time: 246.509362ms

Mutexed structures with 64 threads total time: 254.433402ms
Lockfree structures with 64 threads total time: 268.088123ms

Mutexed structures with 128 threads total time: 323.890508ms
Lockfree structures with 128 threads total time: 339.913444ms
```

