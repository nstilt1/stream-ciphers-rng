# Benchmarks for chacha20 using AVX2
These benches were ran on 12/16/2023 using a 3.0 GHz i9 CPU, immediately after a restart. Prior to the restart, it was slightly slower.
Last commit: 378303a

## ChaCha20::apply_keystream()
```
     Running src/chacha20.rs (target/release/deps/chacha20-5e4721a497c9e862)
Gnuplot not found, using plotters backend
stream-cipher/apply_keystream/1024
                        time:   [971.2808 cycles 973.3628 cycles 975.6034 cycles]
                        thrpt:  [0.9527 cpb 0.9505 cpb 0.9485 cpb]
Found 10 outliers among 100 measurements (10.00%)
  7 (7.00%) high mild
  3 (3.00%) high severe
stream-cipher/apply_keystream/2048
                        time:   [1919.2179 cycles 1924.5813 cycles 1930.4761 cycles]
                        thrpt:  [0.9426 cpb 0.9397 cpb 0.9371 cpb]
Found 15 outliers among 100 measurements (15.00%)
  10 (10.00%) high mild
  5 (5.00%) high severe
stream-cipher/apply_keystream/4096
                        time:   [3807.4784 cycles 3810.2826 cycles 3813.2254 cycles]
                        thrpt:  [0.9310 cpb 0.9302 cpb 0.9296 cpb]
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe
stream-cipher/apply_keystream/8192
                        time:   [7573.1425 cycles 7579.8142 cycles 7587.2330 cycles]
                        thrpt:  [0.9262 cpb 0.9253 cpb 0.9245 cpb]
Found 9 outliers among 100 measurements (9.00%)
  6 (6.00%) high mild
  3 (3.00%) high severe
stream-cipher/apply_keystream/16384
                        time:   [15140.7635 cycles 15158.0590 cycles 15175.8650 cycles]
                        thrpt:  [0.9263 cpb 0.9252 cpb 0.9241 cpb]
Found 11 outliers among 100 measurements (11.00%)
  8 (8.00%) high mild
  3 (3.00%) high severe
```
## ChaCha20Rng::fill_bytes()
```
ChaCha20Rng/fill_bytes/1024
                        time:   [1017.5689 cycles 1019.4255 cycles 1021.4392 cycles]
                        thrpt:  [0.9975 cpb 0.9955 cpb 0.9937 cpb]
Found 9 outliers among 100 measurements (9.00%)
  7 (7.00%) high mild
  2 (2.00%) high severe
ChaCha20Rng/fill_bytes/2048
                        time:   [2049.5323 cycles 2053.3430 cycles 2057.4375 cycles]
                        thrpt:  [1.0046 cpb 1.0026 cpb 1.0007 cpb]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe
ChaCha20Rng/fill_bytes/4096
                        time:   [4076.0061 cycles 4084.7326 cycles 4094.7519 cycles]
                        thrpt:  [0.9997 cpb 0.9972 cpb 0.9951 cpb]
Found 11 outliers among 100 measurements (11.00%)
  10 (10.00%) high mild
  1 (1.00%) high severe
ChaCha20Rng/fill_bytes/8192
                        time:   [8245.1188 cycles 8265.3651 cycles 8287.5906 cycles]
                        thrpt:  [1.0117 cpb 1.0090 cpb 1.0065 cpb]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
ChaCha20Rng/fill_bytes/16384
                        time:   [16478.7693 cycles 16527.7314 cycles 16580.7712 cycles]
                        thrpt:  [1.0120 cpb 1.0088 cpb 1.0058 cpb]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
```