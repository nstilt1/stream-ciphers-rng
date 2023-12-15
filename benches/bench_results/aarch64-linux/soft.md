# Benchmarks for chacha20 using the Soft implementation
These benches were ran on 12/14/2023 using a Raspberry Pi 4.

## ChaCha20::apply_keystream()
     Running src//chacha20.rs (target/release/deps/chacha20-02f555ae0af3670b)
Gnuplot not found, using plotters backend
Benchmarking stream-cipher/apply_keystream/1024: Collecting 100 samples in estimstream-cipher/apply_keystream/1024
                        time:   [24432.5284 cycles 24433.8864 cycles 24435.4709 cycles]
                        thrpt:  [23.8628 cpb 23.8612 cpb 23.8599 cpb]
Found 17 outliers among 100 measurements (17.00%)
  2 (2.00%) low severe
  2 (2.00%) low mild
  4 (4.00%) high mild
  9 (9.00%) high severe
Benchmarking stream-cipher/apply_keystream/2048: Collecting 100 samples in estimstream-cipher/apply_keystream/2048
                        time:   [48503.4127 cycles 48575.0904 cycles 48639.3134 cycles]
                        thrpt:  [23.7497 cpb 23.7183 cpb 23.6833 cpb]
Benchmarking stream-cipher/apply_keystream/4096: Collecting 100 samples in estimstream-cipher/apply_keystream/4096
                        time:   [97558.5737 cycles 97570.7093 cycles 97579.1194 cycles]
                        thrpt:  [23.8230 cpb 23.8210 cpb 23.8180 cpb]
Found 15 outliers among 100 measurements (15.00%)
  9 (9.00%) low severe
  1 (1.00%) low mild
  2 (2.00%) high mild
  3 (3.00%) high severe
Benchmarking stream-cipher/apply_keystream/8192: Collecting 100 samples in estimstream-cipher/apply_keystream/8192
                        time:   [195117.1427 cycles 195127.3258 cycles 195140.4433 cycles]
                        thrpt:  [23.8209 cpb 23.8193 cpb 23.8180 cpb]
Found 17 outliers among 100 measurements (17.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  5 (5.00%) high mild
  9 (9.00%) high severe
Benchmarking stream-cipher/apply_keystream/16384: Collecting 100 samples in estistream-cipher/apply_keystream/16384
                        time:   [390203.3377 cycles 390218.3760 cycles 390236.1068 cycles]
                        thrpt:  [23.8181 cpb 23.8170 cpb 23.8161 cpb]
Found 16 outliers among 100 measurements (16.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  8 (8.00%) high mild
  5 (5.00%) high severe

## ChaCha20Rng::fill_bytes()
Benchmarking ChaCha20Rng/fill_bytes/1024: Collecting 100 samples in estimated 5.ChaCha20Rng/fill_bytes/1024
                        time:   [24198.9063 cycles 24223.9863 cycles 24248.0106 cycles]
                        thrpt:  [23.6797 cpb 23.6562 cpb 23.6317 cpb]
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) low severe
  6 (6.00%) low mild
Benchmarking ChaCha20Rng/fill_bytes/2048: Collecting 100 samples in estimated 5.ChaCha20Rng/fill_bytes/2048
                        time:   [47867.4743 cycles 47969.1502 cycles 48065.7810 cycles]
                        thrpt:  [23.4696 cpb 23.4224 cpb 23.3728 cpb]
Benchmarking ChaCha20Rng/fill_bytes/4096: Collecting 100 samples in estimated 5.ChaCha20Rng/fill_bytes/4096
                        time:   [96798.4981 cycles 96882.1961 cycles 96966.3228 cycles]
                        thrpt:  [23.6734 cpb 23.6529 cpb 23.6324 cpb]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) low mild
Benchmarking ChaCha20Rng/fill_bytes/8192: Collecting 100 samples in estimated 5.ChaCha20Rng/fill_bytes/8192
                        time:   [193592.4063 cycles 193741.8438 cycles 193891.4600 cycles]
                        thrpt:  [23.6684 cpb 23.6501 cpb 23.6319 cpb]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) low mild
Benchmarking ChaCha20Rng/fill_bytes/16384: Collecting 100 samples in estimated 5ChaCha20Rng/fill_bytes/16384
                        time:   [382640.9407 cycles 383239.9480 cycles 383879.9009 cycles]
                        thrpt:  [23.4302 cpb 23.3911 cpb 23.3545 cpb]