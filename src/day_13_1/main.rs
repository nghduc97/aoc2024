use std::{i32, io};
use scanf::sscanf;

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

        let (mut ax, mut ay, mut bx, mut by, mut cx, mut cy) = (0, 0, 0, 0, 0, 0);
        sscanf!(&line_a, "Button A: X+{}, Y+{}", ax, ay).unwrap();
        sscanf!(&line_b, "Button B: X+{}, Y+{}", bx, by).unwrap();
        sscanf!(&line_c, "Prize: X={}, Y={}", cx, cy).unwrap();

        let mut best_cost = i32::MAX;
        for a in 0..100 {
            for b in 0..100 {
                if 3 * a + b < best_cost && ax * a + bx * b == cx && ay * a + by * b == cy {
                    best_cost = 3 * a + b;
                }
            }
        }
        if best_cost != i32::MAX {
            result += best_cost;
        }
    }

    println!("{}", result);
}
