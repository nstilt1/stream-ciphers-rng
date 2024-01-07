# Benching chacha20 using the software implementation and zeroize
These benches were ran on 1/7/2024 using a 3.0 GHz i9 CPU after impling `ZeroizeOnDrop` for the backends.
## Cipher
Gnuplot not found, using plotters backend
stream-cipher/apply_keystream/1024                                                                             
                        time:   [5481.4756 cycles 5572.7008 cycles 5675.0485 cycles]
                        thrpt:  [5.5420 cpb 5.4421 cpb 5.3530 cpb]
Found 8 outliers among 100 measurements (8.00%)
  7 (7.00%) high mild
  1 (1.00%) high severe
stream-cipher/apply_keystream/2048                                                                             
                        time:   [10644.0042 cycles 10758.2066 cycles 10900.8212 cycles]
                        thrpt:  [5.3227 cpb 5.2530 cpb 5.1973 cpb]
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe
stream-cipher/apply_keystream/4096                                                                             
                        time:   [21439.1825 cycles 21685.5707 cycles 21982.9472 cycles]
                        thrpt:  [5.3669 cpb 5.2943 cpb 5.2342 cpb]
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe
stream-cipher/apply_keystream/8192                                                                             
                        time:   [42585.6020 cycles 43143.5948 cycles 43876.8937 cycles]
                        thrpt:  [5.3561 cpb 5.2666 cpb 5.1984 cpb]
Found 16 outliers among 100 measurements (16.00%)
  10 (10.00%) high mild
  6 (6.00%) high severe
stream-cipher/apply_keystream/16384                                                                             
                        time:   [85847.0329 cycles 87006.2075 cycles 88456.8956 cycles]
                        thrpt:  [5.3990 cpb 5.3104 cpb 5.2397 cpb]
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) high mild
  6 (6.00%) high severe
## Rng
chacha-SIMD-comparison-x86/fill_bytes/1024                                                                             
                        time:   [5546.9905 cycles 5637.5032 cycles 5740.9023 cycles]
                        thrpt:  [5.6063 cpb 5.5054 cpb 5.4170 cpb]
Found 13 outliers among 100 measurements (13.00%)
  8 (8.00%) high mild
  5 (5.00%) high severe
chacha-SIMD-comparison-x86/fill_bytes/2048                                                                             
                        time:   [10931.0646 cycles 11040.2793 cycles 11162.3477 cycles]
                        thrpt:  [5.4504 cpb 5.3908 cpb 5.3374 cpb]
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe
chacha-SIMD-comparison-x86/fill_bytes/4096                                                                             
                        time:   [22013.5065 cycles 22257.3454 cycles 22527.2027 cycles]
                        thrpt:  [5.4998 cpb 5.4339 cpb 5.3744 cpb]
Found 9 outliers among 100 measurements (9.00%)
  9 (9.00%) high mild
chacha-SIMD-comparison-x86/fill_bytes/8192                                                                             
                        time:   [43707.8084 cycles 44123.1612 cycles 44599.6750 cycles]
                        thrpt:  [5.4443 cpb 5.3861 cpb 5.3354 cpb]
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) high mild
  6 (6.00%) high severe
chacha-SIMD-comparison-x86/fill_bytes/16384                                                                             
                        time:   [87260.6173 cycles 87995.8802 cycles 88800.3873 cycles]
                        thrpt:  [5.4199 cpb 5.3708 cpb 5.3260 cpb]
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe