# Benchmarks for chacha20 using the Soft implementation
These benches were ran on 12/16/2023 using a Raspberry Pi 4b.
Last commit: 378303a

## ChaCha20::apply_keystream()
```
     Running src/chacha20.rs (target/release/deps/chacha20-066064f66a8e404e)
Gnuplot not found, using plotters backend
Benchmarking stream-cipher/apply_keystream/1024: Collecting 100 samples in estimstream-cipher/apply_keystream/1024
                        time:   [24425.7004 cycles 24429.1225 cycles 24431.9433 cycles]
                        thrpt:  [23.8593 cpb 23.8566 cpb 23.8532 cpb]
Found 26 outliers among 100 measurements (26.00%)
  12 (12.00%) low severe
  1 (1.00%) low mild
  5 (5.00%) high mild
  8 (8.00%) high severe
Benchmarking stream-cipher/apply_keystream/2048: Collecting 100 samples in estimstream-cipher/apply_keystream/2048
                        time:   [48804.3229 cycles 48831.8099 cycles 48870.1969 cycles]
                        thrpt:  [23.8624 cpb 23.8437 cpb 23.8302 cpb]
Found 26 outliers among 100 measurements (26.00%)
  10 (10.00%) low severe
  1 (1.00%) low mild
  6 (6.00%) high mild
  9 (9.00%) high severe
Benchmarking stream-cipher/apply_keystream/4096: Collecting 100 samples in estimstream-cipher/apply_keystream/4096
                        time:   [97565.5832 cycles 97572.0344 cycles 97576.9663 cycles]
                        thrpt:  [23.8225 cpb 23.8213 cpb 23.8197 cpb]
Found 16 outliers among 100 measurements (16.00%)
  5 (5.00%) low severe
  4 (4.00%) low mild
  3 (3.00%) high mild
  4 (4.00%) high severe
Benchmarking stream-cipher/apply_keystream/8192: Collecting 100 samples in estimstream-cipher/apply_keystream/8192
                        time:   [195101.0057 cycles 195108.0196 cycles 195115.8927 cycles]
                        thrpt:  [23.8179 cpb 23.8169 cpb 23.8160 cpb]
Found 15 outliers among 100 measurements (15.00%)
  6 (6.00%) low severe
  1 (1.00%) low mild
  5 (5.00%) high mild
  3 (3.00%) high severe
Benchmarking stream-cipher/apply_keystream/16384: Collecting 100 samples in estistream-cipher/apply_keystream/16384
                        time:   [390687.2144 cycles 390803.8072 cycles 390915.2555 cycles]
                        thrpt:  [23.8596 cpb 23.8528 cpb 23.8457 cpb]
Found 19 outliers among 100 measurements (19.00%)
  3 (3.00%) low severe
  3 (3.00%) high mild
  13 (13.00%) high severe
```
## ChaCha20Rng::fill_bytes()
```
Benchmarking ChaCha20Rng/fill_bytes/1024: Collecting 100 samples in estimated 5.ChaCha20Rng/fill_bytes/1024
                        time:   [24155.5933 cycles 24173.8239 cycles 24193.0800 cycles]
                        thrpt:  [23.6261 cpb 23.6072 cpb 23.5894 cpb]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe
Benchmarking ChaCha20Rng/fill_bytes/2048: Collecting 100 samples in estimated 5.ChaCha20Rng/fill_bytes/2048
                        time:   [48303.2857 cycles 48342.2892 cycles 48383.2136 cycles]
                        thrpt:  [23.6246 cpb 23.6046 cpb 23.5856 cpb]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
Benchmarking ChaCha20Rng/fill_bytes/4096: Collecting 100 samples in estimated 5.ChaCha20Rng/fill_bytes/4096
                        time:   [96243.7629 cycles 96374.5050 cycles 96522.4911 cycles]
                        thrpt:  [23.5651 cpb 23.5289 cpb 23.4970 cpb]
Found 8 outliers among 100 measurements (8.00%)
  8 (8.00%) high mild
Benchmarking ChaCha20Rng/fill_bytes/8192: Collecting 100 samples in estimated 5.ChaCha20Rng/fill_bytes/8192
                        time:   [192194.9883 cycles 192394.0895 cycles 192614.8173 cycles]
                        thrpt:  [23.5126 cpb 23.4856 cpb 23.4613 cpb]
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high mild
Benchmarking ChaCha20Rng/fill_bytes/16384: Collecting 100 samples in estimated 5ChaCha20Rng/fill_bytes/16384
                        time:   [384774.9726 cycles 385183.5637 cycles 385610.0867 cycles]
                        thrpt:  [23.5358 cpb 23.5097 cpb 23.4848 cpb]
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high mild
```