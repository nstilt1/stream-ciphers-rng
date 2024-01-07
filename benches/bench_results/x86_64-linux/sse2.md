# Benchmarks for chacha20 using SSE2 and zeroize for the backends
These benches were ran on 1/7/2024 using a 3.0 GHz i9 CPU after impling `ZeroizeOnDrop` for the backends.

## ChaCha20::apply_keystream()
```
     Running src/chacha20.rs (target/release/deps/chacha20-ec6cae07021ae448)
Gnuplot not found, using plotters backend
stream-cipher/apply_keystream/1024
                        time:   [3072.7467 cycles 3094.6683 cycles 3118.9929 cycles]
                        thrpt:  [3.0459 cpb 3.0221 cpb 3.0007 cpb]
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe
stream-cipher/apply_keystream/2048
                        time:   [6174.7243 cycles 6218.8904 cycles 6269.6278 cycles]
                        thrpt:  [3.0613 cpb 3.0366 cpb 3.0150 cpb]
Found 8 outliers among 100 measurements (8.00%)
  6 (6.00%) high mild
  2 (2.00%) high severe
stream-cipher/apply_keystream/4096
                        time:   [12230.5386 cycles 12327.8379 cycles 12438.0421 cycles]
                        thrpt:  [3.0366 cpb 3.0097 cpb 2.9860 cpb]
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe
stream-cipher/apply_keystream/8192
                        time:   [24640.3839 cycles 24947.0462 cycles 25292.3913 cycles]
                        thrpt:  [3.0875 cpb 3.0453 cpb 3.0079 cpb]
Found 16 outliers among 100 measurements (16.00%)
  10 (10.00%) high mild
  6 (6.00%) high severe
stream-cipher/apply_keystream/16384
                        time:   [49208.2700 cycles 49602.3495 cycles 50040.8766 cycles]
                        thrpt:  [3.0543 cpb 3.0275 cpb 3.0034 cpb]
Found 10 outliers among 100 measurements (10.00%)
  5 (5.00%) high mild
  5 (5.00%) high severe
```

## ChaCha20Rng::fill_bytes()
```
ChaCha20Rng/fill_bytes/1024
                        time:   [3029.6889 cycles 3046.5375 cycles 3066.9859 cycles]
                        thrpt:  [2.9951 cpb 2.9751 cpb 2.9587 cpb]
Found 11 outliers among 100 measurements (11.00%)
  4 (4.00%) high mild
  7 (7.00%) high severe
ChaCha20Rng/fill_bytes/2048
                        time:   [6050.3871 cycles 6095.8893 cycles 6158.0522 cycles]
                        thrpt:  [3.0069 cpb 2.9765 cpb 2.9543 cpb]
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) high mild
  6 (6.00%) high severe
ChaCha20Rng/fill_bytes/4096
                        time:   [12080.4759 cycles 12200.3032 cycles 12348.2699 cycles]
                        thrpt:  [3.0147 cpb 2.9786 cpb 2.9493 cpb]
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe
ChaCha20Rng/fill_bytes/8192
                        time:   [23858.1998 cycles 23956.5697 cycles 24074.0458 cycles]
                        thrpt:  [2.9387 cpb 2.9244 cpb 2.9124 cpb]
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe
ChaCha20Rng/fill_bytes/16384
                        time:   [47612.4582 cycles 47815.9432 cycles 48063.1688 cycles]
                        thrpt:  [2.9335 cpb 2.9185 cpb 2.9060 cpb]
Found 15 outliers among 100 measurements (15.00%)
  7 (7.00%) high mild
  8 (8.00%) high severe
```