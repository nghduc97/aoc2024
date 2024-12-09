use std::io::{self, Read};
fn main() {
    let mut line = String::new();
    io::stdin().read_to_string(&mut line).unwrap();
    let mut space: Vec<u32> = line.trim().chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let mut acc_pos = vec![space[0]];
    for i in 1..space.len() {
        acc_pos.push(acc_pos[i - 1] + space[i]);
    }

    let mut result: i64 = 0;
    for i in (0..space.len()).step_by(2).rev() {
        for k in (1..i).step_by(2) {
            if space[k] >= space[i] {
                for idx in 0..space[i] {
                    result += (i as u32 / 2 * (acc_pos[k - 1] + idx)) as i64;
                }
                acc_pos[k - 1] += space[i];
                space[k] -= space[i];
                space[i] = 0;
                break;
            }
        }
    }
    for i in (0..space.len()).step_by(2) {
        let st = if i > 0 { acc_pos[i - 1] } else { 0 };
        for idx in 0..space[i] {
            result += (i as u32 / 2 * (st + idx)) as i64;
        }
    }
    println!("{}", result);
}
