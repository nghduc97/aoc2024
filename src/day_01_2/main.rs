use std::{collections::HashMap, io};
use scanf::sscanf;

fn main() {
    let mut vec_a: Vec<i32> = Vec::new();
    let mut map_b: HashMap<i32, i32> = HashMap::new();
    for line in io::stdin().lines() {
        let mut a = 0;
        let mut b = 0;
        if sscanf!(line.unwrap().as_str(), "{} {}", a, b).is_ok() {
            vec_a.push(a);
            let cnt = map_b.entry(b).or_insert(0);
            *cnt += 1;
        }
    }

    let mut result = 0;
    let zero = 0i32;
    for i in 0..vec_a.len() {
        result += vec_a[i] * map_b.get(&vec_a[i]).unwrap_or(&zero);
    }
    println!("{}", result);
}
