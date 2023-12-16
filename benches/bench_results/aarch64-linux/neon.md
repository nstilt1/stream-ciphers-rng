# Benchmarks for chacha20 using NEON
These benches were ran on 12/16/2023 using a Raspberry Pi 4b.
The results are compared against the soft implementation.
Last commit: 378303a

## ChaCha20::apply_keystream()
```
     Running src/chacha20.rs (target/release/deps/chacha20-066064f66a8e404e)
Gnuplot not found, using plotters backend
Benchmarking stream-cipher/apply_keystream/1024: Collecting 100 samples in estimstream-cipher/apply_keystream/1024
                        time:   [5662.3672 cycles 5665.4092 cycles 5668.7779 cycles]
                        thrpt:  [5.5359 cpb 5.5326 cpb 5.5297 cpb]
                 change:
                        time:   [-76.812% -76.796% -76.777%] (p = 0.00 < 0.05)
                        thrpt:  [+330.61% +330.96% +331.25%]
                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe
Benchmarking stream-cipher/apply_keystream/2048: Collecting 100 samples in estimstream-cipher/apply_keystream/2048
                        time:   [11235.1394 cycles 11235.6350 cycles 11236.2109 cycles]
                        thrpt:  [5.4864 cpb 5.4861 cpb 5.4859 cpb]
                 change:
                        time:   [-77.025% -76.999% -76.980%] (p = 0.00 < 0.05)
                        thrpt:  [+334.40% +334.77% +335.25%]
                        Performance has improved.
Found 19 outliers among 100 measurements (19.00%)
  3 (3.00%) low severe
  2 (2.00%) low mild
  7 (7.00%) high mild
  7 (7.00%) high severe
Benchmarking stream-cipher/apply_keystream/4096: Collecting 100 samples in estimstream-cipher/apply_keystream/4096
                        time:   [22401.5833 cycles 22403.5092 cycles 22406.1654 cycles]
                        thrpt:  [5.4703 cpb 5.4696 cpb 5.4691 cpb]
                 change:
                        time:   [-77.044% -77.035% -77.022%] (p = 0.00 < 0.05)
                        thrpt:  [+335.21% +335.45% +335.61%]
                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  4 (4.00%) high mild
  7 (7.00%) high severe
Benchmarking stream-cipher/apply_keystream/8192: Collecting 100 samples in estimstream-cipher/apply_keystream/8192
                        time:   [44734.2170 cycles 44736.4793 cycles 44739.0993 cycles]
                        thrpt:  [5.4613 cpb 5.4610 cpb 5.4607 cpb]
                 change:
                        time:   [-77.073% -77.070% -77.067%] (p = 0.00 < 0.05)
                        thrpt:  [+336.05% +336.10% +336.16%]
                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  4 (4.00%) high mild
  7 (7.00%) high severe
Benchmarking stream-cipher/apply_keystream/16384: Collecting 100 samples in estistream-cipher/apply_keystream/16384
                        time:   [90187.5841 cycles 90192.3090 cycles 90198.1297 cycles]
                        thrpt:  [5.5053 cpb 5.5049 cpb 5.5046 cpb]
                 change:
                        time:   [-76.920% -76.914% -76.908%] (p = 0.00 < 0.05)
                        thrpt:  [+333.04% +333.17% +333.28%]
                        Performance has improved.
Found 16 outliers among 100 measurements (16.00%)
  4 (4.00%) low severe
  3 (3.00%) low mild
  3 (3.00%) high mild
  6 (6.00%) high severe
```
## ChaCha20Rng::fill_bytes()
```
Benchmarking ChaCha20Rng/fill_bytes/1024: Collecting 100 samples in estimated 5.ChaCha20Rng/fill_bytes/1024
                        time:   [5579.7789 cycles 5580.5430 cycles 5581.2574 cycles]
                        thrpt:  [5.4504 cpb 5.4497 cpb 5.4490 cpb]
                 change:
                        time:   [-76.957% -76.934% -76.913%] (p = 0.00 < 0.05)
                        thrpt:  [+333.15% +333.53% +333.98%]
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) low severe
  2 (2.00%) low mild
Benchmarking ChaCha20Rng/fill_bytes/2048: Collecting 100 samples in estimated 5.ChaCha20Rng/fill_bytes/2048
                        time:   [11139.7581 cycles 11140.0942 cycles 11140.4842 cycles]
                        thrpt:  [5.4397 cpb 5.4395 cpb 5.4393 cpb]
                 change:
                        time:   [-76.958% -76.945% -76.933%] (p = 0.00 < 0.05)
                        thrpt:  [+333.52% +333.75% +333.99%]
                        Performance has improved.
Found 18 outliers among 100 measurements (18.00%)
  4 (4.00%) low severe
  2 (2.00%) low mild
  3 (3.00%) high mild
  9 (9.00%) high severe
Benchmarking ChaCha20Rng/fill_bytes/4096: Collecting 100 samples in estimated 5.ChaCha20Rng/fill_bytes/4096
                        time:   [22243.4196 cycles 22244.8522 cycles 22246.4737 cycles]
                        thrpt:  [5.4313 cpb 5.4309 cpb 5.4305 cpb]
                 change:
                        time:   [-76.956% -76.925% -76.897%] (p = 0.00 < 0.05)
                        thrpt:  [+332.84% +333.38% +333.96%]
                        Performance has improved.
Found 16 outliers among 100 measurements (16.00%)
  1 (1.00%) low severe
  4 (4.00%) low mild
  5 (5.00%) high mild
  6 (6.00%) high severe
Benchmarking ChaCha20Rng/fill_bytes/8192: Collecting 100 samples in estimated 5.ChaCha20Rng/fill_bytes/8192
                        time:   [44447.8597 cycles 44449.1595 cycles 44450.7449 cycles]
                        thrpt:  [5.4261 cpb 5.4259 cpb 5.4258 cpb]
                 change:
                        time:   [-76.933% -76.907% -76.882%] (p = 0.00 < 0.05)
                        thrpt:  [+332.56% +333.02% +333.52%]
                        Performance has improved.
Found 20 outliers among 100 measurements (20.00%)
  4 (4.00%) low severe
  2 (2.00%) low mild
  6 (6.00%) high mild
  8 (8.00%) high severe
Benchmarking ChaCha20Rng/fill_bytes/16384: Collecting 100 samples in estimated 5ChaCha20Rng/fill_bytes/16384
                        time:   [88896.6509 cycles 88915.0498 cycles 88947.2086 cycles]
                        thrpt:  [5.4289 cpb 5.4269 cpb 5.4258 cpb]
                 change:
                        time:   [-76.975% -76.942% -76.911%] (p = 0.00 < 0.05)
                        thrpt:  [+333.11% +333.68% +334.31%]
                        Performance has improved.
Found 17 outliers among 100 measurements (17.00%)
  4 (4.00%) low severe
  3 (3.00%) high mild
  10 (10.00%) high severe

```