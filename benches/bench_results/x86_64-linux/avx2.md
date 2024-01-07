# Benchmarks for chacha20 using AVX2 and zeroize for the backends
These benches were ran on 1/7/2024 using a 3.0 GHz i9 CPU after impling `ZeroizeOnDrop` for the backends.

## ChaCha20::apply_keystream()
```
     Running src/chacha20.rs (target/release/deps/chacha20-ec6cae07021ae448)
Gnuplot not found, using plotters backend
stream-cipher/apply_keystream/1024
                        time:   [1003.8263 cycles 1007.2224 cycles 1011.2429 cycles]
                        thrpt:  [0.9875 cpb 0.9836 cpb 0.9803 cpb]
Found 11 outliers among 100 measurements (11.00%)
  4 (4.00%) high mild
  7 (7.00%) high severe
stream-cipher/apply_keystream/2048
                        time:   [1956.7275 cycles 1965.4492 cycles 1974.9223 cycles]
                        thrpt:  [0.9643 cpb 0.9597 cpb 0.9554 cpb]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
stream-cipher/apply_keystream/4096
                        time:   [3886.0297 cycles 3916.6986 cycles 3953.0356 cycles]
                        thrpt:  [0.9651 cpb 0.9562 cpb 0.9487 cpb]
Found 12 outliers among 100 measurements (12.00%)
  6 (6.00%) high mild
  6 (6.00%) high severe
stream-cipher/apply_keystream/8192
                        time:   [7770.2799 cycles 7818.4325 cycles 7871.9168 cycles]
                        thrpt:  [0.9609 cpb 0.9544 cpb 0.9485 cpb]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
stream-cipher/apply_keystream/16384
                        time:   [15441.9135 cycles 15516.7315 cycles 15597.9709 cycles]
                        thrpt:  [0.9520 cpb 0.9471 cpb 0.9425 cpb]
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe
```
## ChaCha20Rng::fill_bytes()
```
ChaCha20Rng/fill_bytes/1024
                        time:   [998.5967 cycles 1006.0320 cycles 1014.9664 cycles]
                        thrpt:  [0.9912 cpb 0.9825 cpb 0.9752 cpb]
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe
ChaCha20Rng/fill_bytes/2048
                        time:   [1958.1513 cycles 1972.7250 cycles 1989.6129 cycles]
                        thrpt:  [0.9715 cpb 0.9632 cpb 0.9561 cpb]
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe
ChaCha20Rng/fill_bytes/4096
                        time:   [3883.1675 cycles 3913.3108 cycles 3948.7919 cycles]
                        thrpt:  [0.9641 cpb 0.9554 cpb 0.9480 cpb]
Found 10 outliers among 100 measurements (10.00%)
  9 (9.00%) high mild
  1 (1.00%) high severe
ChaCha20Rng/fill_bytes/8192
                        time:   [7709.5623 cycles 7748.7830 cycles 7797.4447 cycles]
                        thrpt:  [0.9518 cpb 0.9459 cpb 0.9411 cpb]
Found 10 outliers among 100 measurements (10.00%)
  4 (4.00%) high mild
  6 (6.00%) high severe
ChaCha20Rng/fill_bytes/16384
                        time:   [15580.2788 cycles 15727.5348 cycles 15911.8900 cycles]
                        thrpt:  [0.9712 cpb 0.9599 cpb 0.9509 cpb]
Found 12 outliers among 100 measurements (12.00%)
  6 (6.00%) high mild
  6 (6.00%) high severe
```