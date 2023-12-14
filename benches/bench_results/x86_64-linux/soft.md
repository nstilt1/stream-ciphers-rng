# Benchmarks for chacha20 using Soft implementation
These benches were ran on 12/14/2023 using a 3.0 GHz i9 CPU.

## ChaCha20::apply_keystream()
     Running src/chacha20.rs (target/release/deps/chacha20-bc9efd54ac287186)
Gnuplot not found, using plotters backend
stream-cipher/apply_keystream/1024
                        time:   [5218.5540 cycles 5227.5614 cycles 5237.6494 cycles]
                        thrpt:  [5.1149 cpb 5.1050 cpb 5.0962 cpb]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
stream-cipher/apply_keystream/2048
                        time:   [10441.2745 cycles 10463.5009 cycles 10487.5586 cycles]
                        thrpt:  [5.1209 cpb 5.1091 cpb 5.0983 cpb]
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe
stream-cipher/apply_keystream/4096
                        time:   [20923.5960 cycles 20974.5325 cycles 21030.2328 cycles]
                        thrpt:  [5.1343 cpb 5.1207 cpb 5.1083 cpb]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
stream-cipher/apply_keystream/8192
                        time:   [41773.1520 cycles 41842.3158 cycles 41915.6373 cycles]
                        thrpt:  [5.1167 cpb 5.1077 cpb 5.0993 cpb]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
stream-cipher/apply_keystream/16384
                        time:   [83641.9959 cycles 83783.7728 cycles 83928.5586 cycles]
                        thrpt:  [5.1226 cpb 5.1138 cpb 5.1051 cpb]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

## ChaCha20Rng::fill_bytes()
ChaCha20Rng/fill_bytes/1024
                        time:   [4736.7752 cycles 4745.4223 cycles 4754.8510 cycles]
                        thrpt:  [4.6434 cpb 4.6342 cpb 4.6258 cpb]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe
ChaCha20Rng/fill_bytes/2048
                        time:   [9490.3993 cycles 9508.1738 cycles 9526.7642 cycles]
                        thrpt:  [4.6517 cpb 4.6427 cpb 4.6340 cpb]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe
ChaCha20Rng/fill_bytes/4096
                        time:   [18932.7718 cycles 18973.7653 cycles 19019.1529 cycles]
                        thrpt:  [4.6433 cpb 4.6323 cpb 4.6223 cpb]
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe
ChaCha20Rng/fill_bytes/8192
                        time:   [37949.5728 cycles 38039.0155 cycles 38131.9237 cycles]
                        thrpt:  [4.6548 cpb 4.6434 cpb 4.6325 cpb]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe
ChaCha20Rng/fill_bytes/16384
                        time:   [75752.6325 cycles 75862.7721 cycles 75977.3625 cycles]
                        thrpt:  [4.6373 cpb 4.6303 cpb 4.6236 cpb]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild