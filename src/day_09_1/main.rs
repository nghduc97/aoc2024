use std::io::{self, Read};
fn main() {
    let mut line = String::new();
    io::stdin().read_to_string(&mut line).unwrap();
    let mut space: Vec<u32> = line.trim().chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let mut result = 0i64;
    let mut j = if space.len() % 2 == 0 { space.len() - 2 } else { space.len() - 1 };
    let mut pos: usize = 0;
    for i in 0..space.len() {
        if i % 2 == 0 {
            for _ in 0..space[i] {
                result += (i / 2 * pos) as i64;
                pos += 1;
            }
        } else {
            for _ in 0..space[i] {
                while i < j && space[j] == 0 {
                    j -= 2;
                }
                if i < j {
                    result += (j / 2 * pos) as i64;
                    space[j] -= 1;
                }
                pos += 1;
            }
        }
    }

    println!("{}", result);
}
