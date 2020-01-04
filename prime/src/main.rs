use prime::Generator;
use prime;
use std::env;

#[allow(unused_variables)]
fn main() {
    let args: Vec<String> = env::args().collect();
    let args_count = args.len();
    let mut generator = match args.len() {
        1 => Generator::new(),
        2 => {
            let n = match u32::from_str_radix(&args[1], 10) {
                Ok(n) => {
                    n
                },
                Err(e) => {
                    panic!("{:?}", e);
                },
            };
            Generator::new_with_max_prime(n)

        },
        _ => panic!("Usage: cargo run <n>")
    };
    generator.generate();
    let primes = generator.primes();

    match prime::write_to_result_file(primes) {
        Ok(()) => println!("completed"),
        Err(err) => println!("failure: {:?}", err),
    }

}
