use std::io;

fn main() {
    let mut reports = Vec::new();
    for line in io::stdin().lines() {
        let mut report: Vec<i32> = Vec::new();
        for token in line.unwrap().split(' ') {
            let level = token.parse::<i32>().unwrap();
            report.push(level);
        }
        reports.push(report);
    }

    let mut result = 0;
    for report in reports {
        let upward = report[0] < report[4];
        let mut correct = true;
        for i in 0..report.len() - 1 {
            let mut max_lv = report[i] - 1;
            let mut min_lv = report[i] - 3;
            if upward {
                max_lv = report[i] + 3;
                min_lv = report[i] + 1;
            }

            if min_lv > report[i + 1] || report[i + 1] > max_lv {
                correct = false;
                break;
            }
        }

        if correct {
            result += 1;
        }
    }
    println!("{}", result);
}
