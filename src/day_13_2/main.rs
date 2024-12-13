use std::{i64, io};
use scanf::sscanf;

fn get_best_cost(xa: i64, ya: i64, xb: i64, yb: i64, xc: i64, yc: i64) -> Option<i64> {
    let xc = xc + 10000000000000;
    let yc = yc + 10000000000000;

    let mut pa = yc * xb - xc * yb;
    let mut qa = ya * xb - xa * yb;
    if qa == 0 {
        return None;
    }
    if qa < 0 {
        (pa, qa) = (-pa, -qa);
    }
    if pa < 0 || pa % qa != 0 {
        return None;
    }

    let a = pa / qa;
    if (xc - a * xa) % xb != 0 || xc < a * xa {
        return None;
    }
    let b = (xc - a * xa) / xb;

    return Some(a * 3 + b);
}

fn main() {
    let mut result = 0;
    loop {
        let mut line_a = String::new();
        let mut line_b = String::new();
        let mut line_c = String::new();
        let mut blank_line = String::new();
        io::stdin().read_line(&mut line_a).unwrap();
        if line_a.trim().len() == 0 {
            break;
        }
        io::stdin().read_line(&mut line_b).unwrap();
        io::stdin().read_line(&mut line_c).unwrap();
        io::stdin().read_line(&mut blank_line).unwrap();

        let (mut xa, mut ya, mut xb, mut yb, mut xc, mut yc) = (0, 0, 0, 0, 0, 0);
        sscanf!(&line_a, "Button A: X+{}, Y+{}", xa, ya).unwrap();
        sscanf!(&line_b, "Button B: X+{}, Y+{}", xb, yb).unwrap();
        sscanf!(&line_c, "Prize: X={}, Y={}", xc, yc).unwrap();

        match get_best_cost(xa, ya, xb, yb, xc, yc) {
            Some(cost) => result += cost,
            None => {},
        }
    }

    println!("{}", result);
}
