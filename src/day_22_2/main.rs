use std::{collections::HashMap, i32, io};

const MODULO: u64 = 16777216;
const NUM_SECRETS: usize = 2000;

fn price_change_sequence_hash(c1: i32, c2: i32, c3: i32, c4: i32) -> i32 {
    let mut value = 0;
    value = value * 20 + (c1 + 10);
    value = value * 20 + (c2 + 10);
    value = value * 20 + (c3 + 10);
    value = value * 20 + (c4 + 10);
    return value;
}

fn get_price_map(seed: u64) -> HashMap<i32, i32> {
    let mut price_map = HashMap::new();
    let mut c1;
    let (mut c2, mut c3, mut c4) = (0, 0, 0);
    let mut secret = seed;
    for i in 1..=NUM_SECRETS {
        let mut new_secret = secret;
        let value = new_secret * 64;
        new_secret = (new_secret ^ value) % MODULO;
        let value = new_secret / 32;
        new_secret = (new_secret ^ value) % MODULO;
        let value = new_secret * 2048;
        new_secret = (new_secret ^ value) % MODULO;

        let price = (secret % 10) as i32;
        let new_price = (new_secret % 10) as i32;
        let price_change = new_price - price;
        (c1, c2, c3, c4) = (c2, c3, c4, price_change);

        let key = price_change_sequence_hash(c1, c2, c3, c4);
        if i >= 4 && !price_map.contains_key(&key) {
            price_map.insert(key, new_price);
        }
        secret = new_secret;
    }

    return price_map;
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

    let mut total_price_map = HashMap::new();
    let mut result = 0;
    for seed in seeds {
        let price_map = get_price_map(seed);
        for (key, price) in price_map {
            match total_price_map.get_mut(&key) {
                Some(sum) => {
                    *sum += price;
                    result = result.max(*sum);
                },
                None => {
                    total_price_map.insert(key, price);
                },
            }
        }
    }
    println!("{}", result);
}
