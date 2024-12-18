use std::{collections::{HashSet, VecDeque}, io};

use scanf::sscanf;

// const DIMENSION_SIZE: i32 = 6;
// const BYTES_BEFORE_ESCAPE: usize = 12;

const DIMENSION_SIZE: i32 = 70;
const BYTES_BEFORE_ESCAPE: usize = 1024;

fn get_shortest_distance(corrupted_pixels: &HashSet<(i32, i32)>) -> Option<i32> {
    let mut visited_pixels: HashSet<(i32, i32)> = HashSet::new();
    let mut queue = VecDeque::new();

    visited_pixels.insert((0, 0));
    queue.push_back((0, 0, 0));

    while queue.len() > 0 {
        let (dist, x, y) = queue.pop_front().unwrap();
        if x == DIMENSION_SIZE && y == DIMENSION_SIZE {
            return Some(dist);
        }

        for (nx, ny) in [
            (x, y + 1),
            (x + 1, y),
            (x, y - 1),
            (x - 1, y),
        ] {
            if nx < 0 || ny < 0 || nx > DIMENSION_SIZE || ny > DIMENSION_SIZE {
                continue;
            }
            if !visited_pixels.contains(&(nx, ny)) && !corrupted_pixels.contains(&(nx, ny)) {
                queue.push_back((dist + 1, nx, ny));
                visited_pixels.insert((nx, ny));
            }
        }
    }

    return None;
}

fn main() {
    let mut corrupted_pixels = HashSet::new();
    for _ in 0..BYTES_BEFORE_ESCAPE {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        let mut x: i32 = 0;
        let mut y: i32 = 0;
        sscanf!(&line, "{},{}", x, y).unwrap();
        corrupted_pixels.insert((x, y));
    }

    println!("{}", get_shortest_distance(&corrupted_pixels).unwrap());
}
