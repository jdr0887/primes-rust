#[macro_use]
extern crate log;

use humantime::format_duration;
use itertools::Itertools;
use std::error;
use std::time;

fn main() -> Result<(), Box<dyn error::Error>> {
    let start = time::Instant::now();
    env_logger::init();

    let mut ceiling = 1_000_000_000;
    let mut prime_data = vec![true; ceiling + 1];

    let mut x = 2;
    while x * x <= ceiling {
        if prime_data[x] {
            let mut y = x.clone();
            while x * y <= ceiling {
                prime_data[x * y] = false;
                y = y + 1;
            }
        }
        x = x + 1;
    }

    let mut found_primes = std::collections::BTreeSet::new();
    prime_data.into_iter().enumerate().for_each(|(idx, a)| {
        if a {
            found_primes.insert(idx);
        }
    });
    debug!("ceiling: {}, primes: {}", ceiling, found_primes.iter().join(","));

    info!("Duration: {}", format_duration(start.elapsed()).to_string());
    Ok(())
}
