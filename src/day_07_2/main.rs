use std::{io, sync::mpsc::channel};

use threadpool::ThreadPool;

struct Equation {
    result: i64,
    nums: Vec<i64>,
}

fn concat_nums(a: i64, b: i64) -> i64 {
    let mut mul = 10;
    while mul <= b {
        mul *= 10;
    }
    return a * mul + b;
}

fn _equation_solvable(e: &Equation, step: usize, sum: i64) -> bool {
    if sum > e.result {
        return false;
    }
    if step == e.nums.len() {
        return sum == e.result;
    }
    return _equation_solvable(e, step + 1, sum + e.nums[step])
        || _equation_solvable(e, step + 1, sum * e.nums[step])
        || _equation_solvable(e, step + 1, concat_nums(sum, e.nums[step]));
}

fn equation_solvable(e: &Equation) -> bool {
    return _equation_solvable(e, 1, e.nums[0]);
}

fn main() {
    let mut equations: Vec<Equation> = Vec::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line = line.trim().to_string();
        if line.len() == 0 {
            break;
        }

        let parts: Vec<&str> = line.split(":").collect();
        let result = parts[0].parse::<i64>().unwrap();
        let nums: Vec<i64> = parts[1].trim().split(" ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        equations.push(Equation{
            result,
            nums,
        });
    }

    let thread_pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();
    for equation in equations {
        let tx = tx.clone();
        thread_pool.execute(move || {
            if equation_solvable(&equation) {
                tx.send(equation.result).unwrap();
            }
        });
    }
    thread_pool.join();
    drop(tx);

    let mut result: i64 = 0;
    for value in rx {
        result += value;
    }
    println!("{}", result);
}
