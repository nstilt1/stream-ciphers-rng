# Benchmarks for chacha20 using AVX2
These benches were ran on 12/16/2023 using a 3.0 GHz i9 CPU, immediately after a restart. Prior to the restart, it was slightly slower.
Last commit: 378303a

## ChaCha20::apply_keystream()
```
     Running src/chacha20.rs (target/release/deps/chacha20-5e4721a497c9e862)
Gnuplot not found, using plotters backend
stream-cipher/apply_keystream/1024
                        time:   [973.0466 cycles 974.7631 cycles 976.5678 cycles]
                        thrpt:  [0.9537 cpb 0.9519 cpb 0.9502 cpb]
                 change:
                        time:   [-0.6291% -0.2851% +0.1212%] (p = 0.11 > 0.05)
                        thrpt:  [-0.1210% +0.2859% +0.6331%]
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe
stream-cipher/apply_keystream/2048
                        time:   [1917.6186 cycles 1920.8383 cycles 1924.4000 cycles]
                        thrpt:  [0.9396 cpb 0.9379 cpb 0.9363 cpb]
                 change:
                        time:   [-0.2162% -0.0147% +0.2141%] (p = 0.90 > 0.05)
                        thrpt:  [-0.2137% +0.0147% +0.2167%]
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe
stream-cipher/apply_keystream/4096
                        time:   [3827.6324 cycles 3840.9538 cycles 3855.5713 cycles]
                        thrpt:  [0.9413 cpb 0.9377 cpb 0.9345 cpb]
                 change:
                        time:   [-0.0123% +0.2342% +0.5073%] (p = 0.09 > 0.05)
                        thrpt:  [-0.5047% -0.2337% +0.0123%]
                        No change in performance detected.
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild
stream-cipher/apply_keystream/8192
                        time:   [7639.1288 cycles 7658.8594 cycles 7679.5823 cycles]
                        thrpt:  [0.9374 cpb 0.9349 cpb 0.9325 cpb]
                 change:
                        time:   [+0.5371% +0.8539% +1.2006%] (p = 0.00 < 0.05)
                        thrpt:  [-1.1863% -0.8466% -0.5342%]
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
stream-cipher/apply_keystream/16384
                        time:   [15228.0875 cycles 15263.4519 cycles 15302.2441 cycles]
                        thrpt:  [0.9340 cpb 0.9316 cpb 0.9294 cpb]
                 change:
                        time:   [-0.1512% +0.4536% +0.9783%] (p = 0.13 > 0.05)
                        thrpt:  [-0.9688% -0.4516% +0.1514%]
                        No change in performance detected.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
```
## ChaCha20Rng::fill_bytes()
```
ChaCha20Rng/fill_bytes/1024
                        time:   [975.8402 cycles 977.9858 cycles 980.2349 cycles]
                        thrpt:  [0.9573 cpb 0.9551 cpb 0.9530 cpb]
                 change:
                        time:   [-4.9948% -4.7310% -4.4769%] (p = 0.00 < 0.05)
                        thrpt:  [+4.6867% +4.9660% +5.2574%]
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
ChaCha20Rng/fill_bytes/2048
                        time:   [1927.2140 cycles 1931.7218 cycles 1936.2816 cycles]
                        thrpt:  [0.9454 cpb 0.9432 cpb 0.9410 cpb]
                 change:
                        time:   [-6.8659% -6.4123% -5.9901%] (p = 0.00 < 0.05)
                        thrpt:  [+6.3718% +6.8516% +7.3720%]
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe
ChaCha20Rng/fill_bytes/4096
                        time:   [3829.5934 cycles 3845.9512 cycles 3869.8438 cycles]
                        thrpt:  [0.9448 cpb 0.9390 cpb 0.9350 cpb]
                 change:
                        time:   [-7.3485% -7.0100% -6.6283%] (p = 0.00 < 0.05)
                        thrpt:  [+7.0989% +7.5385% +7.9313%]
                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe
ChaCha20Rng/fill_bytes/8192
                        time:   [7629.2859 cycles 7683.6478 cycles 7752.3442 cycles]
                        thrpt:  [0.9463 cpb 0.9379 cpb 0.9313 cpb]
                 change:
                        time:   [-9.4952% -8.3524% -7.5616%] (p = 0.00 < 0.05)
                        thrpt:  [+8.1802% +9.1136% +10.491%]
                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe
ChaCha20Rng/fill_bytes/16384
                        time:   [15216.5871 cycles 15287.2586 cycles 15398.1009 cycles]
                        thrpt:  [0.9398 cpb 0.9331 cpb 0.9287 cpb]
                 change:
                        time:   [-7.7079% -7.3395% -6.8891%] (p = 0.00 < 0.05)
                        thrpt:  [+7.3988% +7.9208% +8.3516%]
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
```