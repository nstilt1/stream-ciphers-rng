# Benchmarks for chacha20 using AVX2
These benches were ran on 1/1/2024 using a 3.0 GHz i9 CPU.

## ChaCha20::apply_keystream()
```
     Running src/chacha20.rs (target/release/deps/chacha20-6f837c27da8cfee7)
Gnuplot not found, using plotters backend
stream-cipher/apply_keystream/1024
                        time:   [1125.8539 cycles 1128.3709 cycles 1131.4741 cycles]
                        thrpt:  [1.1050 cpb 1.1019 cpb 1.0995 cpb]
                 change:
                        time:   [-3.2749% -2.6639% -2.1177%] (p = 0.00 < 0.05)
                        thrpt:  [+2.1635% +2.7368% +3.3858%]
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe
stream-cipher/apply_keystream/2048
                        time:   [2253.1506 cycles 2257.3444 cycles 2261.4883 cycles]
                        thrpt:  [1.1042 cpb 1.1022 cpb 1.1002 cpb]
                 change:
                        time:   [-3.3375% -2.7315% -2.1976%] (p = 0.00 < 0.05)
                        thrpt:  [+2.2469% +2.8082% +3.4527%]
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
stream-cipher/apply_keystream/4096
                        time:   [4518.3306 cycles 4526.5547 cycles 4535.2020 cycles]
                        thrpt:  [1.1072 cpb 1.1051 cpb 1.1031 cpb]
                 change:
                        time:   [-2.6248% -2.0474% -1.4816%] (p = 0.00 < 0.05)
                        thrpt:  [+1.5039% +2.0902% +2.6955%]
                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe
stream-cipher/apply_keystream/8192
                        time:   [9039.2983 cycles 9056.9183 cycles 9074.7142 cycles]
                        thrpt:  [1.1078 cpb 1.1056 cpb 1.1034 cpb]
                 change:
                        time:   [-2.5031% -2.0098% -1.5602%] (p = 0.00 < 0.05)
                        thrpt:  [+1.5849% +2.0510% +2.5674%]
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
stream-cipher/apply_keystream/16384
                        time:   [18037.0334 cycles 18074.3552 cycles 18116.5701 cycles]
                        thrpt:  [1.1057 cpb 1.1032 cpb 1.1009 cpb]
                 change:
                        time:   [-3.0600% -2.4140% -1.8922%] (p = 0.00 < 0.05)
                        thrpt:  [+1.9287% +2.4737% +3.1566%]
                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe
```
## ChaCha20Rng::fill_bytes()
```
ChaCha20Rng/fill_bytes/1024
                        time:   [1140.8323 cycles 1145.8247 cycles 1152.8514 cycles]
                        thrpt:  [1.1258 cpb 1.1190 cpb 1.1141 cpb]
                 change:
                        time:   [-1.7821% -1.1060% -0.4475%] (p = 0.00 < 0.05)
                        thrpt:  [+0.4495% +1.1184% +1.8144%]
                        Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe
ChaCha20Rng/fill_bytes/2048
                        time:   [2282.4429 cycles 2299.3906 cycles 2319.4310 cycles]
                        thrpt:  [1.1325 cpb 1.1227 cpb 1.1145 cpb]
                 change:
                        time:   [-2.1280% -1.0898% -0.1263%] (p = 0.03 < 0.05)
                        thrpt:  [+0.1264% +1.1018% +2.1743%]
                        Change within noise threshold.
Found 13 outliers among 100 measurements (13.00%)
  5 (5.00%) high mild
  8 (8.00%) high severe
ChaCha20Rng/fill_bytes/4096
                        time:   [4544.1053 cycles 4562.0204 cycles 4588.2801 cycles]
                        thrpt:  [1.1202 cpb 1.1138 cpb 1.1094 cpb]
                 change:
                        time:   [-1.8992% -1.2321% -0.6081%] (p = 0.00 < 0.05)
                        thrpt:  [+0.6118% +1.2474% +1.9360%]
                        Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
  6 (6.00%) high mild
  3 (3.00%) high severe
ChaCha20Rng/fill_bytes/8192
                        time:   [9143.9108 cycles 9177.6034 cycles 9214.2306 cycles]
                        thrpt:  [1.1248 cpb 1.1203 cpb 1.1162 cpb]
                 change:
                        time:   [-0.5226% +0.0966% +0.6789%] (p = 0.77 > 0.05)
                        thrpt:  [-0.6743% -0.0965% +0.5254%]
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe
ChaCha20Rng/fill_bytes/16384
                        time:   [18344.9755 cycles 18432.9591 cycles 18539.7267 cycles]
                        thrpt:  [1.1316 cpb 1.1251 cpb 1.1197 cpb]
                 change:
                        time:   [-1.5752% -0.4966% +0.4756%] (p = 0.36 > 0.05)
                        thrpt:  [-0.4733% +0.4990% +1.6004%]
                        No change in performance detected.
Found 10 outliers among 100 measurements (10.00%)
  5 (5.00%) high mild
  5 (5.00%) high severe
```