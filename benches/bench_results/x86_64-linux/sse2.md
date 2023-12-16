# Benchmarks for chacha20 using SSE2
These benches were ran on 12/16/2023 using a 3.0 GHz i9 CPU, immediately after a restart. Prior to the restart, it was slightly slower.
Last commit: 378303a

## ChaCha20::apply_keystream()
```
     Running src/chacha20.rs (target/release/deps/chacha20-5e4721a497c9e862)
Gnuplot not found, using plotters backend
stream-cipher/apply_keystream/1024
                        time:   [2985.7740 cycles 2989.1629 cycles 2992.8442 cycles]
                        thrpt:  [2.9227 cpb 2.9191 cpb 2.9158 cpb]
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe
stream-cipher/apply_keystream/2048
                        time:   [5976.2926 cycles 5985.3740 cycles 5995.8704 cycles]
                        thrpt:  [2.9277 cpb 2.9225 cpb 2.9181 cpb]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe
stream-cipher/apply_keystream/4096
                        time:   [11943.3637 cycles 11962.3455 cycles 11982.3584 cycles]
                        thrpt:  [2.9254 cpb 2.9205 cpb 2.9159 cpb]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
stream-cipher/apply_keystream/8192
                        time:   [23892.3367 cycles 23923.2513 cycles 23957.1875 cycles]
                        thrpt:  [2.9245 cpb 2.9203 cpb 2.9165 cpb]
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe
stream-cipher/apply_keystream/16384
                        time:   [47803.6400 cycles 47876.8759 cycles 47955.3464 cycles]
                        thrpt:  [2.9270 cpb 2.9222 cpb 2.9177 cpb]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe
```

## ChaCha20Rng::fill_bytes()
```
ChaCha20Rng/fill_bytes/1024
                        time:   [2963.9276 cycles 2970.4119 cycles 2977.3474 cycles]
                        thrpt:  [2.9076 cpb 2.9008 cpb 2.8945 cpb]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe
ChaCha20Rng/fill_bytes/2048
                        time:   [5912.3129 cycles 5919.7968 cycles 5928.1030 cycles]
                        thrpt:  [2.8946 cpb 2.8905 cpb 2.8869 cpb]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
ChaCha20Rng/fill_bytes/4096
                        time:   [11809.4518 cycles 11819.9490 cycles 11831.3046 cycles]
                        thrpt:  [2.8885 cpb 2.8857 cpb 2.8832 cpb]
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe
ChaCha20Rng/fill_bytes/8192
                        time:   [23619.4583 cycles 23645.0593 cycles 23671.4262 cycles]
                        thrpt:  [2.8896 cpb 2.8864 cpb 2.8832 cpb]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
ChaCha20Rng/fill_bytes/16384
                        time:   [47227.4538 cycles 47284.9074 cycles 47348.0004 cycles]
                        thrpt:  [2.8899 cpb 2.8860 cpb 2.8825 cpb]
Found 11 outliers among 100 measurements (11.00%)
  7 (7.00%) high mild
  4 (4.00%) high severe
```