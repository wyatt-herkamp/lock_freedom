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
CPU MHz:             600.070
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
mean of 512339998.316 r/s (640475136 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 127659576.136 r/s (159597568 rounds in 1.250 seconds)
Target 2 (blocking with cached access):
mean of 307369551.457 r/s (384269312 rounds in 1.250 seconds)
Target 3 (lockfree):
mean of 127086991.346 r/s (158881792 rounds in 1.250 seconds)
Target 4 (lockfree with cached id):
mean of 171903481.833 r/s (214910976 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 1224534236.758 r/s (1530781696 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 201114124.053 r/s (251430912 rounds in 1.250 seconds)
Target 2 (blocking with cached access):
mean of 341283156.190 r/s (426638336 rounds in 1.250 seconds)
Target 3 (lockfree):
mean of 276940066.187 r/s (346208256 rounds in 1.250 seconds)
Target 4 (lockfree with cached id):
mean of 362172242.440 r/s (452755456 rounds in 1.250 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 1224308314.219 r/s (1530717184 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 146040454.294 r/s (182592512 rounds in 1.250 seconds)
Target 2 (blocking with cached access):
mean of 193072110.939 r/s (241392640 rounds in 1.250 seconds)
Target 3 (lockfree):
mean of 276749425.817 r/s (346010624 rounds in 1.250 seconds)
Target 4 (lockfree with cached id):
mean of 363676507.628 r/s (454684672 rounds in 1.250 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 1221876497.009 r/s (1527902208 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 117031493.999 r/s (146362368 rounds in 1.251 seconds)
Target 2 (blocking with cached access):
mean of 139096494.050 r/s (173952000 rounds in 1.251 seconds)
Target 3 (lockfree):
mean of 276931885.383 r/s (346303488 rounds in 1.251 seconds)
Target 4 (lockfree with cached id):
mean of 363578319.837 r/s (454644736 rounds in 1.250 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 1211117996.261 r/s (1516181504 rounds in 1.252 seconds)
Target 1 (blocking):
mean of 56597748.858 r/s (70948864 rounds in 1.254 seconds)
Target 2 (blocking with cached access):
mean of 61013063.018 r/s (76456960 rounds in 1.253 seconds)
Target 3 (lockfree):
mean of 276770055.082 r/s (346525696 rounds in 1.252 seconds)
Target 4 (lockfree with cached id):
mean of 363486387.640 r/s (455006208 rounds in 1.252 seconds)

```

## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 44827221.767 r/s (56043520 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 29947161.731 r/s (37440512 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 14975138.295 r/s (18722816 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 11816976.969 r/s (14773248 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 3117723.935 r/s (3898368 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 5245779.820 r/s (6558720 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 8588558.800 r/s (10737664 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4948195.464 r/s (6188032 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 5963860.189 r/s (7457792 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8384087.995 r/s (10482688 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4933341.699 r/s (6170624 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 5705837.366 r/s (7157760 rounds in 1.254 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 8366080.727 r/s (10466304 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4933845.329 r/s (6175744 rounds in 1.252 seconds)
Target 2 (lockfree):
mean of 5713875.815 r/s (7193600 rounds in 1.259 seconds)

```

## STACK
```

Result for 1 threads:
Target 0 (mutex vector):
mean of 47975548.931 r/s (59974656 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 29640251.521 r/s (37056512 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 20290779.979 r/s (25367552 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 8127753.037 r/s (10161152 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4134159.447 r/s (5168128 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 6252146.133 r/s (7816192 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 7736580.359 r/s (9673728 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4970742.788 r/s (6214656 rounds in 1.250 seconds)
Target 2 (lockfree):
mean of 6655233.805 r/s (8322048 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 7455939.868 r/s (9323520 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 4870029.522 r/s (6091776 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 6745471.299 r/s (8664064 rounds in 1.284 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 7449937.949 r/s (9319424 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 4852388.221 r/s (6075392 rounds in 1.252 seconds)
Target 2 (lockfree):
mean of 6809277.629 r/s (8544256 rounds in 1.255 seconds)

```

## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 3609502.478 r/s (4513792 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 2028181.837 r/s (2536448 rounds in 1.251 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 1833257.886 r/s (2292736 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 2542847.039 r/s (3180544 rounds in 1.251 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 1898773.520 r/s (2375680 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 3727321.767 r/s (4661248 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2136361.525 r/s (2673664 rounds in 1.252 seconds)
Target 1 (lockfree insert):
mean of 3957622.741 r/s (4982784 rounds in 1.259 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 5115122.479 r/s (6394880 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 4465856.149 r/s (5583872 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 1833675.306 r/s (2292736 rounds in 1.250 seconds)
Target 1 (lockfree get):
mean of 7637352.774 r/s (9548800 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2127430.797 r/s (2662400 rounds in 1.251 seconds)
Target 1 (lockfree get):
mean of 11354179.328 r/s (14196736 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2233692.333 r/s (2798592 rounds in 1.253 seconds)
Target 1 (lockfree get):
mean of 11203599.608 r/s (14010368 rounds in 1.251 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 16498908.173 r/s (20627456 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 3248160.153 r/s (4061184 rounds in 1.250 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 12087612.895 r/s (15111168 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 6996831.431 r/s (8747008 rounds in 1.250 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10814369.122 r/s (13520896 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 11917086.792 r/s (14899200 rounds in 1.250 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10737527.471 r/s (13426688 rounds in 1.250 seconds)
Target 1 (lockfree remove):
mean of 11338454.039 r/s (14177280 rounds in 1.250 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 4106046.738 r/s (5134336 rounds in 1.250 seconds)
Target 1 (lockfree mixed):
mean of 1844771.083 r/s (2307072 rounds in 1.251 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1142608.771 r/s (1428480 rounds in 1.250 seconds)
Target 1 (lockfree mixed):
mean of 887016.845 r/s (1110016 rounds in 1.251 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 990934.456 r/s (1241088 rounds in 1.252 seconds)
Target 1 (lockfree mixed):
mean of 1797733.072 r/s (2249728 rounds in 1.251 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1029758.484 r/s (1292288 rounds in 1.255 seconds)
Target 1 (lockfree mixed):
mean of 2234413.394 r/s (2928640 rounds in 1.311 seconds)

```

## MPSC CHANNEL
```
Mutexed VecDeque with 3 threads total time: 202.190358ms
Std's MPSC with 3 threads total time: 152.467847ms
Lockfree MPSC with 3 threads total time: 114.504723ms

Mutexed VecDeque with 5 threads total time: 451.826018ms
Std's MPSC with 5 threads total time: 249.367375ms
Lockfree MPSC with 5 threads total time: 202.869205ms

Mutexed VecDeque with 9 threads total time: 820.189503ms
Std's MPSC with 9 threads total time: 489.758249ms
Lockfree MPSC with 9 threads total time: 398.933606ms

Mutexed VecDeque with 17 threads total time: 1.597487686s
Std's MPSC with 17 threads total time: 986.944784ms
Lockfree MPSC with 17 threads total time: 775.35985ms

Mutexed VecDeque with 33 threads total time: 3.124759445s
Std's MPSC with 33 threads total time: 1.966289958s
Lockfree MPSC with 33 threads total time: 1.633616088s
```

## SPSC CHANNEL
```
Mutexed VecDeque total time: 542.028355ms
Std's MPSC (as SPSC) total time: 89.458789ms
Lockfree SPSC total time: 346.208161ms
```

## SPMC CHANNEL
```
Mutexed VecDeque with 3 threads total time: 226.262447ms
Mutexed Std's MPSC (as SPMC) with 3 threads total time: 294.380577ms
Lockfree SPMC with 3 threads total time: 112.347338ms

Mutexed VecDeque with 5 threads total time: 354.558832ms
Mutexed Std's MPSC (as SPMC) with 5 threads total time: 154.245138ms
Lockfree SPMC with 5 threads total time: 91.61372ms

Mutexed VecDeque with 9 threads total time: 659.276875ms
Mutexed Std's MPSC (as SPMC) with 9 threads total time: 177.682057ms
Lockfree SPMC with 9 threads total time: 97.507943ms

Mutexed VecDeque with 17 threads total time: 1.145814126s
Mutexed Std's MPSC (as SPMC) with 17 threads total time: 161.009699ms
Lockfree SPMC with 17 threads total time: 195.01359ms

Mutexed VecDeque with 33 threads total time: 2.203173858s
Mutexed Std's MPSC (as SPMC) with 33 threads total time: 233.46754ms
Lockfree SPMC with 33 threads total time: 553.66644ms
```

## MPMC CHANNEL
```
Mutexed VecDeque with 4 threads total time: 84.220211ms
Mutexed Std's MPSC (as MPMC)  with 4 threads total time: 143.699488ms
Lockfree MPMC with 4 threads total time: 51.604701ms

Mutexed VecDeque with 8 threads total time: 187.67572ms
Mutexed Std's MPSC (as MPMC)  with 8 threads total time: 178.270823ms
Lockfree MPMC with 8 threads total time: 104.702646ms

Mutexed VecDeque with 16 threads total time: 387.21746ms
Mutexed Std's MPSC (as MPMC)  with 16 threads total time: 394.700527ms
Lockfree MPMC with 16 threads total time: 302.486073ms
```

## REQUEST PROGRAM
```
A program simulating a concurrent server.

Mutexed HashMap and Std's MPSC with 2 threads total time: 889.723892ms
Lockfree structures with 2 threads total time: 480.609236ms

Mutexed HashMap and Std's MPSC with 4 threads total time: 415.107444ms
Lockfree structures with 4 threads total time: 365.610184ms

Mutexed HashMap and Std's MPSC with 8 threads total time: 350.049227ms
Lockfree structures with 8 threads total time: 389.55723ms

Mutexed HashMap and Std's MPSC with 16 threads total time: 362.004562ms
Lockfree structures with 16 threads total time: 363.746061ms
```

## MESSAGE REVERB PROGRAM
```
A program which reverberates messages through a plain queue channel

Mutexed VecDeque with 2 threads total time: 185.66523ms
Mutexed LinkedList with 2 threads total time: 632.671064ms
Lockfree Queue with 2 threads total time: 197.111475ms

Mutexed VecDeque with 4 threads total time: 238.98724ms
Mutexed LinkedList with 4 threads total time: 459.36915ms
Lockfree Queue with 4 threads total time: 165.211953ms

Mutexed VecDeque with 8 threads total time: 238.63038ms
Mutexed LinkedList with 8 threads total time: 483.533882ms
Lockfree Queue with 8 threads total time: 156.590143ms

Mutexed VecDeque with 16 threads total time: 245.670847ms
Mutexed LinkedList with 16 threads total time: 492.253123ms
Lockfree Queue with 16 threads total time: 153.738852ms
```

## HASH MINING
```
A program simulating a hash miner.

Mutexed structures with 2 threads total time: 451.991989ms
Lockfree structures with 2 threads total time: 451.231423ms

Mutexed structures with 4 threads total time: 306.661305ms
Lockfree structures with 4 threads total time: 306.30003ms

Mutexed structures with 8 threads total time: 266.314533ms
Lockfree structures with 8 threads total time: 256.686569ms

Mutexed structures with 16 threads total time: 249.399111ms
Lockfree structures with 16 threads total time: 239.736868ms

Mutexed structures with 32 threads total time: 253.143116ms
Lockfree structures with 32 threads total time: 241.636613ms

Mutexed structures with 64 threads total time: 296.963983ms
Lockfree structures with 64 threads total time: 304.178749ms

Mutexed structures with 128 threads total time: 401.229491ms
Lockfree structures with 128 threads total time: 413.802962ms
```

