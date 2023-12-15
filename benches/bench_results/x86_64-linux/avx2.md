# Benchmarks for chacha20 using AVX2
These benches were ran on 12/14/2023 using a 3.0 GHz i9 CPU.

## ChaCha20::apply_keystream()
```
     Running src/chacha20.rs (target/release/deps/chacha20-bc9efd54ac287186)
Gnuplot not found, using plotters backend
stream-cipher/apply_keystream/1024
                        time:   [971.8495 cycles 973.4309 cycles 975.1880 cycles]
                        thrpt:  [0.9523 cpb 0.9506 cpb 0.9491 cpb]
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe
stream-cipher/apply_keystream/2048
                        time:   [1919.9530 cycles 1923.4804 cycles 1927.1664 cycles]
                        thrpt:  [0.9410 cpb 0.9392 cpb 0.9375 cpb]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe
stream-cipher/apply_keystream/4096
                        time:   [3819.6429 cycles 3825.7375 cycles 3832.0932 cycles]
                        thrpt:  [0.9356 cpb 0.9340 cpb 0.9325 cpb]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
stream-cipher/apply_keystream/8192
                        time:   [7624.6832 cycles 7640.2698 cycles 7658.3298 cycles]
                        thrpt:  [0.9349 cpb 0.9327 cpb 0.9307 cpb]
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high mild
stream-cipher/apply_keystream/16384
                        time:   [15277.8659 cycles 15306.3642 cycles 15337.4795 cycles]
                        thrpt:  [0.9361 cpb 0.9342 cpb 0.9325 cpb]
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe
```
## ChaCha20Rng::fill_bytes()
```
ChaCha20Rng/fill_bytes/1024
                        time:   [1033.0020 cycles 1034.5844 cycles 1036.1889 cycles]
                        thrpt:  [1.0119 cpb 1.0103 cpb 1.0088 cpb]
Found 10 outliers among 100 measurements (10.00%)
  7 (7.00%) high mild
  3 (3.00%) high severe
ChaCha20Rng/fill_bytes/2048
                        time:   [2058.2873 cycles 2061.8231 cycles 2065.6562 cycles]
                        thrpt:  [1.0086 cpb 1.0067 cpb 1.0050 cpb]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe
ChaCha20Rng/fill_bytes/4096
                        time:   [4103.8855 cycles 4110.1684 cycles 4116.9702 cycles]
                        thrpt:  [1.0051 cpb 1.0035 cpb 1.0019 cpb]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
ChaCha20Rng/fill_bytes/8192
                        time:   [8216.7205 cycles 8229.3034 cycles 8242.1872 cycles]
                        thrpt:  [1.0061 cpb 1.0046 cpb 1.0030 cpb]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) high mild
  5 (5.00%) high severe
ChaCha20Rng/fill_bytes/16384
                        time:   [16447.9940 cycles 16476.3121 cycles 16507.7525 cycles]
                        thrpt:  [1.0076 cpb 1.0056 cpb 1.0039 cpb]
Found 10 outliers among 100 measurements (10.00%)
  7 (7.00%) high mild
  3 (3.00%) high severe
```