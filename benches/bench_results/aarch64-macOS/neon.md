# Benchmarks for chacha20 using NEON
These benches were ran on 12/15/2023 using an M1.

## ChaCha20::apply_keystream()
```
     Running src/chacha20.rs (target/release/deps/chacha20-1ed7885f6d535b43)
Gnuplot not found, using plotters backend
Benchmarking stream-cipher/apply_keystream/1024: Collecting 100 samples in estimated 5.0006 
stream-cipher/apply_keystream/1024
                        time:   [786.17 ns 786.48 ns 787.01 ns]
                        thrpt:  [1.2118 GiB/s 1.2126 GiB/s 1.2131 GiB/s]
                 change:
                        time:   [-3.7089% -3.4701% -3.2665%] (p = 0.00 < 0.05)
                        thrpt:  [+3.3768% +3.5949% +3.8518%]
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) high mild
  8 (8.00%) high severe
stream-cipher/apply_keystream/2048
                        time:   [1.5710 µs 1.5719 µs 1.5730 µs]
                        thrpt:  [1.2125 GiB/s 1.2134 GiB/s 1.2141 GiB/s]
                 change:
                        time:   [-3.1039% -2.9099% -2.7230%] (p = 0.00 < 0.05)
                        thrpt:  [+2.7992% +2.9971% +3.2033%]
                        Performance has improved.
Found 16 outliers among 100 measurements (16.00%)
  5 (5.00%) high mild
  11 (11.00%) high severe
stream-cipher/apply_keystream/4096
                        time:   [3.1419 µs 3.1433 µs 3.1451 µs]
                        thrpt:  [1.2129 GiB/s 1.2136 GiB/s 1.2141 GiB/s]
                 change:
                        time:   [-2.8459% -2.7099% -2.5821%] (p = 0.00 < 0.05)
                        thrpt:  [+2.6505% +2.7853% +2.9293%]
                        Performance has improved.
Found 15 outliers among 100 measurements (15.00%)
  4 (4.00%) high mild
  11 (11.00%) high severe
stream-cipher/apply_keystream/8192
                        time:   [6.2826 µs 6.2881 µs 6.2950 µs]
                        thrpt:  [1.2120 GiB/s 1.2133 GiB/s 1.2144 GiB/s]
                 change:
                        time:   [-2.6497% -2.5544% -2.4567%] (p = 0.00 < 0.05)
                        thrpt:  [+2.5186% +2.6213% +2.7218%]
                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  2 (2.00%) high mild
  11 (11.00%) high severe
stream-cipher/apply_keystream/16384
                        time:   [12.556 µs 12.563 µs 12.572 µs]
                        thrpt:  [1.2137 GiB/s 1.2146 GiB/s 1.2152 GiB/s]
                 change:
                        time:   [-2.8218% -2.7078% -2.5977%] (p = 0.00 < 0.05)
                        thrpt:  [+2.6670% +2.7831% +2.9037%]
                        Performance has improved.
Found 15 outliers among 100 measurements (15.00%)
  4 (4.00%) high mild
  11 (11.00%) high severe
```

## ChaCha20Rng::fill_bytes()
```
ChaCha20Rng/fill_bytes/1024
                        time:   [783.36 ns 783.67 ns 784.04 ns]
                        thrpt:  [1.2164 GiB/s 1.2169 GiB/s 1.2174 GiB/s]
                 change:
                        time:   [-2.6815% -2.5852% -2.4804%] (p = 0.00 < 0.05)
                        thrpt:  [+2.5435% +2.6538% +2.7554%]
                        Performance has improved.
Found 16 outliers among 100 measurements (16.00%)
  6 (6.00%) high mild
  10 (10.00%) high severe
ChaCha20Rng/fill_bytes/2048
                        time:   [1.5665 µs 1.5672 µs 1.5681 µs]
                        thrpt:  [1.2164 GiB/s 1.2170 GiB/s 1.2176 GiB/s]
                 change:
                        time:   [-3.2648% -2.9041% -2.6377%] (p = 0.00 < 0.05)
                        thrpt:  [+2.7092% +2.9910% +3.3750%]
                        Performance has improved.
Found 15 outliers among 100 measurements (15.00%)
  5 (5.00%) high mild
  10 (10.00%) high severe
ChaCha20Rng/fill_bytes/4096
                        time:   [3.1284 µs 3.1298 µs 3.1318 µs]
                        thrpt:  [1.2180 GiB/s 1.2188 GiB/s 1.2194 GiB/s]
                 change:
                        time:   [-2.7688% -2.6660% -2.5627%] (p = 0.00 < 0.05)
                        thrpt:  [+2.6300% +2.7390% +2.8476%]
                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  5 (5.00%) high mild
  7 (7.00%) high severe
ChaCha20Rng/fill_bytes/8192
                        time:   [6.2569 µs 6.2600 µs 6.2643 µs]
                        thrpt:  [1.2179 GiB/s 1.2187 GiB/s 1.2194 GiB/s]
                 change:
                        time:   [-2.8217% -2.6746% -2.5392%] (p = 0.00 < 0.05)
                        thrpt:  [+2.6054% +2.7481% +2.9036%]
                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  2 (2.00%) high mild
  10 (10.00%) high severe
ChaCha20Rng/fill_bytes/16384
                        time:   [12.512 µs 12.516 µs 12.520 µs]
                        thrpt:  [1.2188 GiB/s 1.2192 GiB/s 1.2195 GiB/s]
                 change:
                        time:   [-3.0246% -2.8426% -2.6691%] (p = 0.00 < 0.05)
                        thrpt:  [+2.7423% +2.9257% +3.1190%]
                        Performance has improved.
Found 16 outliers among 100 measurements (16.00%)
  3 (3.00%) high mild
  13 (13.00%) high severe
```