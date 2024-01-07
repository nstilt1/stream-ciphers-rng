# Benching chacha20 using NEON and zeroize
These benches were ran on 1/7/2024 using a Raspberry Pi Model 4b after impling `ZeroizeOnDrop` for the backends. The comparisons within the bench can be ignored, because compiling the bench takes 5+ minutes, and I don't use `cargo clean` very often because of that.
## Cipher
     Running src/chacha20.rs (target/release/deps/chacha20-3e29041298637591)
Gnuplot not found, using plotters backend
Benchmarking stream-cipher/apply_keystream/1024: Collecting 100 samples in estimstream-cipher/apply_keystream/1024
                        time:   [6311.8301 cycles 6314.1568 cycles 6316.5309 cycles]
                        thrpt:  [6.1685 cpb 6.1662 cpb 6.1639 cpb]
                 change:
                        time:   [+0.1442% +0.1834% +0.2186%] (p = 0.00 < 0.05)
                        thrpt:  [-0.2181% -0.1830% -0.1440%]
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe
Benchmarking stream-cipher/apply_keystream/2048: Collecting 100 samples in estimstream-cipher/apply_keystream/2048
                        time:   [12533.6516 cycles 12534.2541 cycles 12534.9714 cycles]
                        thrpt:  [6.1206 cpb 6.1202 cpb 6.1199 cpb]
                 change:
                        time:   [-0.0585% -0.0224% +0.0060%] (p = 0.19 > 0.05)
                        thrpt:  [-0.0060% +0.0224% +0.0585%]
                        No change in performance detected.
Found 12 outliers among 100 measurements (12.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  4 (4.00%) high mild
  6 (6.00%) high severe
Benchmarking stream-cipher/apply_keystream/4096: Collecting 100 samples in estimstream-cipher/apply_keystream/4096
                        time:   [25017.0804 cycles 25018.6038 cycles 25020.4498 cycles]
                        thrpt:  [6.1085 cpb 6.1081 cpb 6.1077 cpb]
                 change:
                        time:   [-0.1001% -0.0744% -0.0551%] (p = 0.00 < 0.05)
                        thrpt:  [+0.0552% +0.0745% +0.1002%]
                        Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  3 (3.00%) high mild
  3 (3.00%) high severe
Benchmarking stream-cipher/apply_keystream/8192: Collecting 100 samples in estimstream-cipher/apply_keystream/8192
                        time:   [49925.9219 cycles 49929.2608 cycles 49933.1698 cycles]
                        thrpt:  [6.0954 cpb 6.0949 cpb 6.0945 cpb]
                 change:
                        time:   [-97.491% -92.833% -0.1794%] (p = 0.26 > 0.05)
                        thrpt:  [+0.1798% +1295.4% +3885.8%]
                        No change in performance detected.
Found 18 outliers among 100 measurements (18.00%)
  2 (2.00%) low severe
  1 (1.00%) low mild
  6 (6.00%) high mild
  9 (9.00%) high severe
Benchmarking stream-cipher/apply_keystream/16384: Collecting 100 samples in estistream-cipher/apply_keystream/16384
                        time:   [99615.5898 cycles 99625.5853 cycles 99638.3303 cycles]
                        thrpt:  [6.0814 cpb 6.0807 cpb 6.0801 cpb]
                 change:
                        time:   [-0.3273% -0.3106% -0.2928%] (p = 0.00 < 0.05)
                        thrpt:  [+0.2936% +0.3116% +0.3284%]
                        Change within noise threshold.
Found 14 outliers among 100 measurements (14.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  8 (8.00%) high mild
  4 (4.00%) high severe

## Rng
Benchmarking ChaCha20Rng/fill_bytes/1024: Collecting 100 samples in estimated 5.ChaCha20Rng/fill_bytes/1024
                        time:   [5982.4086 cycles 5983.2582 cycles 5984.2480 cycles]
                        thrpt:  [5.8440 cpb 5.8430 cpb 5.8422 cpb]
                 change:
                        time:   [-0.0491% +0.0017% +0.0526%] (p = 0.95 > 0.05)
                        thrpt:  [-0.0525% -0.0017% +0.0491%]
                        No change in performance detected.
Found 12 outliers among 100 measurements (12.00%)
  1 (1.00%) low mild
  5 (5.00%) high mild
  6 (6.00%) high severe
Benchmarking ChaCha20Rng/fill_bytes/2048: Collecting 100 samples in estimated 5.ChaCha20Rng/fill_bytes/2048
                        time:   [11953.7934 cycles 11954.4879 cycles 11955.3441 cycles]
                        thrpt:  [5.8376 cpb 5.8372 cpb 5.8368 cpb]
                 change:
                        time:   [+0.6667% +0.6799% +0.6952%] (p = 0.00 < 0.05)
                        thrpt:  [-0.6904% -0.6753% -0.6623%]
                        Change within noise threshold.
Found 19 outliers among 100 measurements (19.00%)
  2 (2.00%) low severe
  2 (2.00%) low mild
  4 (4.00%) high mild
  11 (11.00%) high severe
Benchmarking ChaCha20Rng/fill_bytes/4096: Collecting 100 samples in estimated 5.ChaCha20Rng/fill_bytes/4096
                        time:   [23901.7807 cycles 23903.0389 cycles 23904.5828 cycles]
                        thrpt:  [5.8361 cpb 5.8357 cpb 5.8354 cpb]
                 change:
                        time:   [+0.9727% +0.9869% +1.0080%] (p = 0.00 < 0.05)
                        thrpt:  [-0.9979% -0.9773% -0.9633%]
                        Change within noise threshold.
Found 12 outliers among 100 measurements (12.00%)
  1 (1.00%) low severe
  3 (3.00%) low mild
  4 (4.00%) high mild
  4 (4.00%) high severe
Benchmarking ChaCha20Rng/fill_bytes/8192: Collecting 100 samples in estimated 5.ChaCha20Rng/fill_bytes/8192
                        time:   [47801.7730 cycles 47805.1471 cycles 47809.2849 cycles]
                        thrpt:  [5.8361 cpb 5.8356 cpb 5.8352 cpb]
                 change:
                        time:   [+1.1205% +1.1375% +1.1696%] (p = 0.00 < 0.05)
                        thrpt:  [-1.1561% -1.1247% -1.1081%]
                        Performance has regressed.
Found 11 outliers among 100 measurements (11.00%)
  2 (2.00%) low severe
  1 (1.00%) low mild
  3 (3.00%) high mild
  5 (5.00%) high severe
Benchmarking ChaCha20Rng/fill_bytes/16384: Collecting 100 samples in estimated 5ChaCha20Rng/fill_bytes/16384
                        time:   [95618.6808 cycles 95633.1994 cycles 95651.4965 cycles]
                        thrpt:  [5.8381 cpb 5.8370 cpb 5.8361 cpb]
                 change:
                        time:   [+1.2957% +1.3295% +1.3725%] (p = 0.00 < 0.05)
                        thrpt:  [-1.3539% -1.3120% -1.2791%]
                        Performance has regressed.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe