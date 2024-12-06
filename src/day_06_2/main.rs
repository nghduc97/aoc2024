use std::{collections::HashSet, io, sync::mpsc::channel};

use threadpool::ThreadPool;

fn clone_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid = Vec::new();
    new_grid.reserve(grid.len());
    for row in grid {
        new_grid.push(row.clone());
    }
    return new_grid;
}

fn patrol_in_loop(grid: Vec<Vec<char>>) -> bool {
    let d_up = (-1, 0);
    let d_right = (0, 1);
    let d_down = (1, 0);
    let d_left = (0, -1);

    let n_row = grid.len();
    let n_col = grid[0].len();

    let mut current_position = (0, 0);
    let mut current_direction = d_up;
    'start_position_search: for x in 0..n_row {
        for y in 0..n_col {
            if grid[x][y] == '^' {
                current_position = (x as i32, y as i32);
                break 'start_position_search;
            }
        }
    }

    let mut seen_position_set = HashSet::new();
    let mut step = 0;
    loop {
        assert!(step <= 100000);
        step += 1;

        let (x, y) = current_position;
        let (dx, dy) = current_direction;
        if seen_position_set.contains(&(x, y, dx, dy)) {
            return true;
        }
        seen_position_set.insert((x, y, dx, dy));

        let (nx, ny) = (x + dx, y + dy);
        if nx < 0 || ny < 0 || nx >= n_row as i32 || ny >= n_col as i32 {
            return false;
        }

        if grid[nx as usize][ny as usize] == '#' {
            if current_direction == d_up {
                current_direction = d_right;
            } else if current_direction == d_right {
                current_direction = d_down;
            } else if current_direction == d_down {
                current_direction = d_left;
            } else if current_direction == d_left {
                current_direction = d_up;
            } else {
                panic!("unknown direction");
            }
        } else {
            current_position = (nx, ny);
        }
    }
}

fn main() {
    let thread_pool = ThreadPool::new(8);
    let (tx, rx) = channel();

    let mut grid: Vec<Vec<char>> = Vec::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line = line.trim().to_string();
        if line.len() == 0 {
            break;
        }
        grid.push(line.chars().collect());
    }

    let n_row = grid.len();
    let n_col = grid[0].len();

    for x in 0..n_row {
        for y in 0..n_col {
            if grid[x][y] == '.' {
                let mut new_grid = clone_grid(&grid);
                let tx = tx.clone();
                thread_pool.execute(move|| {
                    new_grid[x][y] = '#';
                    if patrol_in_loop(new_grid) {
                        tx.send(1).unwrap();
                    }
                });
            }
        }
    }
    thread_pool.join();
    drop(tx);

    let mut result = 0;
    for value in rx {
        result += value;
    }
    println!("{}", result);
}
