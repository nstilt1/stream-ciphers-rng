# Benching chacha20 using AVX2 and zeroize
These benches were ran on 1/7/2024 using a 3.0 GHz i9 CPU after impling `ZeroizeOnDrop` for the backends.
## Cipher
Gnuplot not found, using plotters backend
stream-cipher/apply_keystream/1024                                                                            
                        time:   [994.0656 cycles 999.1554 cycles 1005.3121 cycles]
                        thrpt:  [0.9818 cpb 0.9757 cpb 0.9708 cpb]
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe
stream-cipher/apply_keystream/2048                                                                             
                        time:   [1948.4408 cycles 1956.3192 cycles 1966.1652 cycles]
                        thrpt:  [0.9600 cpb 0.9552 cpb 0.9514 cpb]
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe
stream-cipher/apply_keystream/4096                                                                             
                        time:   [3912.3491 cycles 3961.3904 cycles 4021.2151 cycles]
                        thrpt:  [0.9817 cpb 0.9671 cpb 0.9552 cpb]
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe
stream-cipher/apply_keystream/8192                                                                             
                        time:   [7862.8613 cycles 7954.1754 cycles 8057.9583 cycles]
                        thrpt:  [0.9836 cpb 0.9710 cpb 0.9598 cpb]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
stream-cipher/apply_keystream/16384                                                                             
                        time:   [15589.7397 cycles 15666.2580 cycles 15750.1348 cycles]
                        thrpt:  [0.9613 cpb 0.9562 cpb 0.9515 cpb]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high severe
## Rng
chacha-SIMD-comparison-x86/fill_bytes/1024                                                                            
                        time:   [1102.0769 cycles 1110.8181 cycles 1120.4514 cycles]
                        thrpt:  [1.0942 cpb 1.0848 cpb 1.0762 cpb]
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe
chacha-SIMD-comparison-x86/fill_bytes/2048                                                                             
                        time:   [2172.6894 cycles 2188.8218 cycles 2208.4025 cycles]
                        thrpt:  [1.0783 cpb 1.0688 cpb 1.0609 cpb]
Found 14 outliers among 100 measurements (14.00%)
  10 (10.00%) high mild
  4 (4.00%) high severe
chacha-SIMD-comparison-x86/fill_bytes/4096                                                                             
                        time:   [4360.4714 cycles 4393.3799 cycles 4429.1466 cycles]
                        thrpt:  [1.0813 cpb 1.0726 cpb 1.0646 cpb]
Found 10 outliers among 100 measurements (10.00%)
  7 (7.00%) high mild
  3 (3.00%) high severe
chacha-SIMD-comparison-x86/fill_bytes/8192                                                                             
                        time:   [8677.8619 cycles 8733.5175 cycles 8799.2676 cycles]
                        thrpt:  [1.0741 cpb 1.0661 cpb 1.0593 cpb]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
chacha-SIMD-comparison-x86/fill_bytes/16384                                                                             
                        time:   [17403.5219 cycles 17588.0777 cycles 17844.7899 cycles]
                        thrpt:  [1.0892 cpb 1.0735 cpb 1.0622 cpb]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) high mild
  5 (5.00%) high severe