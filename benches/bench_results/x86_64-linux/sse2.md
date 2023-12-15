# Benchmarks for chacha20 using SSE2
These benches were ran on 12/14/2023 using a 3.0 GHz i9 CPU.

## ChaCha20::apply_keystream()
```
     Running src/chacha20.rs (target/release/deps/chacha20-bc9efd54ac287186)
Gnuplot not found, using plotters backend
stream-cipher/apply_keystream/1024
                        time:   [2999.5388 cycles 3004.0906 cycles 3009.1423 cycles]
                        thrpt:  [2.9386 cpb 2.9337 cpb 2.9292 cpb]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe
stream-cipher/apply_keystream/2048
                        time:   [6006.4750 cycles 6014.2741 cycles 6022.5129 cycles]
                        thrpt:  [2.9407 cpb 2.9367 cpb 2.9328 cpb]
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe
stream-cipher/apply_keystream/4096
                        time:   [11995.3160 cycles 12012.3856 cycles 12030.5994 cycles]
                        thrpt:  [2.9372 cpb 2.9327 cpb 2.9285 cpb]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
stream-cipher/apply_keystream/8192
                        time:   [23965.0435 cycles 24005.9395 cycles 24049.9039 cycles]
                        thrpt:  [2.9358 cpb 2.9304 cpb 2.9254 cpb]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild
stream-cipher/apply_keystream/16384
                        time:   [47834.7353 cycles 47930.1593 cycles 48044.2354 cycles]
                        thrpt:  [2.9324 cpb 2.9254 cpb 2.9196 cpb]
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe
```

## ChaCha20Rng::fill_bytes()
```
ChaCha20Rng/fill_bytes/1024
                        time:   [2963.9750 cycles 2970.9180 cycles 2979.9447 cycles]
                        thrpt:  [2.9101 cpb 2.9013 cpb 2.8945 cpb]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe
ChaCha20Rng/fill_bytes/2048
                        time:   [5932.0069 cycles 5940.0828 cycles 5948.1821 cycles]
                        thrpt:  [2.9044 cpb 2.9004 cpb 2.8965 cpb]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild
ChaCha20Rng/fill_bytes/4096
                        time:   [11878.8840 cycles 11902.2035 cycles 11926.1258 cycles]
                        thrpt:  [2.9117 cpb 2.9058 cpb 2.9001 cpb]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe
ChaCha20Rng/fill_bytes/8192
                        time:   [23717.4493 cycles 23747.6444 cycles 23779.2748 cycles]
                        thrpt:  [2.9027 cpb 2.8989 cpb 2.8952 cpb]
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild
ChaCha20Rng/fill_bytes/16384
                        time:   [47556.8685 cycles 47634.8619 cycles 47716.7970 cycles]
                        thrpt:  [2.9124 cpb 2.9074 cpb 2.9026 cpb]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe
```