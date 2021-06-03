# primes-rust

## How to Build & Run
```
primes-rust$ cargo build --release
primes-rust$ RUST_LOG=info ./target/release/sieve_of_atkin_serial -c 1000000000
[2021-06-03T15:43:19Z INFO  sieve_of_atkin_serial] Options { ceiling: 1000000000 }
[2021-06-03T15:43:33Z INFO  sieve_of_atkin_serial] Duration: 14s 98ms 326us 654ns
primes-rust$ RUST_LOG=info ./target/release/sieve_of_eratosthenes_serial -c 1000000000
[2021-06-03T15:43:36Z INFO  sieve_of_eratosthenes_serial] Options { ceiling: 1000000000 }
[2021-06-03T15:43:47Z INFO  sieve_of_eratosthenes_serial] Duration: 10s 359ms 538us 896ns
primes-rust$ RUST_LOG=info ./target/release/sieve_of_sundaram_serial -c 1000000000
[2021-06-03T15:43:49Z INFO  sieve_of_sundaram_serial] Options { ceiling: 1000000000 }
[2021-06-03T15:44:05Z INFO  sieve_of_sundaram_serial] Duration: 15s 858ms 50us 461ns
```