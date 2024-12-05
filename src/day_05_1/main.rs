use std::{collections::HashSet, io};

fn main() {
    let mut order_set = HashSet::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line = line.trim().to_string();
        if line.len() == 0 {
            break;
        }

        let nums: Vec<i32> = line.split('|').map(|tk| tk.parse::<i32>().unwrap()).collect();
        assert!(nums.len() == 2);
        order_set.insert((nums[0], nums[1]));
    }

    let mut result = 0i32;
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line = line.trim().to_string();
        if line.len() == 0 {
            break;
        }

        let nums: Vec<i32> = line.split(',').map(|tk| tk.parse::<i32>().unwrap()).collect();
        assert!(nums.len() % 2 == 1);

        let mut correct = true;
        'outer: for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if order_set.contains(&(nums[j], nums[i])) {
                    correct = false;
                    break 'outer;
                }
            }
        }

        if correct {
            result += nums[nums.len() / 2];
        }
    }

    println!("{}", result);
}
