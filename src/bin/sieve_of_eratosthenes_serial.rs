#[macro_use]
extern crate log;

use humantime::format_duration;
use itertools::Itertools;
use std::error;
use std::time;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "sieve_of_eratosthenes_serial", about = "sieve of Eratosthenes serial")]
struct Options {
    #[structopt(short = "c", long = "ceiling", long_help = "ceiling", default_value = "100000000")]
    ceiling: usize,
}
fn main() -> Result<(), Box<dyn error::Error>> {
    let start = time::Instant::now();
    env_logger::init();

    let options = Options::from_args();
    info!("{:?}", options);

    let ceiling = options.ceiling;
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

    // let mut found_primes = prime_data.into_iter().enumerate().filter(|(_, a)| *a).map(|(idx, _)| idx).collect_vec();
    // found_primes.sort();
    // debug!("ceiling: {}, primes: {}", ceiling, found_primes.iter().join(","));

    info!("Duration: {}", format_duration(start.elapsed()).to_string());
    Ok(())
}
