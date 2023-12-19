# Benchmarks for chacha20 using NEON
These benches were ran on 12/16/2023 using a Raspberry Pi 4b.
The results are compared against the soft implementation.
This bench is compared against the last one, commit: 378303a
This commit: 97cdfc5

## ChaCha20::apply_keystream()
```
     Running src/chacha20.rs (target/release/deps/chacha20-066064f66a8e404e)
Gnuplot not found, using plotters backend
stream-cipher/apply_keystream/1024
                        time:   [5663.1130 cycles 5664.7384 cycles 5666.3671 cycles]
                        thrpt:  [5.5336 cpb 5.5320 cpb 5.5304 cpb]
                 change:
                        time:   [-76.761% -76.753% -76.745%] (p = 0.00 < 0.05)
                        thrpt:  [+330.01% +330.16% +330.31%]
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
stream-cipher/apply_keystream/2048
                        time:   [11236.2742 cycles 11236.4735 cycles 11236.7120 cycles]
                        thrpt:  [5.4867 cpb 5.4866 cpb 5.4865 cpb]
                 change:
                        time:   [-76.943% -76.939% -76.936%] (p = 0.00 < 0.05)
                        thrpt:  [+333.58% +333.63% +333.70%]
                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  2 (2.00%) low severe
  2 (2.00%) low mild
  2 (2.00%) high mild
  5 (5.00%) high severe
stream-cipher/apply_keystream/4096
                        time:   [22414.0715 cycles 22421.5037 cycles 22428.6990 cycles]
                        thrpt:  [5.4758 cpb 5.4740 cpb 5.4722 cpb]
                 change:
                        time:   [-77.019% -77.005% -76.995%] (p = 0.00 < 0.05)
                        thrpt:  [+334.69% +334.87% +335.14%]
                        Performance has improved.
Found 19 outliers among 100 measurements (19.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
  15 (15.00%) high severe
stream-cipher/apply_keystream/8192
                        time:   [44732.0691 cycles 44733.1569 cycles 44734.3759 cycles]
                        thrpt:  [5.4607 cpb 5.4606 cpb 5.4605 cpb]
                 change:
                        time:   [-77.090% -77.064% -77.045%] (p = 0.00 < 0.05)
                        thrpt:  [+335.63% +335.99% +336.49%]
                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  2 (2.00%) low severe
  3 (3.00%) high mild
  5 (5.00%) high severe
stream-cipher/apply_keystream/16384
                        time:   [89773.9768 cycles 89786.9712 cycles 89806.0094 cycles]
                        thrpt:  [5.4813 cpb 5.4802 cpb 5.4794 cpb]
                 change:
                        time:   [-77.000% -76.969% -76.910%] (p = 0.00 < 0.05)
                        thrpt:  [+333.09% +334.19% +334.79%]
                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  2 (2.00%) low severe
  1 (1.00%) low mild
  2 (2.00%) high mild
  6 (6.00%) high severe
```
## ChaCha20Rng::fill_bytes()
```
ChaCha20Rng/fill_bytes/1024
                        time:   [5528.5089 cycles 5531.7093 cycles 5536.5838 cycles]
                        thrpt:  [5.4068 cpb 5.4021 cpb 5.3989 cpb]
                 change:
                        time:   [-77.043% -77.015% -76.967%] (p = 0.00 < 0.05)
                        thrpt:  [+334.17% +335.06% +335.59%]
                        Performance has improved.
Found 15 outliers among 100 measurements (15.00%)
  4 (4.00%) low mild
  8 (8.00%) high mild
  3 (3.00%) high severe
ChaCha20Rng/fill_bytes/2048
                        time:   [11021.0685 cycles 11022.2222 cycles 11023.3029 cycles]
                        thrpt:  [5.3825 cpb 5.3819 cpb 5.3814 cpb]
                 change:
                        time:   [-77.178% -77.161% -77.145%] (p = 0.00 < 0.05)
                        thrpt:  [+337.54% +337.84% +338.18%]
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe
ChaCha20Rng/fill_bytes/4096
                        time:   [22021.6757 cycles 22024.0396 cycles 22026.8780 cycles]
                        thrpt:  [5.3777 cpb 5.3770 cpb 5.3764 cpb]
                 change:
                        time:   [-77.118% -77.100% -77.088%] (p = 0.00 < 0.05)
                        thrpt:  [+336.46% +336.68% +337.02%]
                        Performance has improved.
ChaCha20Rng/fill_bytes/8192
                        time:   [44012.5687 cycles 44013.9395 cycles 44015.3632 cycles]
                        thrpt:  [5.3730 cpb 5.3728 cpb 5.3726 cpb]
                 change:
                        time:   [-77.310% -77.286% -77.262%] (p = 0.00 < 0.05)
                        thrpt:  [+339.80% +340.27% +340.73%]
                        Performance has improved.
Found 19 outliers among 100 measurements (19.00%)
  4 (4.00%) low severe
  5 (5.00%) low mild
  3 (3.00%) high mild
  7 (7.00%) high severe
ChaCha20Rng/fill_bytes/16384
                        time:   [87992.1011 cycles 88000.6051 cycles 88011.3685 cycles]
                        thrpt:  [5.3718 cpb 5.3711 cpb 5.3706 cpb]
                 change:
                        time:   [-77.261% -77.235% -77.207%] (p = 0.00 < 0.05)
                        thrpt:  [+338.73% +339.26% +339.78%]
                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  2 (2.00%) low severe
  2 (2.00%) low mild
  4 (4.00%) high mild
  3 (3.00%) high severe
```