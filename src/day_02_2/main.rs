use std::io;

fn report_is_correct(report: Vec<i32>) -> bool {
    if report.len() < 2 {
        return true;
    }
    let upward = report[0] < report[1];

    for i in 0..report.len() - 1 {
        let mut max_lv = report[i] - 1;
        let mut min_lv = report[i] - 3;
        if upward {
            max_lv = report[i] + 3;
            min_lv = report[i] + 1;
        }

        if min_lv > report[i + 1] || report[i + 1] > max_lv {
            return false;
        }
    }

    return true;
}

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
        for i in 0..report.len() {
            let mut report_without_i = report.clone();
            report_without_i.remove(i);

            if report_is_correct(report_without_i) {
                result += 1;
                break;
            }
        }
    }
    println!("{}", result);
}
