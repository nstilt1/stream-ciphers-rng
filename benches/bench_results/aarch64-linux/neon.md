# Benchmarks for chacha20 using AVX2 and zeroize for the backends
These benches were ran on 1/6/2024 using a Raspberry Pi Model 4b

## ChaCha20::apply_keystream()
```
     Running src/chacha20.rs (target/release/deps/chacha20-066064f66a8e404e)
Gnuplot not found, using plotters backend
Benchmarking stream-cipher/apply_keystream/1024: Collecting 100 samples in estimstream-cipher/apply_keystream/1024
                        time:   [5639.8700 cycles 5642.8530 cycles 5645.8067 cycles]
                        thrpt:  [5.5135 cpb 5.5106 cpb 5.5077 cpb]
                 change:
                        time:   [-0.4676% -0.4102% -0.3516%] (p = 0.00 < 0.05)
                        thrpt:  [+0.3528% +0.4119% +0.4698%]
                        Change within noise threshold.
Benchmarking stream-cipher/apply_keystream/2048: Collecting 100 samples in estimstream-cipher/apply_keystream/2048
                        time:   [11192.2365 cycles 11195.6098 cycles 11202.5461 cycles]
                        thrpt:  [5.4700 cpb 5.4666 cpb 5.4650 cpb]
                 change:
                        time:   [-0.4047% -0.3577% -0.2759%] (p = 0.00 < 0.05)
                        thrpt:  [+0.2767% +0.3590% +0.4064%]
                        Change within noise threshold.
Found 17 outliers among 100 measurements (17.00%)
  2 (2.00%) low severe
  1 (1.00%) low mild
  7 (7.00%) high mild
  7 (7.00%) high severe
Benchmarking stream-cipher/apply_keystream/4096: Collecting 100 samples in estimstream-cipher/apply_keystream/4096
                        time:   [22322.5785 cycles 22323.9442 cycles 22325.5818 cycles]
                        thrpt:  [5.4506 cpb 5.4502 cpb 5.4498 cpb]
                 change:
                        time:   [-0.4127% -0.3849% -0.3537%] (p = 0.00 < 0.05)
                        thrpt:  [+0.3549% +0.3864% +0.4144%]
                        Change within noise threshold.
Found 10 outliers among 100 measurements (10.00%)
  2 (2.00%) low severe
  1 (1.00%) low mild
  4 (4.00%) high mild
  3 (3.00%) high severe
Benchmarking stream-cipher/apply_keystream/8192: Collecting 100 samples in estimstream-cipher/apply_keystream/8192
                        time:   [44606.0165 cycles 44619.3032 cycles 44633.0404 cycles]
                        thrpt:  [5.4484 cpb 5.4467 cpb 5.4451 cpb]
                 change:
                        time:   [-0.3100% -0.2880% -0.2649%] (p = 0.00 < 0.05)
                        thrpt:  [+0.2656% +0.2888% +0.3110%]
                        Change within noise threshold.
Found 23 outliers among 100 measurements (23.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  18 (18.00%) high severe
Benchmarking stream-cipher/apply_keystream/16384: Collecting 100 samples in estistream-cipher/apply_keystream/16384
                        time:   [89103.7557 cycles 89107.8725 cycles 89115.1389 cycles]
                        thrpt:  [5.4392 cpb 5.4387 cpb 5.4385 cpb]
                 change:
                        time:   [-1.1200% -0.8610% -0.7208%] (p = 0.00 < 0.05)
                        thrpt:  [+0.7261% +0.8685% +1.1326%]
                        Change within noise threshold.
Found 13 outliers among 100 measurements (13.00%)
  2 (2.00%) low severe
  2 (2.00%) low mild
  1 (1.00%) high mild
  8 (8.00%) high severe

```
## ChaCha20Rng::fill_bytes()
```
Benchmarking ChaCha20Rng/fill_bytes/1024: Collecting 100 samples in estimated 5.ChaCha20Rng/fill_bytes/1024
                        time:   [5503.0151 cycles 5503.7816 cycles 5504.6071 cycles]
                        thrpt:  [5.3756 cpb 5.3748 cpb 5.3740 cpb]
                 change:
                        time:   [-0.7049% -0.5296% -0.4177%] (p = 0.00 < 0.05)
                        thrpt:  [+0.4195% +0.5324% +0.7099%]
                        Change within noise threshold.
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe
Benchmarking ChaCha20Rng/fill_bytes/2048: Collecting 100 samples in estimated 5.ChaCha20Rng/fill_bytes/2048
                        time:   [10941.8876 cycles 10942.1713 cycles 10942.5220 cycles]
                        thrpt:  [5.3430 cpb 5.3429 cpb 5.3427 cpb]
                 change:
                        time:   [-0.7389% -0.7191% -0.6949%] (p = 0.00 < 0.05)
                        thrpt:  [+0.6998% +0.7243% +0.7444%]
                        Change within noise threshold.
Found 12 outliers among 100 measurements (12.00%)
  2 (2.00%) low severe
  2 (2.00%) low mild
  2 (2.00%) high mild
  6 (6.00%) high severe
Benchmarking ChaCha20Rng/fill_bytes/4096: Collecting 100 samples in estimated 5.ChaCha20Rng/fill_bytes/4096
                        time:   [21835.6489 cycles 21836.6611 cycles 21838.1757 cycles]
                        thrpt:  [5.3316 cpb 5.3312 cpb 5.3310 cpb]
                 change:
                        time:   [-0.9491% -0.9239% -0.8998%] (p = 0.00 < 0.05)
                        thrpt:  [+0.9080% +0.9325% +0.9582%]
                        Change within noise threshold.
Found 14 outliers among 100 measurements (14.00%)
  2 (2.00%) low severe
  1 (1.00%) low mild
  5 (5.00%) high mild
  6 (6.00%) high severe
Benchmarking ChaCha20Rng/fill_bytes/8192: Collecting 100 samples in estimated 5.ChaCha20Rng/fill_bytes/8192
                        time:   [43617.2426 cycles 43618.1146 cycles 43619.1340 cycles]
                        thrpt:  [5.3246 cpb 5.3245 cpb 5.3244 cpb]
                 change:
                        time:   [-0.9279% -0.8987% -0.8721%] (p = 0.00 < 0.05)
                        thrpt:  [+0.8797% +0.9069% +0.9366%]
                        Change within noise threshold.
Found 10 outliers among 100 measurements (10.00%)
  2 (2.00%) low severe
  4 (4.00%) high mild
  4 (4.00%) high severe
Benchmarking ChaCha20Rng/fill_bytes/16384: Collecting 100 samples in estimated 5ChaCha20Rng/fill_bytes/16384
                        time:   [87192.0424 cycles 87193.6887 cycles 87195.5921 cycles]
                        thrpt:  [5.3220 cpb 5.3219 cpb 5.3218 cpb]
                 change:
                        time:   [-0.9448% -0.9224% -0.9065%] (p = 0.00 < 0.05)
                        thrpt:  [+0.9148% +0.9310% +0.9539%]
                        Change within noise threshold.
Found 12 outliers among 100 measurements (12.00%)
  2 (2.00%) low severe
  1 (1.00%) low mild
  6 (6.00%) high mild
  3 (3.00%) high severe
```