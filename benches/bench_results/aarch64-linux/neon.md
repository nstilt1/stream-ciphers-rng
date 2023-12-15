# Benchmarks for chacha20 using the Soft implementation
These benches were ran on 12/14/2023 using a Raspberry Pi 4.

## ChaCha20::apply_keystream()
     Running src/chacha20.rs (target/release/deps/chacha20-02f555ae0af3670b)
Gnuplot not found, using plotters backend
Benchmarking stream-cipher/apply_keystream/1024: Collecting 100 samples in estimstream-cipher/apply_keystream/1024
                        time:   [5662.5123 cycles 5664.6409 cycles 5666.9057 cycles]
                        thrpt:  [5.5341 cpb 5.5319 cpb 5.5298 cpb]
                 change:
                        time:   [-76.818% -76.809% -76.800%] (p = 0.00 < 0.05)
                        thrpt:  [+331.03% +331.20% +331.38%]
                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
Benchmarking stream-cipher/apply_keystream/2048: Collecting 100 samples in estimstream-cipher/apply_keystream/2048
                        time:   [11235.1813 cycles 11235.9136 cycles 11236.8848 cycles]
                        thrpt:  [5.4868 cpb 5.4863 cpb 5.4859 cpb]
                 change:
                        time:   [-76.987% -76.983% -76.979%] (p = 0.00 < 0.05)
                        thrpt:  [+334.39% +334.46% +334.54%]
                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  2 (2.00%) low severe
  2 (2.00%) low mild
  2 (2.00%) high mild
  7 (7.00%) high severe
Benchmarking stream-cipher/apply_keystream/4096: Collecting 100 samples in estimstream-cipher/apply_keystream/4096
                        time:   [22400.7526 cycles 22401.7166 cycles 22402.8724 cycles]
                        thrpt:  [5.4695 cpb 5.4692 cpb 5.4689 cpb]
                 change:
                        time:   [-77.071% -77.055% -77.043%] (p = 0.00 < 0.05)
                        thrpt:  [+335.60% +335.82% +336.13%]
                        Performance has improved.
Found 17 outliers among 100 measurements (17.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  5 (5.00%) high mild
  9 (9.00%) high severe
Benchmarking stream-cipher/apply_keystream/8192: Collecting 100 samples in estimstream-cipher/apply_keystream/8192
                        time:   [44733.9988 cycles 44735.5014 cycles 44737.2729 cycles]
                        thrpt:  [5.4611 cpb 5.4609 cpb 5.4607 cpb]
                 change:
                        time:   [-77.102% -77.085% -77.073%] (p = 0.00 < 0.05)
                        thrpt:  [+336.17% +336.40% +336.72%]
                        Performance has improved.
Found 17 outliers among 100 measurements (17.00%)
  2 (2.00%) low severe
  2 (2.00%) low mild
  2 (2.00%) high mild
  11 (11.00%) high severe
Benchmarking stream-cipher/apply_keystream/16384: Collecting 100 samples in estistream-cipher/apply_keystream/16384
                        time:   [90580.1284 cycles 90583.4349 cycles 90587.2335 cycles]
                        thrpt:  [5.5290 cpb 5.5288 cpb 5.5286 cpb]
                 change:
                        time:   [-76.789% -76.787% -76.784%] (p = 0.00 < 0.05)
                        thrpt:  [+330.74% +330.79% +330.83%]
                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  4 (4.00%) high mild
  6 (6.00%) high severe

## ChaCha20Rng::fill_bytes()
Benchmarking ChaCha20Rng/fill_bytes/1024: Collecting 100 samples in estimated 5.ChaCha20Rng/fill_bytes/1024
                        time:   [5572.6311 cycles 5573.1775 cycles 5573.7676 cycles]
                        thrpt:  [5.4431 cpb 5.4426 cpb 5.4420 cpb]
                 change:
                        time:   [-76.985% -76.957% -76.927%] (p = 0.00 < 0.05)
                        thrpt:  [+333.40% +333.97% +334.50%]
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
Benchmarking ChaCha20Rng/fill_bytes/2048: Collecting 100 samples in estimated 5.ChaCha20Rng/fill_bytes/2048
                        time:   [11121.2086 cycles 11121.4617 cycles 11121.7880 cycles]
                        thrpt:  [5.4306 cpb 5.4304 cpb 5.4303 cpb]
                 change:
                        time:   [-76.775% -76.737% -76.699%] (p = 0.00 < 0.05)
                        thrpt:  [+329.16% +329.86% +330.57%]
                        Performance has improved.
Found 14 outliers among 100 measurements (14.00%)
  2 (2.00%) low severe
  3 (3.00%) low mild
  5 (5.00%) high mild
  4 (4.00%) high severe
Benchmarking ChaCha20Rng/fill_bytes/4096: Collecting 100 samples in estimated 5.ChaCha20Rng/fill_bytes/4096
                        time:   [22210.6070 cycles 22211.3258 cycles 22212.1931 cycles]
                        thrpt:  [5.4229 cpb 5.4227 cpb 5.4225 cpb]
                 change:
                        time:   [-77.090% -77.073% -77.056%] (p = 0.00 < 0.05)
                        thrpt:  [+335.84% +336.17% +336.50%]
                        Performance has improved.
Found 14 outliers among 100 measurements (14.00%)
  2 (2.00%) low severe
  2 (2.00%) low mild
  4 (4.00%) high mild
  6 (6.00%) high severe
Benchmarking ChaCha20Rng/fill_bytes/8192: Collecting 100 samples in estimated 5.ChaCha20Rng/fill_bytes/8192
                        time:   [44398.0573 cycles 44399.0441 cycles 44400.3312 cycles]
                        thrpt:  [5.4200 cpb 5.4198 cpb 5.4197 cpb]
                 change:
                        time:   [-77.089% -77.071% -77.051%] (p = 0.00 < 0.05)
                        thrpt:  [+335.75% +336.12% +336.48%]
                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  2 (2.00%) low severe
  1 (1.00%) low mild
  1 (1.00%) high mild
  7 (7.00%) high severe
Benchmarking ChaCha20Rng/fill_bytes/16384: Collecting 100 samples in estimated 5ChaCha20Rng/fill_bytes/16384
                        time:   [88765.3666 cycles 88767.7005 cycles 88770.7646 cycles]
                        thrpt:  [5.4181 cpb 5.4180 cpb 5.4178 cpb]
                 change:
                        time:   [-76.874% -76.845% -76.816%] (p = 0.00 < 0.05)
                        thrpt:  [+331.33% +331.87% +332.42%]
                        Performance has improved.
Found 16 outliers among 100 measurements (16.00%)
  3 (3.00%) low severe
  1 (1.00%) low mild
  3 (3.00%) high mild
  9 (9.00%) high severe
