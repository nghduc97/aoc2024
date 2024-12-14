use std::{i32, io};
use scanf::sscanf;

// how many seconds to simulate
const N_SECONDS: i32 = 100;

// for test example
// const X_MAX: i32 = 11;
// const Y_MAX: i32 = 7;

// for real input
const X_MAX: i32 = 101;
const Y_MAX: i32 = 103;

struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

fn main() {
    assert!(X_MAX % 2 == 1);
    assert!(Y_MAX % 2 == 1);

    let mut robots = Vec::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line = line.trim().to_string();
        if line.len() == 0 {
            break;
        }

        let (mut px, mut py, mut vx, mut vy) = (0, 0, 0, 0);
        sscanf!(&line, "p={},{} v={},{}", px, py, vx, vy).unwrap();
        robots.push(Robot{
            position: (px, py),
            velocity: (vx, vy),
        });
    }

    let quadrant_x = X_MAX / 2;
    let quadrant_y = Y_MAX / 2;
    let mut quadrant_robot_cnt = [0, 0, 0, 0];
    for i in 0..robots.len() {
        let (mut px, mut py) = robots[i].position;
        let (vx, vy) = robots[i].velocity;
        px = ((px + vx * N_SECONDS) % X_MAX + X_MAX) % X_MAX;
        py = ((py + vy * N_SECONDS) % Y_MAX + Y_MAX) % Y_MAX;

        if px != quadrant_x && py != quadrant_y {
            let mut q_idx = 0;
            if px > quadrant_x { q_idx += 1 }
            if py > quadrant_y { q_idx += 2 }
            quadrant_robot_cnt[q_idx] += 1;
        }
    }

    let mut result: i64 = 1;
    for cnt in quadrant_robot_cnt {
        result *= cnt as i64;
    }
    println!("{}", result);
}
