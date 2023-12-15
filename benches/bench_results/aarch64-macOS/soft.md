# Benchmarks for chacha20 using the Soft implementation
These benches were ran on 12/15/2023 using an M1.

## ChaCha20::apply_keystream()
```
Running src/chacha20.rs (target/release/deps/chacha20-1ed7885f6d535b43)
Gnuplot not found, using plotters backend
stream-cipher/apply_keystream/1024
                        time:   [3.1058 µs 3.1071 µs 3.1087 µs]
                        thrpt:  [314.14 MiB/s 314.30 MiB/s 314.44 MiB/s]
Found 11 outliers among 100 measurements (11.00%)
  2 (2.00%) high mild
  9 (9.00%) high severe
stream-cipher/apply_keystream/2048
                        time:   [6.2033 µs 6.2049 µs 6.2067 µs]
                        thrpt:  [314.68 MiB/s 314.77 MiB/s 314.85 MiB/s]
Found 10 outliers among 100 measurements (10.00%)
  4 (4.00%) high mild
  6 (6.00%) high severe
stream-cipher/apply_keystream/4096
                        time:   [12.401 µs 12.409 µs 12.419 µs]
                        thrpt:  [314.54 MiB/s 314.80 MiB/s 314.99 MiB/s]
Found 13 outliers among 100 measurements (13.00%)
  7 (7.00%) high mild
  6 (6.00%) high severe
stream-cipher/apply_keystream/8192
                        time:   [24.793 µs 24.803 µs 24.816 µs]
                        thrpt:  [314.81 MiB/s 314.98 MiB/s 315.12 MiB/s]
Found 13 outliers among 100 measurements (13.00%)
  5 (5.00%) high mild
  8 (8.00%) high severe
stream-cipher/apply_keystream/16384
                        time:   [49.598 µs 49.629 µs 49.666 µs]
                        thrpt:  [314.60 MiB/s 314.84 MiB/s 315.03 MiB/s]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe
```

## ChaCha20Rng::fill_bytes()
```
ChaCha20Rng/fill_bytes/1024
                        time:   [3.1149 µs 3.1185 µs 3.1228 µs]
                        thrpt:  [312.72 MiB/s 313.15 MiB/s 313.51 MiB/s]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe
ChaCha20Rng/fill_bytes/2048
                        time:   [6.2150 µs 6.2215 µs 6.2333 µs]
                        thrpt:  [313.34 MiB/s 313.93 MiB/s 314.26 MiB/s]
Found 13 outliers among 100 measurements (13.00%)
  1 (1.00%) high mild
  12 (12.00%) high severe
ChaCha20Rng/fill_bytes/4096
                        time:   [12.423 µs 12.427 µs 12.431 µs]
                        thrpt:  [314.23 MiB/s 314.34 MiB/s 314.43 MiB/s]
Found 11 outliers among 100 measurements (11.00%)
  2 (2.00%) high mild
  9 (9.00%) high severe
ChaCha20Rng/fill_bytes/8192
                        time:   [24.850 µs 24.884 µs 24.947 µs]
                        thrpt:  [313.17 MiB/s 313.96 MiB/s 314.38 MiB/s]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
ChaCha20Rng/fill_bytes/16384
                        time:   [49.727 µs 49.779 µs 49.846 µs]
                        thrpt:  [313.47 MiB/s 313.88 MiB/s 314.21 MiB/s]
Found 15 outliers among 100 measurements (15.00%)
  4 (4.00%) high mild
  11 (11.00%) high severe
```