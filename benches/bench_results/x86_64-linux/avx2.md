# Benchmarks for chacha20 using AVX2
These benches were ran on 12/18/2023 using a 3.0 GHz i9 CPU.
This bench is compared against the last one, commit: 378303a
This commit: 97cdfc5

## ChaCha20::apply_keystream()
```
     Running src/chacha20.rs (target/release/deps/chacha20-6f837c27da8cfee7)
Gnuplot not found, using plotters backend
stream-cipher/apply_keystream/1024
                        time:   [1029.9477 cycles 1033.5128 cycles 1037.3747 cycles]
                        thrpt:  [1.0131 cpb 1.0093 cpb 1.0058 cpb]
                 change:
                        time:   [-2.1626% -1.2213% -0.4057%] (p = 0.01 < 0.05)
                        thrpt:  [+0.4073% +1.2364% +2.2104%]
                        Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
stream-cipher/apply_keystream/2048
                        time:   [2039.9442 cycles 2053.1398 cycles 2067.3905 cycles]
                        thrpt:  [1.0095 cpb 1.0025 cpb 0.9961 cpb]
                 change:
                        time:   [-1.8734% -0.9273% -0.1145%] (p = 0.04 < 0.05)
                        thrpt:  [+0.1147% +0.9360% +1.9092%]
                        Change within noise threshold.
Found 10 outliers among 100 measurements (10.00%)
  4 (4.00%) high mild
  6 (6.00%) high severe
stream-cipher/apply_keystream/4096
                        time:   [4052.7380 cycles 4062.3360 cycles 4072.5014 cycles]
                        thrpt:  [0.9943 cpb 0.9918 cpb 0.9894 cpb]
                 change:
                        time:   [-1.9957% -1.4771% -1.0037%] (p = 0.00 < 0.05)
                        thrpt:  [+1.0139% +1.4993% +2.0364%]
                        Performance has improved.
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe
stream-cipher/apply_keystream/8192
                        time:   [8099.0261 cycles 8123.3995 cycles 8152.1496 cycles]
                        thrpt:  [0.9951 cpb 0.9916 cpb 0.9887 cpb]
                 change:
                        time:   [-0.9322% -0.2573% +0.4899%] (p = 0.50 > 0.05)
                        thrpt:  [-0.4875% +0.2579% +0.9410%]
                        No change in performance detected.
Found 10 outliers among 100 measurements (10.00%)
  4 (4.00%) high mild
  6 (6.00%) high severe
stream-cipher/apply_keystream/16384
                        time:   [16163.9533 cycles 16221.4655 cycles 16290.3286 cycles]
                        thrpt:  [0.9943 cpb 0.9901 cpb 0.9866 cpb]
                 change:
                        time:   [-0.5321% -0.0704% +0.3570%] (p = 0.76 > 0.05)
                        thrpt:  [-0.3557% +0.0704% +0.5349%]
                        No change in performance detected.
Found 13 outliers among 100 measurements (13.00%)
  6 (6.00%) high mild
  7 (7.00%) high severe
```
## ChaCha20Rng::fill_bytes()
```
ChaCha20Rng/fill_bytes/1024
                        time:   [950.1280 cycles 954.8761 cycles 960.9864 cycles]
                        thrpt:  [0.9385 cpb 0.9325 cpb 0.9279 cpb]
                 change:
                        time:   [-2.0516% -1.4293% -0.7375%] (p = 0.00 < 0.05)
                        thrpt:  [+0.7429% +1.4500% +2.0946%]
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe
ChaCha20Rng/fill_bytes/2048
                        time:   [1893.2543 cycles 1901.5493 cycles 1911.9818 cycles]
                        thrpt:  [0.9336 cpb 0.9285 cpb 0.9244 cpb]
                 change:
                        time:   [-0.7933% -0.1917% +0.4488%] (p = 0.55 > 0.05)
                        thrpt:  [-0.4468% +0.1920% +0.7996%]
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe
ChaCha20Rng/fill_bytes/4096
                        time:   [3781.5977 cycles 3801.4851 cycles 3824.5162 cycles]
                        thrpt:  [0.9337 cpb 0.9281 cpb 0.9232 cpb]
                 change:
                        time:   [-1.4956% -0.8224% -0.1254%] (p = 0.02 < 0.05)
                        thrpt:  [+0.1255% +0.8292% +1.5183%]
                        Change within noise threshold.
Found 10 outliers among 100 measurements (10.00%)
  4 (4.00%) high mild
  6 (6.00%) high severe
ChaCha20Rng/fill_bytes/8192
                        time:   [7530.0898 cycles 7555.0686 cycles 7586.7694 cycles]
                        thrpt:  [0.9261 cpb 0.9222 cpb 0.9192 cpb]
                 change:
                        time:   [-2.8542% -2.1555% -1.5177%] (p = 0.00 < 0.05)
                        thrpt:  [+1.5411% +2.2030% +2.9380%]
                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) high mild
  7 (7.00%) high severe
ChaCha20Rng/fill_bytes/16384
                        time:   [15194.5407 cycles 15267.6254 cycles 15349.2592 cycles]
                        thrpt:  [0.9368 cpb 0.9319 cpb 0.9274 cpb]
                 change:
                        time:   [-1.9886% -1.0572% -0.2646%] (p = 0.02 < 0.05)
                        thrpt:  [+0.2653% +1.0685% +2.0289%]
                        Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe
```