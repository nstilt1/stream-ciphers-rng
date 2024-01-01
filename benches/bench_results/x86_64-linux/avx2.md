# Benchmarks for chacha20 using AVX2
These benches were ran on 1/1/2024 using a 3.0 GHz i9 CPU.

## ChaCha20::apply_keystream()
```
     Running src/chacha20.rs (target/release/deps/chacha20-6f837c27da8cfee7)
Gnuplot not found, using plotters backend
stream-cipher/apply_keystream/1024
                        time:   [1152.4690 cycles 1156.9226 cycles 1161.9308 cycles]
                        thrpt:  [1.1347 cpb 1.1298 cpb 1.1255 cpb]
                 change:
                        time:   [+0.0152% +0.6133% +1.2866%] (p = 0.07 > 0.05)
                        thrpt:  [-1.2703% -0.6095% -0.0152%]
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  7 (7.00%) high mild
  1 (1.00%) high severe
stream-cipher/apply_keystream/2048
                        time:   [2307.4544 cycles 2322.0052 cycles 2340.8519 cycles]
                        thrpt:  [1.1430 cpb 1.1338 cpb 1.1267 cpb]
                 change:
                        time:   [-0.5248% +0.1823% +0.9112%] (p = 0.64 > 0.05)
                        thrpt:  [-0.9030% -0.1819% +0.5275%]
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  6 (6.00%) high mild
  3 (3.00%) high severe
stream-cipher/apply_keystream/4096
                        time:   [4602.0880 cycles 4630.4334 cycles 4664.8997 cycles]
                        thrpt:  [1.1389 cpb 1.1305 cpb 1.1236 cpb]
                 change:
                        time:   [-1.1346% -0.3209% +0.4320%] (p = 0.43 > 0.05)
                        thrpt:  [-0.4302% +0.3220% +1.1477%]
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe
stream-cipher/apply_keystream/8192
                        time:   [9205.0368 cycles 9258.2379 cycles 9318.1269 cycles]
                        thrpt:  [1.1375 cpb 1.1302 cpb 1.1237 cpb]
                 change:
                        time:   [-0.8864% -0.2562% +0.3682%] (p = 0.43 > 0.05)
                        thrpt:  [-0.3668% +0.2568% +0.8943%]
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe
stream-cipher/apply_keystream/16384
                        time:   [18434.3291 cycles 18498.9081 cycles 18568.1721 cycles]
                        thrpt:  [1.1333 cpb 1.1291 cpb 1.1251 cpb]
                 change:
                        time:   [-1.2878% -0.3832% +0.4634%] (p = 0.41 > 0.05)
                        thrpt:  [-0.4613% +0.3847% +1.3046%]
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe
```
## ChaCha20Rng::fill_bytes()
```
ChaCha20Rng/fill_bytes/1024
                        time:   [1149.5890 cycles 1157.2002 cycles 1166.3934 cycles]
                        thrpt:  [1.1391 cpb 1.1301 cpb 1.1226 cpb]
                 change:
                        time:   [+0.0062% +0.7555% +1.4906%] (p = 0.06 > 0.05)
                        thrpt:  [-1.4687% -0.7498% -0.0062%]
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe
ChaCha20Rng/fill_bytes/2048
                        time:   [2305.7542 cycles 2329.6436 cycles 2359.5068 cycles]
                        thrpt:  [1.1521 cpb 1.1375 cpb 1.1259 cpb]
                 change:
                        time:   [-0.3312% +0.7713% +1.8437%] (p = 0.17 > 0.05)
                        thrpt:  [-1.8104% -0.7654% +0.3323%]
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe
ChaCha20Rng/fill_bytes/4096
                        time:   [4595.9961 cycles 4621.5508 cycles 4650.8701 cycles]
                        thrpt:  [1.1355 cpb 1.1283 cpb 1.1221 cpb]
                 change:
                        time:   [-0.1964% +0.5460% +1.3161%] (p = 0.16 > 0.05)
                        thrpt:  [-1.2990% -0.5431% +0.1968%]
                        No change in performance detected.
Found 11 outliers among 100 measurements (11.00%)
  7 (7.00%) high mild
  4 (4.00%) high severe
ChaCha20Rng/fill_bytes/8192
                        time:   [9124.0933 cycles 9177.3612 cycles 9244.0411 cycles]
                        thrpt:  [1.1284 cpb 1.1203 cpb 1.1138 cpb]
                 change:
                        time:   [-2.3674% -1.4260% -0.5259%] (p = 0.00 < 0.05)
                        thrpt:  [+0.5287% +1.4466% +2.4248%]
                        Change within noise threshold.
Found 10 outliers among 100 measurements (10.00%)
  4 (4.00%) high mild
  6 (6.00%) high severe
ChaCha20Rng/fill_bytes/16384
                        time:   [18362.5814 cycles 18419.4005 cycles 18483.4119 cycles]
                        thrpt:  [1.1281 cpb 1.1242 cpb 1.1208 cpb]
                 change:
                        time:   [+0.6157% +1.5262% +2.6330%] (p = 0.00 < 0.05)
                        thrpt:  [-2.5655% -1.5033% -0.6120%]
                        Change within noise threshold.
Found 11 outliers among 100 measurements (11.00%)
  7 (7.00%) high mild
  4 (4.00%) high severe
```