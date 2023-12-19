# Benchmarks for chacha20 using the Soft implementation
These benches were ran on 12/16/2023 using a Raspberry Pi 4b.
This bench is compared against the last one, commit: 378303a
This commit: 97cdfc5

## ChaCha20::apply_keystream()
```
     Running src/chacha20.rs (target/release/deps/chacha20-066064f66a8e404e)
Gnuplot not found, using plotters backend
stream-cipher/apply_keystream/1024
                        time:   [24374.6861 cycles 24375.3062 cycles 24375.9936 cycles]
                        thrpt:  [23.8047 cpb 23.8040 cpb 23.8034 cpb]
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  1 (1.00%) high mild
  6 (6.00%) high severe
stream-cipher/apply_keystream/2048
                        time:   [48723.3096 cycles 48726.0911 cycles 48729.5279 cycles]
                        thrpt:  [23.7937 cpb 23.7920 cpb 23.7907 cpb]
Found 14 outliers among 100 measurements (14.00%)
  3 (3.00%) low severe
  1 (1.00%) low mild
  4 (4.00%) high mild
  6 (6.00%) high severe
stream-cipher/apply_keystream/4096
                        time:   [97427.1798 cycles 97484.7685 cycles 97598.3805 cycles]
                        thrpt:  [23.8277 cpb 23.8000 cpb 23.7859 cpb]
Found 14 outliers among 100 measurements (14.00%)
  2 (2.00%) low severe
  1 (1.00%) low mild
  5 (5.00%) high mild
  6 (6.00%) high severe
stream-cipher/apply_keystream/8192
                        time:   [194855.0591 cycles 194888.9259 cycles 194934.3983 cycles]
                        thrpt:  [23.7957 cpb 23.7902 cpb 23.7860 cpb]
Found 20 outliers among 100 measurements (20.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  5 (5.00%) high mild
  12 (12.00%) high severe
stream-cipher/apply_keystream/16384
                        time:   [390255.4021 cycles 390273.8740 cycles 390295.1114 cycles]
                        thrpt:  [23.8217 cpb 23.8204 cpb 23.8193 cpb]
Found 11 outliers among 100 measurements (11.00%)
  2 (2.00%) low mild
  6 (6.00%) high mild
  3 (3.00%) high severe
```
## ChaCha20Rng::fill_bytes()
```
ChaCha20Rng/fill_bytes/1024
                        time:   [24067.7086 cycles 24071.4233 cycles 24074.9280 cycles]
                        thrpt:  [23.5107 cpb 23.5072 cpb 23.5036 cpb]
Found 17 outliers among 100 measurements (17.00%)
  4 (4.00%) low severe
  3 (3.00%) low mild
  4 (4.00%) high mild
  6 (6.00%) high severe
ChaCha20Rng/fill_bytes/2048
                        time:   [48235.2914 cycles 48287.1989 cycles 48358.5222 cycles]
                        thrpt:  [23.6126 cpb 23.5777 cpb 23.5524 cpb]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
ChaCha20Rng/fill_bytes/4096
                        time:   [96211.3241 cycles 96281.8343 cycles 96426.7674 cycles]
                        thrpt:  [23.5417 cpb 23.5063 cpb 23.4891 cpb]
Found 17 outliers among 100 measurements (17.00%)
  3 (3.00%) low severe
  4 (4.00%) low mild
  3 (3.00%) high mild
  7 (7.00%) high severe
ChaCha20Rng/fill_bytes/8192
                        time:   [193525.6511 cycles 193740.0708 cycles 193926.6228 cycles]
                        thrpt:  [23.6727 cpb 23.6499 cpb 23.6237 cpb]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low severe
  6 (6.00%) low mild
  1 (1.00%) high severe
ChaCha20Rng/fill_bytes/16384
                        time:   [386105.7670 cycles 386639.8637 cycles 387114.8610 cycles]
                        thrpt:  [23.6276 cpb 23.5986 cpb 23.5660 cpb]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) low mild

```