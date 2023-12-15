# Benchmarks for chacha20 using NEON
These benches were ran on 12/15/2023 using an M1.

## ChaCha20::apply_keystream()
```
     Running src/chacha20.rs (target/release/deps/chacha20-1ed7885f6d535b43)
Gnuplot not found, using plotters backend
Benchmarking stream-cipher/apply_keystream/1024: Collecting 100 samples in estimated 5.0006 
stream-cipher/apply_keystream/1024
                        time:   [814.07 ns 817.28 ns 821.71 ns]
                        thrpt:  [1.1606 GiB/s 1.1669 GiB/s 1.1715 GiB/s]
                 change:
                        time:   [-74.335% -74.206% -74.101%] (p = 0.00 < 0.05)
                        thrpt:  [+286.11% +287.69% +289.63%]
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
Benchmarking stream-cipher/apply_keystream/2048: Collecting 100 samples in estimated 5.0049 
stream-cipher/apply_keystream/2048
                        time:   [1.6184 µs 1.6224 µs 1.6272 µs]
                        thrpt:  [1.1721 GiB/s 1.1756 GiB/s 1.1785 GiB/s]
                 change:
                        time:   [-76.810% -76.104% -75.469%] (p = 0.00 < 0.05)
                        thrpt:  [+307.64% +318.48% +331.21%]
                        Performance has improved.
Found 14 outliers among 100 measurements (14.00%)
  2 (2.00%) low mild
  6 (6.00%) high mild
  6 (6.00%) high severe
Benchmarking stream-cipher/apply_keystream/4096: Collecting 100 samples in estimated 5.0108 
stream-cipher/apply_keystream/4096
                        time:   [3.2277 µs 3.2299 µs 3.2330 µs]
                        thrpt:  [1.1799 GiB/s 1.1811 GiB/s 1.1819 GiB/s]
                 change:
                        time:   [-74.308% -74.255% -74.204%] (p = 0.00 < 0.05)
                        thrpt:  [+287.65% +288.42% +289.22%]
                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  1 (1.00%) low severe
  1 (1.00%) high mild
  9 (9.00%) high severe
Benchmarking stream-cipher/apply_keystream/8192: Collecting 100 samples in estimated 5.0204 
stream-cipher/apply_keystream/8192
                        time:   [6.4495 µs 6.4524 µs 6.4556 µs]
                        thrpt:  [1.1818 GiB/s 1.1824 GiB/s 1.1829 GiB/s]
                 change:
                        time:   [-74.328% -74.264% -74.213%] (p = 0.00 < 0.05)
                        thrpt:  [+287.80% +288.56% +289.53%]
                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  3 (3.00%) high mild
  3 (3.00%) high severe
Benchmarking stream-cipher/apply_keystream/16384: Collecting 100 samples in estimated 5.0165
stream-cipher/apply_keystream/16384
                        time:   [12.905 µs 12.912 µs 12.921 µs]
                        thrpt:  [1.1809 GiB/s 1.1817 GiB/s 1.1824 GiB/s]
                 change:
                        time:   [-74.208% -74.173% -74.142%] (p = 0.00 < 0.05)
                        thrpt:  [+286.72% +287.20% +287.71%]
                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  1 (1.00%) low mild
  9 (9.00%) high mild
  3 (3.00%) high severe
```

## ChaCha20Rng::fill_bytes()
```
Benchmarking ChaCha20Rng/fill_bytes/1024: Collecting 100 samples in estimated 5.0014 s (6.2M
ChaCha20Rng/fill_bytes/1024
                        time:   [804.63 ns 804.92 ns 805.23 ns]
                        thrpt:  [1.1843 GiB/s 1.1848 GiB/s 1.1852 GiB/s]
                 change:
                        time:   [-74.451% -74.409% -74.370%] (p = 0.00 < 0.05)
                        thrpt:  [+290.17% +290.76% +291.40%]
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) low mild
  3 (3.00%) high mild
  3 (3.00%) high severe
Benchmarking ChaCha20Rng/fill_bytes/2048: Collecting 100 samples in estimated 5.0079 s (3.1M
ChaCha20Rng/fill_bytes/2048
                        time:   [1.6102 µs 1.6169 µs 1.6272 µs]
                        thrpt:  [1.1722 GiB/s 1.1796 GiB/s 1.1846 GiB/s]
                 change:
                        time:   [-74.413% -74.313% -74.200%] (p = 0.00 < 0.05)
                        thrpt:  [+287.60% +289.30% +290.82%]
                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) high mild
  9 (9.00%) high severe
Benchmarking ChaCha20Rng/fill_bytes/4096: Collecting 100 samples in estimated 5.0055 s (1.6M
ChaCha20Rng/fill_bytes/4096
                        time:   [3.2143 µs 3.2159 µs 3.2179 µs]
                        thrpt:  [1.1855 GiB/s 1.1862 GiB/s 1.1868 GiB/s]
                 change:
                        time:   [-74.546% -74.473% -74.412%] (p = 0.00 < 0.05)
                        thrpt:  [+290.81% +291.74% +292.86%]
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe
Benchmarking ChaCha20Rng/fill_bytes/8192: Collecting 100 samples in estimated 5.0020 s (778k
ChaCha20Rng/fill_bytes/8192
                        time:   [6.4277 µs 6.4304 µs 6.4333 µs]
                        thrpt:  [1.1859 GiB/s 1.1865 GiB/s 1.1870 GiB/s]
                 change:
                        time:   [-74.572% -74.478% -74.409%] (p = 0.00 < 0.05)
                        thrpt:  [+290.76% +291.82% +293.27%]
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
  5 (5.00%) high severe
Benchmarking ChaCha20Rng/fill_bytes/16384: Collecting 100 samples in estimated 5.0040 s (389
ChaCha20Rng/fill_bytes/16384
                        time:   [12.889 µs 12.910 µs 12.933 µs]
                        thrpt:  [1.1798 GiB/s 1.1819 GiB/s 1.1839 GiB/s]
                 change:
                        time:   [-74.486% -74.420% -74.357%] (p = 0.00 < 0.05)
                        thrpt:  [+289.97% +290.93% +291.94%]
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe
```