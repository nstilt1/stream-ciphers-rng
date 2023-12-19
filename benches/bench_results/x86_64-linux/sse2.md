# Benchmarks for chacha20 using SSE2
These benches were ran on 12/16/2023 using a 3.0 GHz i9 CPU.
This bench is compared against the last one, commit: 378303a
This commit: 97cdfc5

## ChaCha20::apply_keystream()
```
     Running src/chacha20.rs (target/release/deps/chacha20-5e4721a497c9e862)
Gnuplot not found, using plotters backend
stream-cipher/apply_keystream/1024
                        time:   [3012.0120 cycles 3016.8319 cycles 3021.9428 cycles]
                        thrpt:  [2.9511 cpb 2.9461 cpb 2.9414 cpb]
                 change:
                        time:   [-0.5988% -0.2426% +0.0942%] (p = 0.18 > 0.05)
                        thrpt:  [-0.0942% +0.2432% +0.6024%]
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
stream-cipher/apply_keystream/2048
                        time:   [6008.8553 cycles 6018.8299 cycles 6028.8187 cycles]
                        thrpt:  [2.9438 cpb 2.9389 cpb 2.9340 cpb]
                 change:
                        time:   [-1.3615% -0.8991% -0.4467%] (p = 0.00 < 0.05)
                        thrpt:  [+0.4487% +0.9073% +1.3803%]
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe
stream-cipher/apply_keystream/4096
                        time:   [11965.3329 cycles 11983.8200 cycles 12003.1489 cycles]
                        thrpt:  [2.9305 cpb 2.9257 cpb 2.9212 cpb]
                 change:
                        time:   [-0.5504% -0.2386% +0.0489%] (p = 0.12 > 0.05)
                        thrpt:  [-0.0489% +0.2392% +0.5534%]
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild
stream-cipher/apply_keystream/8192
                        time:   [23964.2812 cycles 24010.5128 cycles 24063.7286 cycles]
                        thrpt:  [2.9375 cpb 2.9310 cpb 2.9253 cpb]
                 change:
                        time:   [-0.2233% +0.0619% +0.3784%] (p = 0.69 > 0.05)
                        thrpt:  [-0.3769% -0.0619% +0.2238%]
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe
stream-cipher/apply_keystream/16384
                        time:   [47903.4369 cycles 47981.2118 cycles 48065.2043 cycles]
                        thrpt:  [2.9337 cpb 2.9285 cpb 2.9238 cpb]
                 change:
                        time:   [-0.4035% -0.0307% +0.3812%] (p = 0.88 > 0.05)
                        thrpt:  [-0.3798% +0.0307% +0.4051%]
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
```

## ChaCha20Rng::fill_bytes()
```
ChaCha20Rng/fill_bytes/1024
                        time:   [2962.6298 cycles 2967.0192 cycles 2971.3807 cycles]
                        thrpt:  [2.9017 cpb 2.8975 cpb 2.8932 cpb]
                 change:
                        time:   [-0.5213% -0.2739% -0.0469%] (p = 0.03 < 0.05)
                        thrpt:  [+0.0469% +0.2747% +0.5240%]
                        Change within noise threshold.
ChaCha20Rng/fill_bytes/2048
                        time:   [5904.8629 cycles 5916.3262 cycles 5928.6479 cycles]
                        thrpt:  [2.8948 cpb 2.8888 cpb 2.8832 cpb]
                 change:
                        time:   [-0.4387% -0.2261% +0.0017%] (p = 0.04 < 0.05)
                        thrpt:  [-0.0017% +0.2266% +0.4406%]
                        Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
ChaCha20Rng/fill_bytes/4096
                        time:   [11799.0067 cycles 11819.8360 cycles 11841.6055 cycles]
                        thrpt:  [2.8910 cpb 2.8857 cpb 2.8806 cpb]
                 change:
                        time:   [-0.7938% -0.4824% -0.1934%] (p = 0.00 < 0.05)
                        thrpt:  [+0.1938% +0.4847% +0.8002%]
                        Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
ChaCha20Rng/fill_bytes/8192
                        time:   [23644.0812 cycles 23677.1730 cycles 23712.3274 cycles]
                        thrpt:  [2.8946 cpb 2.8903 cpb 2.8862 cpb]
                 change:
                        time:   [+0.0624% +0.2979% +0.5429%] (p = 0.02 < 0.05)
                        thrpt:  [-0.5400% -0.2970% -0.0624%]
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe
ChaCha20Rng/fill_bytes/16384
                        time:   [47261.9033 cycles 47332.0914 cycles 47407.6803 cycles]
                        thrpt:  [2.8935 cpb 2.8889 cpb 2.8846 cpb]
                 change:
                        time:   [-0.1175% +0.1162% +0.3779%] (p = 0.34 > 0.05)
                        thrpt:  [-0.3765% -0.1161% +0.1176%]
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe
```