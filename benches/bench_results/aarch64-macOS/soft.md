# Benchmarks for chacha20 using the Soft implementation
These benches were ran on 12/15/2023 using an M1.

## ChaCha20::apply_keystream()
```
Running src/chacha20.rs (target/release/deps/chacha20-1ed7885f6d535b43)
Gnuplot not found, using plotters backend
Benchmarking stream-cipher/apply_keystream/1024: Collecting 100 samples in estimated 5.0056 
stream-cipher/apply_keystream/1024
                        time:   [3.1451 µs 3.1546 µs 3.1707 µs]
                        thrpt:  [308.00 MiB/s 309.57 MiB/s 310.51 MiB/s]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
Benchmarking stream-cipher/apply_keystream/2048: Collecting 100 samples in estimated 5.0518 
stream-cipher/apply_keystream/2048
                        time:   [6.4806 µs 6.5965 µs 6.7629 µs]
                        thrpt:  [288.80 MiB/s 296.08 MiB/s 301.38 MiB/s]
Found 12 outliers among 100 measurements (12.00%)
  12 (12.00%) high severe
Benchmarking stream-cipher/apply_keystream/4096: Collecting 100 samples in estimated 5.0490 
stream-cipher/apply_keystream/4096
                        time:   [12.534 µs 12.551 µs 12.572 µs]
                        thrpt:  [310.72 MiB/s 311.22 MiB/s 311.65 MiB/s]
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe
Benchmarking stream-cipher/apply_keystream/8192: Collecting 100 samples in estimated 5.0807 
stream-cipher/apply_keystream/8192
                        time:   [25.018 µs 25.049 µs 25.090 µs]
                        thrpt:  [311.38 MiB/s 311.89 MiB/s 312.27 MiB/s]
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe
Benchmarking stream-cipher/apply_keystream/16384: Collecting 100 samples in estimated 5.0587
stream-cipher/apply_keystream/16384
                        time:   [49.975 µs 49.994 µs 50.014 µs]
                        thrpt:  [312.41 MiB/s 312.54 MiB/s 312.66 MiB/s]
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) low mild
  3 (3.00%) high mild
  4 (4.00%) high severe
```

## ChaCha20Rng::fill_bytes()
```
Benchmarking ChaCha20Rng/fill_bytes/1024: Collecting 100 samples in estimated 5.0056 s (1.6M
ChaCha20Rng/fill_bytes/1024
                        time:   [3.1424 µs 3.1476 µs 3.1537 µs]
                        thrpt:  [309.65 MiB/s 310.25 MiB/s 310.77 MiB/s]
Found 13 outliers among 100 measurements (13.00%)
  1 (1.00%) low mild
  5 (5.00%) high mild
  7 (7.00%) high severe
Benchmarking ChaCha20Rng/fill_bytes/2048: Collecting 100 samples in estimated 5.0233 s (798k
ChaCha20Rng/fill_bytes/2048
                        time:   [6.2738 µs 6.2978 µs 6.3387 µs]
                        thrpt:  [308.13 MiB/s 310.13 MiB/s 311.31 MiB/s]
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe
Benchmarking ChaCha20Rng/fill_bytes/4096: Collecting 100 samples in estimated 5.0096 s (399k
ChaCha20Rng/fill_bytes/4096
                        time:   [12.576 µs 12.595 µs 12.614 µs]
                        thrpt:  [309.68 MiB/s 310.14 MiB/s 310.60 MiB/s]
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe
Benchmarking ChaCha20Rng/fill_bytes/8192: Collecting 100 samples in estimated 5.0888 s (202k
ChaCha20Rng/fill_bytes/8192
                        time:   [25.191 µs 25.231 µs 25.278 µs]
                        thrpt:  [309.07 MiB/s 309.64 MiB/s 310.13 MiB/s]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe
Benchmarking ChaCha20Rng/fill_bytes/16384: Collecting 100 samples in estimated 5.1692 s (101
ChaCha20Rng/fill_bytes/16384
                        time:   [50.255 µs 50.322 µs 50.402 µs]
                        thrpt:  [310.01 MiB/s 310.50 MiB/s 310.92 MiB/s]
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe
```