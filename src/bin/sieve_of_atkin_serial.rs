#[macro_use]
extern crate log;

use humantime::format_duration;
use itertools::Itertools;
use std::error;
use std::time;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "sieve_of_atkin_serial", about = "sieve of Atkin serial")]
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
    let mut prime_data = vec![false; ceiling + 1];

    prime_data[0] = false;
    prime_data[1] = false;
    prime_data[2] = true;
    prime_data[3] = true;

    let mut x = 1;
    while x * x < ceiling {
        let mut y = 1;
        while y * y < ceiling {
            let n = (4 * x * x) + (y * y);
            if n <= ceiling && (n % 12 == 1 || n % 12 == 5) {
                prime_data[n] ^= true;
            }
            let n = (3 * x * x) + (y * y);
            if n <= ceiling && (n % 12 == 7) {
                prime_data[n] ^= true;
            }
            let n = (3 * x * x) - (y * y);
            if x > y && n <= ceiling && (n % 12 == 11) {
                prime_data[n] ^= true;
            }
            y = y + 1;
        }
        x = x + 1;
    }

    let mut x = 5;
    while x * x < ceiling {
        if prime_data[x] {
            let mut y = x * x;
            while y <= ceiling {
                prime_data[y] = false;
                y = y + x * x;
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
