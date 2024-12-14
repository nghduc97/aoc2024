use std::{i32, io};
use scanf::sscanf;

const STDDEV_THRESHOLD: f64 = 500.;

const X_MAX: i32 = 101;
const Y_MAX: i32 = 103;

struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

fn main() {
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

    for step in 1..10000 {
        let mut board = Vec::new();
        for _ in 0..Y_MAX {
            board.push(vec![0].repeat(X_MAX as usize));
        }

        let mut sum_x: f64 = 0.;
        let mut sum_y: f64 = 0.;
        for i in 0..robots.len() {
            let (mut px, mut py) = robots[i].position;
            let (vx, vy) = robots[i].velocity;
            px = ((px + vx) % X_MAX + X_MAX) % X_MAX;
            py = ((py + vy) % Y_MAX + Y_MAX) % Y_MAX;
            robots[i].position = (px, py);

            board[py as usize][px as usize] += 1;
            sum_x += px as f64;
            sum_y += py as f64;
        }

        let avg_x = sum_x / (robots.len() as f64);
        let avg_y = sum_y / (robots.len() as f64);
        let mut stddev_x = 0.;
        let mut stddev_y = 0.;
        for i in 0..robots.len() {
            let (px, py) = robots[i].position;
            stddev_x += ((px as f64) - avg_x).powi(2) / (robots.len() as f64);
            stddev_y += ((py as f64) - avg_y).powi(2) / (robots.len() as f64);
        }

        if stddev_x > STDDEV_THRESHOLD || stddev_y > STDDEV_THRESHOLD {
            continue;
        }

        println!("step {}:", step);
        for y in 0..Y_MAX {
            for x in 0..X_MAX {
                print!("{}", if board[y as usize][x as usize] > 0 { '*' } else { '.' });
            }
            println!("");
        }
        println!("");
    }
}
