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
CPU MHz:             2362.060
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
mean of 19143066.116 r/s (28714700 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 17425172.747 r/s (26137800 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 17430244.705 r/s (26145400 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 46303184.804 r/s (69455200 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 38786184.142 r/s (58179600 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 40622335.844 r/s (60933600 rounds in 1.500 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 46404317.110 r/s (69608000 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 33687344.471 r/s (50531200 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 41480429.522 r/s (62220800 rounds in 1.500 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 39938501.971 r/s (59910400 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 28357015.478 r/s (42537600 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 37161410.165 r/s (55744000 rounds in 1.500 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 41333305.817 r/s (62003200 rounds in 1.500 seconds)
Target 1 (blocking):
mean of 15478449.722 r/s (23219200 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 39013527.215 r/s (58521600 rounds in 1.500 seconds)

Result for 512 threads:
Target 0 (std/global):
mean of 32728545.891 r/s (49152000 rounds in 1.502 seconds)
Target 1 (blocking):
mean of 5694848.451 r/s (8550400 rounds in 1.501 seconds)
Target 2 (lockfree):
mean of 31909714.215 r/s (47872000 rounds in 1.500 seconds)

```
## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 13735746.000 r/s (20603700 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 11406977.262 r/s (17110500 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 5539013.780 r/s (8308600 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 16818729.462 r/s (25228200 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 4773424.953 r/s (7160200 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 3967540.347 r/s (5951400 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 10146287.809 r/s (15219600 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 5112750.778 r/s (7669200 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 3971142.535 r/s (5956800 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8649810.061 r/s (12975200 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 4520154.813 r/s (6780800 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 4509500.402 r/s (6764800 rounds in 1.500 seconds)

```
## STACK
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 13647543.581 r/s (20471400 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 10757844.498 r/s (16136800 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 9787247.849 r/s (14680900 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 15983631.539 r/s (23975600 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 4101181.304 r/s (6151800 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 6931394.183 r/s (10397200 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 9944129.743 r/s (14916400 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 5046788.104 r/s (7570400 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 5517758.846 r/s (8276800 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 8670489.543 r/s (13006400 rounds in 1.500 seconds)
Target 1 (mutex linked list):
mean of 4522721.192 r/s (6784800 rounds in 1.500 seconds)
Target 2 (lockfree):
mean of 6041037.930 r/s (9062400 rounds in 1.500 seconds)

```
## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 2781378.258 r/s (4520600 rounds in 1.625 seconds)
Target 1 (lockfree insert):
mean of 2118597.458 r/s (3177900 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 2087594.890 r/s (3131400 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 2291151.858 r/s (3436800 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 2256744.039 r/s (3385200 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 2771804.715 r/s (4158000 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 2449234.674 r/s (3674400 rounds in 1.500 seconds)
Target 1 (lockfree insert):
mean of 3266996.288 r/s (4900800 rounds in 1.500 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 4628947.029 r/s (6943500 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 3149193.410 r/s (4723900 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 2311543.654 r/s (3467400 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 3700221.608 r/s (5550400 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 2559702.388 r/s (3839600 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 5046395.519 r/s (7569600 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 2816312.865 r/s (4224800 rounds in 1.500 seconds)
Target 1 (lockfree get):
mean of 5696760.944 r/s (8545600 rounds in 1.500 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 7834992.823 r/s (11752500 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 1609243.695 r/s (2413900 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 14942427.202 r/s (22413800 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 3787170.081 r/s (5681000 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 10486702.290 r/s (15730400 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 6726653.317 r/s (10090000 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 10470273.331 r/s (15705600 rounds in 1.500 seconds)
Target 1 (lockfree remove):
mean of 8515137.465 r/s (12772800 rounds in 1.500 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 3571328.195 r/s (5357100 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 1502949.733 r/s (2254500 rounds in 1.500 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1777168.377 r/s (2665800 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 954125.109 r/s (1431400 rounds in 1.500 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 1391333.253 r/s (2087200 rounds in 1.500 seconds)
Target 1 (lockfree mixed):
mean of 1173328.553 r/s (1760000 rounds in 1.500 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 1478414.248 r/s (2218400 rounds in 1.501 seconds)
Target 1 (lockfree mixed):
mean of 1336781.215 r/s (2005600 rounds in 1.500 seconds)

```
