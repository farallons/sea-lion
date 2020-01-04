use std::io;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::BufWriter;

const RESULT_FILENAME : &str = "result.prime";
const U32_SIZE : usize = 4294967296;

#[derive(Debug)]
pub struct Generator {
    n : u32,
    primes: Box<[u32]>,
}

impl Generator {
    pub fn new () -> Generator {
        Generator :: new_with_max_prime(u32 :: max_value())
        }

    pub fn new_with_max_prime (n: u32) -> Generator {
        Generator {
            n,
            primes: Box::new([]),
        }
    }

    pub fn generate(&mut self) -> () {
        let mut eliminated: Vec<bool> = vec![false; U32_SIZE];
        let mut primes : Vec<u32> = Vec::new();
        let mut i = 0;

        for number in 2..=self.n{
            if ! eliminated[number as usize] {
                primes.push(number);
                i += 1;
            }
            let mut last_prime = 0;
            let mut compound_number: u64 = 0;
            for j in 0..i {
                let current_prime = primes[j];
                let additions_to_do = current_prime - last_prime;
                last_prime = primes[j];
                for _ in 0..additions_to_do {
                    compound_number += number as u64;
                }
                if compound_number > self.n as u64 {
                    break;
                }
                eliminated[compound_number as usize] = true;
                if number % current_prime == 0 {
                    break;
                }
            }
        }
        self.primes = primes.into_boxed_slice();
    }

    pub fn primes(&self) -> Box<[u32]> {
        self.primes.clone()
    }
}

pub fn write_to_result_file(primes: Box<[u32]>) -> io::Result<()> {
    let mut path = Path::new("..").canonicalize()?;
    path.push(RESULT_FILENAME);

    let mut result_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)?;

    let mut writer = BufWriter::new(&mut result_file);
    for prime in primes.iter() {
        write!(writer, "{} ", prime)?;
    }
    writeln!(writer, " ")?;
    writer.flush()?;

    Ok(())
}


