# Benchmarks for chacha20 using AVX2
These benches were ran on 12/18/2023 using a 3.0 GHz i9 CPU.
This bench is compared against using the `zeroize` feature.

## ChaCha20::apply_keystream()
```
     Running src/chacha20.rs (target/release/deps/chacha20-6f837c27da8cfee7)
Gnuplot not found, using plotters backend
stream-cipher/apply_keystream/1024
                        time:   [1006.7490 cycles 1012.0615 cycles 1018.0909 cycles]
                        thrpt:  [0.9942 cpb 0.9883 cpb 0.9832 cpb]
                 change:
                        time:   [+0.2058% +0.9358% +1.6404%] (p = 0.01 < 0.05)
                        thrpt:  [-1.6140% -0.9272% -0.2054%]
                        Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe
stream-cipher/apply_keystream/2048
                        time:   [1973.1338 cycles 1985.2891 cycles 1999.4976 cycles]
                        thrpt:  [0.9763 cpb 0.9694 cpb 0.9634 cpb]
                 change:
                        time:   [-0.9097% +0.0421% +1.0061%] (p = 0.94 > 0.05)
                        thrpt:  [-0.9961% -0.0421% +0.9181%]
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe
stream-cipher/apply_keystream/4096
                        time:   [3860.3133 cycles 3877.1631 cycles 3899.0889 cycles]
                        thrpt:  [0.9519 cpb 0.9466 cpb 0.9425 cpb]
                 change:
                        time:   [-1.6581% -0.5698% +0.4898%] (p = 0.31 > 0.05)
                        thrpt:  [-0.4874% +0.5731% +1.6861%]
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe
stream-cipher/apply_keystream/8192
                        time:   [7763.3003 cycles 7834.3254 cycles 7924.1758 cycles]
                        thrpt:  [0.9673 cpb 0.9563 cpb 0.9477 cpb]
                 change:
                        time:   [-0.7545% -0.0069% +0.7385%] (p = 0.98 > 0.05)
                        thrpt:  [-0.7331% +0.0069% +0.7602%]
                        No change in performance detected.
Found 12 outliers among 100 measurements (12.00%)
  7 (7.00%) high mild
  5 (5.00%) high severe
stream-cipher/apply_keystream/16384
                        time:   [15606.6305 cycles 15692.6869 cycles 15792.4130 cycles]
                        thrpt:  [0.9639 cpb 0.9578 cpb 0.9526 cpb]
                 change:
                        time:   [+0.7331% +1.6425% +2.7218%] (p = 0.00 < 0.05)
                        thrpt:  [-2.6497% -1.6160% -0.7277%]
                        Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
```
## ChaCha20Rng::fill_bytes()
```
ChaCha20Rng/fill_bytes/1024
                        time:   [982.7928 cycles 987.2659 cycles 992.5007 cycles]
                        thrpt:  [0.9692 cpb 0.9641 cpb 0.9598 cpb]
                 change:
                        time:   [-5.0614% -4.2609% -3.4122%] (p = 0.00 < 0.05)
                        thrpt:  [+3.5327% +4.4505% +5.3313%]
                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  6 (6.00%) high mild
  7 (7.00%) high severe
ChaCha20Rng/fill_bytes/2048
                        time:   [1946.2252 cycles 1959.5692 cycles 1976.6045 cycles]
                        thrpt:  [0.9651 cpb 0.9568 cpb 0.9503 cpb]
                 change:
                        time:   [-4.2561% -3.5565% -2.7744%] (p = 0.00 < 0.05)
                        thrpt:  [+2.8536% +3.6877% +4.4453%]
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe
ChaCha20Rng/fill_bytes/4096
                        time:   [3877.0279 cycles 3908.1395 cycles 3945.3822 cycles]
                        thrpt:  [0.9632 cpb 0.9541 cpb 0.9465 cpb]
                 change:
                        time:   [-5.1026% -4.2503% -3.3907%] (p = 0.00 < 0.05)
                        thrpt:  [+3.5097% +4.4390% +5.3770%]
                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  8 (8.00%) high mild
  4 (4.00%) high severe
ChaCha20Rng/fill_bytes/8192
                        time:   [7778.4294 cycles 7842.2564 cycles 7911.6062 cycles]
                        thrpt:  [0.9658 cpb 0.9573 cpb 0.9495 cpb]
                 change:
                        time:   [-3.9617% -3.3186% -2.6300%] (p = 0.00 < 0.05)
                        thrpt:  [+2.7010% +3.4325% +4.1251%]
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe
ChaCha20Rng/fill_bytes/16384
                        time:   [15515.6870 cycles 15625.7733 cycles 15756.2142 cycles]
                        thrpt:  [0.9617 cpb 0.9537 cpb 0.9470 cpb]
                 change:
                        time:   [-5.8070% -4.8278% -3.8428%] (p = 0.00 < 0.05)
                        thrpt:  [+3.9963% +5.0727% +6.1650%]
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe
```