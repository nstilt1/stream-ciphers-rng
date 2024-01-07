# Benchmarks for chacha20 using Soft implementation
These benches were ran on 1/7/2024 using a 3.0 GHz i9 CPU after changing `soft.rs` by removing the `Backend` struct, and instead implementing `StreamBackend` for `ChaChaCore`.

## ChaCha20::apply_keystream()
```
     Running src/chacha20.rs (target/release/deps/chacha20-ec6cae07021ae448)
Gnuplot not found, using plotters backend
stream-cipher/apply_keystream/1024
                        time:   [5331.9195 cycles 5381.9934 cycles 5437.3186 cycles]
                        thrpt:  [5.3099 cpb 5.2559 cpb 5.2070 cpb]
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) high mild
  7 (7.00%) high severe
stream-cipher/apply_keystream/2048
                        time:   [10598.9237 cycles 10645.5383 cycles 10697.1537 cycles]
                        thrpt:  [5.2232 cpb 5.1980 cpb 5.1753 cpb]
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe
stream-cipher/apply_keystream/4096
                        time:   [21380.3750 cycles 21551.7043 cycles 21733.9517 cycles]
                        thrpt:  [5.3061 cpb 5.2616 cpb 5.2198 cpb]
Found 9 outliers among 100 measurements (9.00%)
  6 (6.00%) high mild
  3 (3.00%) high severe
stream-cipher/apply_keystream/8192
                        time:   [42653.9597 cycles 43000.0606 cycles 43405.4639 cycles]
                        thrpt:  [5.2985 cpb 5.2490 cpb 5.2068 cpb]
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe
stream-cipher/apply_keystream/16384
                        time:   [86389.8661 cycles 87724.6072 cycles 89315.0435 cycles]
                        thrpt:  [5.4514 cpb 5.3543 cpb 5.2728 cpb]
Found 11 outliers among 100 measurements (11.00%)
  7 (7.00%) high mild
  4 (4.00%) high severe
```
## ChaCha20Rng::fill_bytes()
```
ChaCha20Rng/fill_bytes/1024
                        time:   [5434.0588 cycles 5481.7613 cycles 5535.7891 cycles]
                        thrpt:  [5.4060 cpb 5.3533 cpb 5.3067 cpb]
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe
ChaCha20Rng/fill_bytes/2048
                        time:   [10820.2930 cycles 10892.4229 cycles 10974.9663 cycles]
                        thrpt:  [5.3589 cpb 5.3186 cpb 5.2833 cpb]
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild
ChaCha20Rng/fill_bytes/4096
                        time:   [21661.7773 cycles 21846.0198 cycles 22063.0999 cycles]
                        thrpt:  [5.3865 cpb 5.3335 cpb 5.2885 cpb]
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe
ChaCha20Rng/fill_bytes/8192
                        time:   [43279.8826 cycles 44011.0918 cycles 44887.0850 cycles]
                        thrpt:  [5.4794 cpb 5.3724 cpb 5.2832 cpb]
Found 13 outliers among 100 measurements (13.00%)
  5 (5.00%) high mild
  8 (8.00%) high severe
ChaCha20Rng/fill_bytes/16384
                        time:   [86528.5974 cycles 87226.0293 cycles 88021.2023 cycles]
                        thrpt:  [5.3724 cpb 5.3239 cpb 5.2813 cpb]
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) high mild
  6 (6.00%) high severe
```


# Re-benched immediately after this bench without using `cargo clean`
For some reason, `soft.rs` sometimes performs at 4.7ish cpb, and sometimes it's at 5.3+ cpb
## ChaCha20::apply_keystream()
```
     Running src/chacha20.rs (target/release/deps/chacha20-ec6cae07021ae448)
Gnuplot not found, using plotters backend
stream-cipher/apply_keystream/1024
                        time:   [4963.2247 cycles 5015.7690 cycles 5079.4132 cycles]
                        thrpt:  [4.9604 cpb 4.8982 cpb 4.8469 cpb]
                 change:
                        time:   [-8.2418% -7.1612% -6.0927%] (p = 0.00 < 0.05)
                        thrpt:  [+6.4880% +7.7136% +8.9821%]
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe
stream-cipher/apply_keystream/2048
                        time:   [9720.8774 cycles 9765.4210 cycles 9810.0805 cycles]
                        thrpt:  [4.7901 cpb 4.7683 cpb 4.7465 cpb]
                 change:
                        time:   [-9.5704% -8.7058% -7.8783%] (p = 0.00 < 0.05)
                        thrpt:  [+8.5520% +9.5360% +10.583%]
                        Performance has improved.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe
stream-cipher/apply_keystream/4096
                        time:   [19569.9790 cycles 19680.9183 cycles 19806.9018 cycles]
                        thrpt:  [4.8357 cpb 4.8049 cpb 4.7778 cpb]
                 change:
                        time:   [-9.0576% -7.6529% -6.2020%] (p = 0.00 < 0.05)
                        thrpt:  [+6.6121% +8.2871% +9.9597%]
                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe
stream-cipher/apply_keystream/8192
                        time:   [38787.4781 cycles 39218.6035 cycles 39724.9017 cycles]
                        thrpt:  [4.8492 cpb 4.7874 cpb 4.7348 cpb]
                 change:
                        time:   [-10.165% -9.1356% -8.1100%] (p = 0.00 < 0.05)
                        thrpt:  [+8.8258% +10.054% +11.315%]
                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe
stream-cipher/apply_keystream/16384
                        time:   [77908.9160 cycles 78630.2971 cycles 79501.0430 cycles]
                        thrpt:  [4.8524 cpb 4.7992 cpb 4.7552 cpb]
                 change:
                        time:   [-10.097% -8.6239% -7.0302%] (p = 0.00 < 0.05)
                        thrpt:  [+7.5618% +9.4378% +11.231%]
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe
```
## ChaCha20Rng::fill_bytes()
```
ChaCha20Rng/fill_bytes/1024
                        time:   [4969.7265 cycles 5035.8023 cycles 5111.3781 cycles]
                        thrpt:  [4.9916 cpb 4.9178 cpb 4.8532 cpb]
                 change:
                        time:   [-9.2025% -7.8618% -6.4605%] (p = 0.00 < 0.05)
                        thrpt:  [+6.9067% +8.5326% +10.135%]
                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  11 (11.00%) high mild
  1 (1.00%) high severe
ChaCha20Rng/fill_bytes/2048
                        time:   [9834.7517 cycles 9912.4235 cycles 10000.8437 cycles]
                        thrpt:  [4.8832 cpb 4.8401 cpb 4.8021 cpb]
                 change:
                        time:   [-10.633% -9.6356% -8.6183%] (p = 0.00 < 0.05)
                        thrpt:  [+9.4312% +10.663% +11.898%]
                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe
ChaCha20Rng/fill_bytes/4096
                        time:   [19800.1388 cycles 20009.9842 cycles 20254.1604 cycles]
                        thrpt:  [4.9449 cpb 4.8853 cpb 4.8340 cpb]
                 change:
                        time:   [-10.580% -9.5670% -8.5319%] (p = 0.00 < 0.05)
                        thrpt:  [+9.3277% +10.579% +11.832%]
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) high mild
  5 (5.00%) high severe
ChaCha20Rng/fill_bytes/8192
                        time:   [39623.1319 cycles 39923.8427 cycles 40255.8394 cycles]
                        thrpt:  [4.9140 cpb 4.8735 cpb 4.8368 cpb]
                 change:
                        time:   [-10.571% -9.2816% -8.0656%] (p = 0.00 < 0.05)
                        thrpt:  [+8.7732% +10.231% +11.820%]
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  8 (8.00%) high mild
  1 (1.00%) high severe
ChaCha20Rng/fill_bytes/16384
                        time:   [79217.2501 cycles 79876.6622 cycles 80657.3595 cycles]
                        thrpt:  [4.9229 cpb 4.8753 cpb 4.8350 cpb]
                 change:
                        time:   [-11.160% -9.3209% -7.5531%] (p = 0.00 < 0.05)
                        thrpt:  [+8.1702% +10.279% +12.562%]
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe
```