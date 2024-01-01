# Benchmarks for chacha20 using AVX2
These benches were ran on 12/18/2023 using a 3.0 GHz i9 CPU.
This bench is compared against the last one, commit: 378303a
This commit: 97cdfc5

## ChaCha20::apply_keystream()
```
     Running src/chacha20.rs (target/release/deps/chacha20-6f837c27da8cfee7)
Gnuplot not found, using plotters backend
stream-cipher/apply_keystream/1024
                        time:   [957.8124 cycles 958.2293 cycles 958.7342 cycles]
                        thrpt:  [0.9363 cpb 0.9358 cpb 0.9354 cpb]
                 change:
                        time:   [-4.6878% -4.1909% -3.7345%] (p = 0.00 < 0.05)
                        thrpt:  [+3.8794% +4.3742% +4.9184%]
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) low mild
  4 (4.00%) high mild
  3 (3.00%) high severe
stream-cipher/apply_keystream/2048
                        time:   [1906.6587 cycles 1908.6083 cycles 1911.3419 cycles]
                        thrpt:  [0.9333 cpb 0.9319 cpb 0.9310 cpb]
                 change:
                        time:   [-2.3585% -1.9491% -1.5503%] (p = 0.00 < 0.05)
                        thrpt:  [+1.5747% +1.9879% +2.4155%]
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
stream-cipher/apply_keystream/4096
                        time:   [3806.3942 cycles 3811.2544 cycles 3817.6644 cycles]
                        thrpt:  [0.9320 cpb 0.9305 cpb 0.9293 cpb]
                 change:
                        time:   [-2.3627% -1.8225% -1.3634%] (p = 0.00 < 0.05)
                        thrpt:  [+1.3823% +1.8563% +2.4199%]
                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  7 (7.00%) high severe
stream-cipher/apply_keystream/8192
                        time:   [7733.4907 cycles 7775.6224 cycles 7822.7032 cycles]
                        thrpt:  [0.9549 cpb 0.9492 cpb 0.9440 cpb]
                 change:
                        time:   [-2.2783% -0.4983% +0.9400%] (p = 0.58 > 0.05)
                        thrpt:  [-0.9312% +0.5008% +2.3314%]
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe
stream-cipher/apply_keystream/16384
                        time:   [15501.0803 cycles 15610.5622 cycles 15740.7526 cycles]
                        thrpt:  [0.9607 cpb 0.9528 cpb 0.9461 cpb]
                 change:
                        time:   [+0.4677% +1.2511% +2.2194%] (p = 0.00 < 0.05)
                        thrpt:  [-2.1712% -1.2356% -0.4655%]
                        Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe
```
## ChaCha20Rng::fill_bytes()
```
ChaCha20Rng/fill_bytes/1024
                        time:   [961.9632 cycles 967.4754 cycles 973.6152 cycles]
                        thrpt:  [0.9508 cpb 0.9448 cpb 0.9394 cpb]
                 change:
                        time:   [-2.6722% -1.8412% -0.9991%] (p = 0.00 < 0.05)
                        thrpt:  [+1.0092% +1.8757% +2.7456%]
                        Change within noise threshold.
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe
ChaCha20Rng/fill_bytes/2048
                        time:   [1906.7578 cycles 1913.2175 cycles 1919.6353 cycles]
                        thrpt:  [0.9373 cpb 0.9342 cpb 0.9310 cpb]
                 change:
                        time:   [-2.1818% -1.7631% -1.3289%] (p = 0.00 < 0.05)
                        thrpt:  [+1.3468% +1.7948% +2.2305%]
                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
ChaCha20Rng/fill_bytes/4096
                        time:   [3842.0112 cycles 3858.6417 cycles 3876.6690 cycles]
                        thrpt:  [0.9465 cpb 0.9421 cpb 0.9380 cpb]
                 change:
                        time:   [-1.0650% -0.5005% +0.0644%] (p = 0.08 > 0.05)
                        thrpt:  [-0.0643% +0.5031% +1.0765%]
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high mild
ChaCha20Rng/fill_bytes/8192
                        time:   [7641.3253 cycles 7675.0733 cycles 7715.0001 cycles]
                        thrpt:  [0.9418 cpb 0.9369 cpb 0.9328 cpb]
                 change:
                        time:   [-1.4356% -0.6958% +0.0424%] (p = 0.07 > 0.05)
                        thrpt:  [-0.0424% +0.7007% +1.4565%]
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe
ChaCha20Rng/fill_bytes/16384
                        time:   [15258.6043 cycles 15346.1307 cycles 15454.1358 cycles]
                        thrpt:  [0.9432 cpb 0.9367 cpb 0.9313 cpb]
                 change:
                        time:   [-1.8515% -1.0812% -0.3516%] (p = 0.01 < 0.05)
                        thrpt:  [+0.3529% +1.0931% +1.8864%]
                        Change within noise threshold.
Found 11 outliers among 100 measurements (11.00%)
  6 (6.00%) high mild
  5 (5.00%) high severe
```