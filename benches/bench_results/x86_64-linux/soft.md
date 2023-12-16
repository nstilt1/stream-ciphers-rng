# Benchmarks for chacha20 using Soft implementation
These benches were ran on 12/16/2023 using a 3.0 GHz i9 CPU, immediately after a restart. Prior to the restart, it was slightly slower.
Last commit: 378303a

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
                        time:   [4915.1026 cycles 4929.8101 cycles 4944.1027 cycles]
                        thrpt:  [4.8282 cpb 4.8143 cpb 4.7999 cpb]
ChaCha20Rng/fill_bytes/2048
                        time:   [9720.4919 cycles 9744.1491 cycles 9770.4277 cycles]
                        thrpt:  [4.7707 cpb 4.7579 cpb 4.7463 cpb]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
ChaCha20Rng/fill_bytes/4096
                        time:   [19401.4572 cycles 19447.2252 cycles 19504.3730 cycles]
                        thrpt:  [4.7618 cpb 4.7479 cpb 4.7367 cpb]
Found 8 outliers among 100 measurements (8.00%)
  6 (6.00%) high mild
  2 (2.00%) high severe
ChaCha20Rng/fill_bytes/8192
                        time:   [38750.0171 cycles 38822.3414 cycles 38899.3476 cycles]
                        thrpt:  [4.7485 cpb 4.7391 cpb 4.7302 cpb]
Found 8 outliers among 100 measurements (8.00%)
  8 (8.00%) high mild
ChaCha20Rng/fill_bytes/16384
                        time:   [77530.9385 cycles 77699.9591 cycles 77888.0939 cycles]
                        thrpt:  [4.7539 cpb 4.7424 cpb 4.7321 cpb]
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe
```