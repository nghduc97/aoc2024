use std::io::{self, Read};
use regex::Regex;

fn main() {
    let mut content = String::new();
    _ = io::stdin().read_to_string(&mut content);

    let pat = Regex::new(r"(do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\))").unwrap();
    let mut result = 0;
    let mut mul_enabled = true;

    for cap in pat.captures_iter(&content) {
        let cmd = cap.get(0).unwrap().as_str();
        if cmd == "do()" {
            mul_enabled = true;
        } else if cmd == "don't()" {
            mul_enabled = false;
        } else if mul_enabled {
            let v1 = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let v2 = cap.get(3).unwrap().as_str().parse::<i32>().unwrap();
            result += v1 * v2;
        }
    }
    println!("{}", result);
}
