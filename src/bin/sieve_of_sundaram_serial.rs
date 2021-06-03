#[macro_use]
extern crate log;

use humantime::format_duration;
use itertools::Itertools;
use std::error;
use std::time;

fn main() -> Result<(), Box<dyn error::Error>> {
    let start = time::Instant::now();
    env_logger::init();

    let ceiling = 10;
    let mut prime_data: Vec<bool> = vec![true; ceiling];
    let n = ceiling / 2;
    let mut x = 1;
    while x < n {
        let mut y = x.clone();
        let num = n - 1;
        let den = 2 * x + 1;
        while y <= num / den {
            let idx = x + y + 2 * x * y;
            info!("x: {}, y: {}, num: {}, den: {}, idx: {}", x, y, num, den, idx);
            prime_data[idx] = false;
            y = y + 1;
        }
        x = x + 1;
    }

    // for (int i = 1; i < n; i++) {
    //     for (int j = i; j <= (n - i) / (2 * i + 1); j++) {
    //         primeArray[i + j + 2 * i * j] = false;
    //     }
    // }

    let mut found_primes = std::collections::BTreeSet::new();
    prime_data.into_iter().enumerate().for_each(|(idx, a)| {
        if a {
            found_primes.insert(idx);
        }
    });
    info!("primes: {}", found_primes.iter().join(","));
    info!("Duration: {}", format_duration(start.elapsed()).to_string());
    Ok(())
}
