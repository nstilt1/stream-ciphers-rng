# Benchmarks for chacha20 using AVX2 and zeroize for the backends
These benches were ran on 1/6/2024 using a Raspberry Pi Model 4b

## ChaCha20::apply_keystream()
```
     Running src/chacha20.rs (target/release/deps/chacha20-3e29041298637591)
Gnuplot not found, using plotters backend
Benchmarking stream-cipher/apply_keystream/1024: Collecting 100 samples in estimstream-cipher/apply_keystream/1024
                        time:   [6301.1456 cycles 6302.4779 cycles 6304.1368 cycles]
                        thrpt:  [6.1564 cpb 6.1548 cpb 6.1535 cpb]
                 change:
                        time:   [-0.0037% +0.0186% +0.0416%] (p = 0.11 > 0.05)
                        thrpt:  [-0.0416% -0.0186% +0.0037%]
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) high mild
  7 (7.00%) high severe
Benchmarking stream-cipher/apply_keystream/2048: Collecting 100 samples in estimstream-cipher/apply_keystream/2048
                        time:   [12545.0033 cycles 12545.5859 cycles 12546.3223 cycles]
                        thrpt:  [6.1261 cpb 6.1258 cpb 6.1255 cpb]
                 change:
                        time:   [-0.0061% +0.0062% +0.0220%] (p = 0.39 > 0.05)
                        thrpt:  [-0.0220% -0.0062% +0.0061%]
                        No change in performance detected.
Found 16 outliers among 100 measurements (16.00%)
  1 (1.00%) low severe
  4 (4.00%) low mild
  5 (5.00%) high mild
  6 (6.00%) high severe
Benchmarking stream-cipher/apply_keystream/4096: Collecting 100 samples in estimstream-cipher/apply_keystream/4096
                        time:   [25033.3870 cycles 25036.6206 cycles 25040.5965 cycles]
                        thrpt:  [6.1134 cpb 6.1125 cpb 6.1117 cpb]
                 change:
                        time:   [-0.1452% -0.0596% +0.0036%] (p = 0.12 > 0.05)
                        thrpt:  [-0.0036% +0.0597% +0.1454%]
                        No change in performance detected.
Found 13 outliers among 100 measurements (13.00%)
  1 (1.00%) low severe
  4 (4.00%) high mild
  8 (8.00%) high severe
Benchmarking stream-cipher/apply_keystream/8192: Collecting 100 samples in estimstream-cipher/apply_keystream/8192
                        time:   [50035.4146 cycles 403443.3712 cycles 1245575.7772 cycles]
                        thrpt:  [152.0478 cpb 49.2485 cpb 6.1078 cpb]
                 change:
                        time:   [+0.1562% +1295.0% +3884.6%] (p = 0.26 > 0.05)
                        thrpt:  [-97.490% -92.832% -0.1560%]
                        No change in performance detected.
Found 11 outliers among 100 measurements (11.00%)
  2 (2.00%) high mild
  9 (9.00%) high severe
Benchmarking stream-cipher/apply_keystream/16384: Collecting 100 samples in estistream-cipher/apply_keystream/16384
                        time:   [99929.5809 cycles 99938.1050 cycles 99949.3843 cycles]
                        thrpt:  [6.1004 cpb 6.0997 cpb 6.0992 cpb]
                 change:
                        time:   [+0.0493% +0.0680% +0.0858%] (p = 0.00 < 0.05)
                        thrpt:  [-0.0857% -0.0679% -0.0493%]
                        Change within noise threshold.
Found 16 outliers among 100 measurements (16.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  6 (6.00%) high mild
  7 (7.00%) high severe
```
## ChaCha20Rng::fill_bytes()
```
Benchmarking ChaCha20Rng/fill_bytes/1024: Collecting 100 samples in estimated 5.ChaCha20Rng/fill_bytes/1024
                        time:   [5981.0663 cycles 5983.9321 cycles 5987.0792 cycles]
                        thrpt:  [5.8468 cpb 5.8437 cpb 5.8409 cpb]
                 change:
                        time:   [-0.1396% -0.0641% +0.0078%] (p = 0.09 > 0.05)
                        thrpt:  [-0.0078% +0.0642% +0.1398%]
                        No change in performance detected.
Benchmarking ChaCha20Rng/fill_bytes/2048: Collecting 100 samples in estimated 5.ChaCha20Rng/fill_bytes/2048
                        time:   [11873.3096 cycles 11874.0110 cycles 11874.8390 cycles]
                        thrpt:  [5.7983 cpb 5.7979 cpb 5.7975 cpb]
                 change:
                        time:   [-0.0392% -0.0075% +0.0133%] (p = 0.69 > 0.05)
                        thrpt:  [-0.0133% +0.0075% +0.0392%]
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  6 (6.00%) high severe
Benchmarking ChaCha20Rng/fill_bytes/4096: Collecting 100 samples in estimated 5.ChaCha20Rng/fill_bytes/4096
                        time:   [23670.0071 cycles 23671.0129 cycles 23672.3867 cycles]
                        thrpt:  [5.7794 cpb 5.7791 cpb 5.7788 cpb]
                 change:
                        time:   [-1.3306% -0.5141% -0.0707%] (p = 0.18 > 0.05)
                        thrpt:  [+0.0708% +0.5167% +1.3485%]
                        No change in performance detected.
Found 13 outliers among 100 measurements (13.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  4 (4.00%) high mild
  6 (6.00%) high severe
Benchmarking ChaCha20Rng/fill_bytes/8192: Collecting 100 samples in estimated 5.ChaCha20Rng/fill_bytes/8192
                        time:   [47267.9611 cycles 47270.5865 cycles 47273.7164 cycles]
                        thrpt:  [5.7707 cpb 5.7703 cpb 5.7700 cpb]
                 change:
                        time:   [-0.2916% -0.1595% -0.0591%] (p = 0.00 < 0.05)
                        thrpt:  [+0.0592% +0.1597% +0.2924%]
                        Change within noise threshold.
Found 18 outliers among 100 measurements (18.00%)
  2 (2.00%) low severe
  1 (1.00%) low mild
  7 (7.00%) high mild
  8 (8.00%) high severe
Benchmarking ChaCha20Rng/fill_bytes/16384: Collecting 100 samples in estimated 5ChaCha20Rng/fill_bytes/16384
                        time:   [94463.7918 cycles 94470.7933 cycles 94479.0512 cycles]
                        thrpt:  [5.7665 cpb 5.7660 cpb 5.7656 cpb]
                 change:
                        time:   [-0.1784% -0.0801% -0.0101%] (p = 0.05 < 0.05)
                        thrpt:  [+0.0101% +0.0801% +0.1787%]
                        Change within noise threshold.
Found 15 outliers among 100 measurements (15.00%)
  3 (3.00%) low severe
  1 (1.00%) low mild
  5 (5.00%) high mild
  6 (6.00%) high severe
```