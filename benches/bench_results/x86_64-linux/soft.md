# Benchmarks for chacha20 using Soft implementation
These benches were ran on 12/16/2023 using a 3.0 GHz i9 CPU.
This bench is compared against the last one, commit: 378303a
This commit: 97cdfc5

## ChaCha20::apply_keystream()
```
     Running src/chacha20.rs (target/release/deps/chacha20-5e4721a497c9e862)
Gnuplot not found, using plotters backend
stream-cipher/apply_keystream/1024
                        time:   [5245.1941 cycles 5258.4651 cycles 5273.2768 cycles]
                        thrpt:  [5.1497 cpb 5.1352 cpb 5.1223 cpb]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
stream-cipher/apply_keystream/2048
                        time:   [10452.2060 cycles 10478.9397 cycles 10507.7643 cycles]
                        thrpt:  [5.1307 cpb 5.1167 cpb 5.1036 cpb]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
stream-cipher/apply_keystream/4096
                        time:   [21011.2948 cycles 21085.2075 cycles 21161.5458 cycles]
                        thrpt:  [5.1664 cpb 5.1478 cpb 5.1297 cpb]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
stream-cipher/apply_keystream/8192
                        time:   [41811.4815 cycles 41929.4400 cycles 42061.1797 cycles]
                        thrpt:  [5.1344 cpb 5.1183 cpb 5.1039 cpb]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe
stream-cipher/apply_keystream/16384
                        time:   [83798.9644 cycles 84066.1982 cycles 84378.8143 cycles]
                        thrpt:  [5.1501 cpb 5.1310 cpb 5.1147 cpb]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe
```
## ChaCha20Rng::fill_bytes()
```
ChaCha20Rng/fill_bytes/1024
                        time:   [4776.0768 cycles 4782.4926 cycles 4789.9293 cycles]
                        thrpt:  [4.6777 cpb 4.6704 cpb 4.6641 cpb]
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  2 (2.00%) high severe
ChaCha20Rng/fill_bytes/2048
                        time:   [9558.4517 cycles 9580.1423 cycles 9605.0202 cycles]
                        thrpt:  [4.6900 cpb 4.6778 cpb 4.6672 cpb]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe
ChaCha20Rng/fill_bytes/4096
                        time:   [19130.0705 cycles 19161.2837 cycles 19194.4355 cycles]
                        thrpt:  [4.6861 cpb 4.6780 cpb 4.6704 cpb]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
ChaCha20Rng/fill_bytes/8192
                        time:   [38283.9441 cycles 38339.5541 cycles 38396.3487 cycles]
                        thrpt:  [4.6871 cpb 4.6801 cpb 4.6733 cpb]
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe
ChaCha20Rng/fill_bytes/16384
                        time:   [76550.5142 cycles 76703.3393 cycles 76871.7348 cycles]
                        thrpt:  [4.6919 cpb 4.6816 cpb 4.6723 cpb]
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe
```