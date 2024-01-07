# Benching chacha20 using SSE2 and zeroize
These benches were ran on 1/7/2024 using a 3.0 GHz i9 CPU after impling `ZeroizeOnDrop` for the backends.
## Cipher
Gnuplot not found, using plotters backend
stream-cipher/apply_keystream/1024                                                                             
                        time:   [3059.6833 cycles 3072.7088 cycles 3087.8967 cycles]
                        thrpt:  [3.0155 cpb 3.0007 cpb 2.9880 cpb]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe
stream-cipher/apply_keystream/2048                                                                             
                        time:   [6090.6506 cycles 6121.1111 cycles 6156.0687 cycles]
                        thrpt:  [3.0059 cpb 2.9888 cpb 2.9740 cpb]
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe
stream-cipher/apply_keystream/4096                                                                             
                        time:   [12203.0203 cycles 12253.1127 cycles 12309.1702 cycles]
                        thrpt:  [3.0052 cpb 2.9915 cpb 2.9793 cpb]
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe
stream-cipher/apply_keystream/8192                                                                             
                        time:   [24209.3013 cycles 24360.3693 cycles 24530.2273 cycles]
                        thrpt:  [2.9944 cpb 2.9737 cpb 2.9552 cpb]
Found 10 outliers among 100 measurements (10.00%)
  8 (8.00%) high mild
  2 (2.00%) high severe
stream-cipher/apply_keystream/16384                                                                             
                        time:   [48566.5034 cycles 48962.9482 cycles 49474.5260 cycles]
                        thrpt:  [3.0197 cpb 2.9885 cpb 2.9643 cpb]
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe
  
## Rng
chacha-SIMD-comparison-x86/fill_bytes/1024                                                                             
                        time:   [3024.3691 cycles 3038.8125 cycles 3054.3135 cycles]
                        thrpt:  [2.9827 cpb 2.9676 cpb 2.9535 cpb]
Found 10 outliers among 100 measurements (10.00%)
  9 (9.00%) high mild
  1 (1.00%) high severe
chacha-SIMD-comparison-x86/fill_bytes/2048                                                                             
                        time:   [6086.3564 cycles 6109.0015 cycles 6133.0261 cycles]
                        thrpt:  [2.9946 cpb 2.9829 cpb 2.9719 cpb]
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe
chacha-SIMD-comparison-x86/fill_bytes/4096                                                                             
                        time:   [12149.2000 cycles 12189.5143 cycles 12232.0129 cycles]
                        thrpt:  [2.9863 cpb 2.9760 cpb 2.9661 cpb]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
chacha-SIMD-comparison-x86/fill_bytes/8192                                                                             
                        time:   [24363.4351 cycles 24536.8944 cycles 24759.3034 cycles]
                        thrpt:  [3.0224 cpb 2.9952 cpb 2.9741 cpb]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
chacha-SIMD-comparison-x86/fill_bytes/16384                                                                             
                        time:   [48341.7282 cycles 48531.0519 cycles 48779.6832 cycles]
                        thrpt:  [2.9773 cpb 2.9621 cpb 2.9505 cpb]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe