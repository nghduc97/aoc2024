use std::{collections::{HashSet, VecDeque}, io};

use scanf::sscanf;

// const DIMENSION_SIZE: i32 = 6;
const DIMENSION_SIZE: i32 = 70;


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

fn check_if_passable_at(corrupted_pixels: &Vec<(i32, i32)>, idx: usize) -> bool {
    let mut pixel_set = HashSet::new();
    for i in 0..=idx {
        pixel_set.insert(corrupted_pixels[i]);
    }
    return get_shortest_distance(&pixel_set).is_some();
}

fn main() {
    let mut corrupted_pixels = Vec::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line = line.trim().to_string();
        if line.len() == 0 {
            break;
        }

        let mut x: i32 = 0;
        let mut y: i32 = 0;
        sscanf!(&line, "{},{}", x, y).unwrap();
        corrupted_pixels.push((x, y));
    }

    let mut l = 0;
    let mut r = corrupted_pixels.len() - 1;
    while l < r {
        let mid = l + (r - l) / 2;
        if !check_if_passable_at(&corrupted_pixels, mid) {
            r = mid;
        } else {
            l = mid + 1;
        }
    }

    let (rx, ry) = corrupted_pixels[l];
    println!("{},{}", rx, ry);
}
