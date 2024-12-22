use std::io;

const MODULO: u64 = 16777216;
const NUM_SECRETS: usize = 2000;

fn get_last_secret(seed: u64) -> u64 {
    let mut secret = seed;
    for _ in 0..NUM_SECRETS {
        let value = secret * 64;
        secret = (secret ^ value) % MODULO;
        let value = secret / 32;
        secret = (secret ^ value) % MODULO;
        let value = secret * 2048;
        secret = (secret ^ value) % MODULO;
    }

    return secret;
}

fn main() {
    let mut seeds = Vec::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line = line.trim().to_string();
        if line.len() == 0 {
            break;
        }
        seeds.push(line.parse::<u64>().unwrap());
    }

    let mut result = 0;
    for seed in seeds {
        result += get_last_secret(seed);
    }
    println!("{}", result);
}
