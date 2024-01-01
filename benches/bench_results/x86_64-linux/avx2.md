# Benchmarks for chacha20 using AVX2
These benches were ran on 12/18/2023 using a 3.0 GHz i9 CPU.
This bench is compared against the last one, commit: 378303a
This commit: 97cdfc5

## ChaCha20::apply_keystream()
```
     Running src/chacha20.rs (target/release/deps/chacha20-6f837c27da8cfee7)
Gnuplot not found, using plotters backend
stream-cipher/apply_keystream/1024
                        time:   [1148.1979 cycles 1153.0922 cycles 1158.5350 cycles]
                        thrpt:  [1.1314 cpb 1.1261 cpb 1.1213 cpb]
                 change:
                        time:   [+19.937% +20.306% +20.722%] (p = 0.00 < 0.05)
                        thrpt:  [-17.165% -16.878% -16.623%]
                        Performance has regressed.
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe
stream-cipher/apply_keystream/2048
                        time:   [2302.0364 cycles 2313.7749 cycles 2326.8890 cycles]
                        thrpt:  [1.1362 cpb 1.1298 cpb 1.1240 cpb]
                 change:
                        time:   [+20.132% +20.788% +21.475%] (p = 0.00 < 0.05)
                        thrpt:  [-17.679% -17.210% -16.758%]
                        Performance has regressed.
Found 14 outliers among 100 measurements (14.00%)
  7 (7.00%) high mild
  7 (7.00%) high severe
stream-cipher/apply_keystream/4096
                        time:   [4613.3004 cycles 4647.7296 cycles 4694.0007 cycles]
                        thrpt:  [1.1460 cpb 1.1347 cpb 1.1263 cpb]
                 change:
                        time:   [+21.330% +22.070% +22.891%] (p = 0.00 < 0.05)
                        thrpt:  [-18.627% -18.080% -17.580%]
                        Performance has regressed.
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe
stream-cipher/apply_keystream/8192
                        time:   [9225.6952 cycles 9272.7785 cycles 9329.4163 cycles]
                        thrpt:  [1.1388 cpb 1.1319 cpb 1.1262 cpb]
                 change:
                        time:   [+18.278% +19.111% +19.885%] (p = 0.00 < 0.05)
                        thrpt:  [-16.587% -16.045% -15.453%]
                        Performance has regressed.
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe
stream-cipher/apply_keystream/16384
                        time:   [18454.1270 cycles 18557.8582 cycles 18676.2222 cycles]
                        thrpt:  [1.1399 cpb 1.1327 cpb 1.1264 cpb]
                 change:
                        time:   [+17.981% +19.335% +20.615%] (p = 0.00 < 0.05)
                        thrpt:  [-17.092% -16.202% -15.240%]
                        Performance has regressed.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe
```
## ChaCha20Rng::fill_bytes()
```
ChaCha20Rng/fill_bytes/1024
                        time:   [1137.9394 cycles 1144.2750 cycles 1152.1062 cycles]
                        thrpt:  [1.1251 cpb 1.1175 cpb 1.1113 cpb]
                 change:
                        time:   [+17.463% +18.587% +19.616%] (p = 0.00 < 0.05)
                        thrpt:  [-16.399% -15.674% -14.867%]
                        Performance has regressed.
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe
ChaCha20Rng/fill_bytes/2048
                        time:   [2281.6523 cycles 2288.7452 cycles 2296.2144 cycles]
                        thrpt:  [1.1212 cpb 1.1176 cpb 1.1141 cpb]
                 change:
                        time:   [+19.938% +20.686% +21.619%] (p = 0.00 < 0.05)
                        thrpt:  [-17.776% -17.141% -16.623%]
                        Performance has regressed.
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe
ChaCha20Rng/fill_bytes/4096
                        time:   [4574.3231 cycles 4600.0175 cycles 4630.0494 cycles]
                        thrpt:  [1.1304 cpb 1.1231 cpb 1.1168 cpb]
                 change:
                        time:   [+18.721% +19.524% +20.364%] (p = 0.00 < 0.05)
                        thrpt:  [-16.919% -16.335% -15.769%]
                        Performance has regressed.
Found 11 outliers among 100 measurements (11.00%)
  4 (4.00%) high mild
  7 (7.00%) high severe
ChaCha20Rng/fill_bytes/8192
                        time:   [9185.7479 cycles 9256.1322 cycles 9339.3566 cycles]
                        thrpt:  [1.1401 cpb 1.1299 cpb 1.1213 cpb]
                 change:
                        time:   [+19.503% +20.667% +21.980%] (p = 0.00 < 0.05)
                        thrpt:  [-18.020% -17.127% -16.320%]
                        Performance has regressed.
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) high mild
  6 (6.00%) high severe
ChaCha20Rng/fill_bytes/16384
                        time:   [18179.7827 cycles 18266.0506 cycles 18362.6824 cycles]
                        thrpt:  [1.1208 cpb 1.1149 cpb 1.1096 cpb]
                 change:
                        time:   [+18.141% +18.982% +19.874%] (p = 0.00 < 0.05)
                        thrpt:  [-16.579% -15.954% -15.356%]
                        Performance has regressed.
Found 10 outliers among 100 measurements (10.00%)
  4 (4.00%) high mild
  6 (6.00%) high severe
```