#[macro_use]
extern crate log;

use humantime::format_duration;
use itertools::Itertools;
use std::error;
use std::time;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "sieve_of_sundaram_serial", about = "sieve of Sundaram serial")]
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
    let mut prime_data: Vec<bool> = vec![false; ceiling + 1];
    let n = (ceiling - 1) / 2;
    let mut x = 1;
    while x <= n {
        let mut y = x.clone();
        while (x + y + 2 * x * y) <= n {
            let idx = x + y + 2 * x * y;
            prime_data[idx] = true;
            y = y + 1;
        }
        x = x + 1;
    }

    // let mut found_primes = prime_data.into_iter().enumerate().filter(|(_, a)| *a).map(|(idx, _)| idx).collect_vec();
    // found_primes.sort();
    // debug!("ceiling: {}, primes: {}", ceiling, found_primes.iter().join(","));
    info!("Duration: {}", format_duration(start.elapsed()).to_string());
    Ok(())
}
