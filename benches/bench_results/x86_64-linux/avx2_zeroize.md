# Benchmarks for chacha20 using AVX2
These benches were ran on 1/1/2024 using a 3.0 GHz i9 CPU.
This bench is compared against not using the `zeroize` feature, and `apply_keystream()` did not involve `zeroize` in `avx2.rs`.

## ChaCha20::apply_keystream()
```
     Running src/chacha20.rs (target/release/deps/chacha20-e956f9ace8e82945)
Gnuplot not found, using plotters backend
stream-cipher/apply_keystream/1024
                        time:   [1011.4205 cycles 1020.6545 cycles 1031.1495 cycles]
                        thrpt:  [1.0070 cpb 0.9967 cpb 0.9877 cpb]
                 change:
                        time:   [+1.3119% +2.1247% +3.0625%] (p = 0.00 < 0.05)
                        thrpt:  [-2.9715% -2.0805% -1.2949%]
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe
stream-cipher/apply_keystream/2048
                        time:   [1980.8355 cycles 1992.6469 cycles 2006.7129 cycles]
                        thrpt:  [0.9798 cpb 0.9730 cpb 0.9672 cpb]
                 change:
                        time:   [+0.0718% +1.3052% +2.7021%] (p = 0.05 < 0.05)
                        thrpt:  [-2.6310% -1.2884% -0.0718%]
                        Change within noise threshold.
Found 12 outliers among 100 measurements (12.00%)
  6 (6.00%) high mild
  6 (6.00%) high severe
stream-cipher/apply_keystream/4096
                        time:   [3897.8477 cycles 3927.7687 cycles 3963.7925 cycles]
                        thrpt:  [0.9677 cpb 0.9589 cpb 0.9516 cpb]
                 change:
                        time:   [+0.0645% +1.1127% +2.4382%] (p = 0.05 < 0.05)
                        thrpt:  [-2.3802% -1.1004% -0.0645%]
                        Change within noise threshold.
Found 12 outliers among 100 measurements (12.00%)
  4 (4.00%) high mild
  8 (8.00%) high severe
stream-cipher/apply_keystream/8192
                        time:   [7769.8234 cycles 7846.2751 cycles 7946.5980 cycles]
                        thrpt:  [0.9700 cpb 0.9578 cpb 0.9485 cpb]
                 change:
                        time:   [-0.3084% +0.5498% +1.5702%] (p = 0.26 > 0.05)
                        thrpt:  [-1.5459% -0.5468% +0.3094%]
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe
stream-cipher/apply_keystream/16384
                        time:   [15560.3310 cycles 15727.9992 cycles 15915.7157 cycles]
                        thrpt:  [0.9714 cpb 0.9600 cpb 0.9497 cpb]
                 change:
                        time:   [-1.7637% -0.6182% +0.4095%] (p = 0.29 > 0.05)
                        thrpt:  [-0.4079% +0.6221% +1.7953%]
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe
```
## ChaCha20Rng::fill_bytes()
```
ChaCha20Rng/fill_bytes/1024
                        time:   [1039.1623 cycles 1044.2221 cycles 1049.7263 cycles]
                        thrpt:  [1.0251 cpb 1.0197 cpb 1.0148 cpb]
                 change:
                        time:   [+4.6186% +5.4148% +6.1633%] (p = 0.00 < 0.05)
                        thrpt:  [-5.8055% -5.1367% -4.4147%]
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe
ChaCha20Rng/fill_bytes/2048
                        time:   [2057.7115 cycles 2069.1091 cycles 2082.4978 cycles]
                        thrpt:  [1.0168 cpb 1.0103 cpb 1.0047 cpb]
                 change:
                        time:   [+4.4959% +5.6141% +6.7549%] (p = 0.00 < 0.05)
                        thrpt:  [-6.3275% -5.3156% -4.3024%]
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe
ChaCha20Rng/fill_bytes/4096
                        time:   [4077.4161 cycles 4101.4425 cycles 4130.3240 cycles]
                        thrpt:  [1.0084 cpb 1.0013 cpb 0.9955 cpb]
                 change:
                        time:   [+3.9245% +4.7690% +5.6001%] (p = 0.00 < 0.05)
                        thrpt:  [-5.3031% -4.5519% -3.7763%]
                        Performance has regressed.
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe
ChaCha20Rng/fill_bytes/8192
                        time:   [8066.9012 cycles 8099.7997 cycles 8135.7708 cycles]
                        thrpt:  [0.9931 cpb 0.9887 cpb 0.9847 cpb]
                 change:
                        time:   [+3.1380% +4.2721% +5.5699%] (p = 0.00 < 0.05)
                        thrpt:  [-5.2761% -4.0971% -3.0425%]
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe
ChaCha20Rng/fill_bytes/16384
                        time:   [16183.5630 cycles 16298.2466 cycles 16438.6144 cycles]
                        thrpt:  [1.0033 cpb 0.9948 cpb 0.9878 cpb]
                 change:
                        time:   [+3.4820% +4.4552% +5.3629%] (p = 0.00 < 0.05)
                        thrpt:  [-5.0900% -4.2652% -3.3649%]
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe
```