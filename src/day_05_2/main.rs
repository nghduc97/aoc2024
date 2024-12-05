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

    let mut result: i32 = 0;
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line = line.trim().to_string();
        if line.len() == 0 {
            break;
        }

        let nums: Vec<i32> = line.split(',').map(|tk| tk.parse::<i32>().unwrap()).collect();
        assert!(nums.len() % 2 == 1);

        // correction
        let mut ordered_nums = nums.clone();
        let mut corrected = false;
        'correction: loop {
            for i in 0..ordered_nums.len() {
                for j in i + 1..ordered_nums.len() {
                    if order_set.contains(&(ordered_nums[j], ordered_nums[i])) {
                        (ordered_nums[i], ordered_nums[j]) = (ordered_nums[j], ordered_nums[i]);
                        corrected = true;
                        continue 'correction;
                    }
                }
            }
            break;
        }

        if corrected {
            result += ordered_nums[ordered_nums.len() / 2];
        }
    }

    println!("{}", result);
}
