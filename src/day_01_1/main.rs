use std::io;
use scanf::sscanf;

fn main() {
    let mut vec_a: Vec<i32> = Vec::new();
    let mut vec_b: Vec<i32> = Vec::new();
    for line in io::stdin().lines() {
        let mut a = 0;
        let mut b = 0;
        if sscanf!(line.unwrap().as_str(), "{} {}", a, b).is_ok() {
            vec_a.push(a);
            vec_b.push(b);
        }
    }

    vec_a.sort();
    vec_b.sort();
    let mut result = 0;
    for i in 0..vec_a.len() {
        result += vec_a[i].abs_diff(vec_b[i]);
    }
    println!("{}", result);
}
