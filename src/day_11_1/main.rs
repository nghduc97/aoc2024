use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut stones: Vec<i64> = line.trim().to_string()
        .split(' ')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut new_stones = Vec::new();
    for _ in 0..25 {
        for i in 0..stones.len() {
            let stone = stones[i];
            if stone == 0 {
                new_stones.push(1);
            } else {
                let stone_str = stone.to_string();
                if stone_str.len() % 2 == 0 {
                    let left = &stone_str[0..stone_str.len() / 2];
                    let right = &stone_str[stone_str.len() / 2..stone_str.len()];
                    new_stones.push(left.parse::<i64>().unwrap());
                    new_stones.push(right.parse::<i64>().unwrap());
                } else {
                    new_stones.push(stone * 2024);
                }
            }
        }

        (stones, new_stones) = (new_stones, stones);
        new_stones.clear();
    }

    println!("{}", stones.len());
}
