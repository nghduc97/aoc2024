use std::{collections::HashMap, io};

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let stones: Vec<i64> = line.trim().to_string()
        .split(' ')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut stone_map: HashMap<i64, i64> = HashMap::new();
    for i in 0..stones.len() {
        let cnt = stone_map.entry(stones[i]).or_insert(0);
        *cnt += 1;
    }

    for _ in 0..75 {
        let mut new_stone_map = HashMap::new();
        for (num, cnt) in stone_map.iter() {
            if *num == 0 {
                *new_stone_map.entry(1).or_insert(0) += cnt;
            } else {
                let num_str = num.to_string();
                if num_str.len() % 2 == 0 {
                    let v1 = num_str[0..num_str.len() / 2].parse::<i64>().unwrap();
                    let v2 = num_str[num_str.len() / 2..num_str.len()].parse::<i64>().unwrap();
                    *new_stone_map.entry(v1).or_insert(0) += cnt;
                    *new_stone_map.entry(v2).or_insert(0) += cnt;
                } else {
                    *new_stone_map.entry(num * 2024).or_insert(0) += cnt;
                }
            }
        }

        stone_map = new_stone_map;
    }

    let mut result = 0;
    for (_, cnt) in stone_map {
        result += cnt;
    }
    println!("{}", result);
}
