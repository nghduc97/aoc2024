use std::{collections::VecDeque, io};

const CHEAT_DURATION: usize = 20;
const CHEAT_THRESHOLD: i32 = 100;

fn get_shortest_path(grid: &Vec<Vec<char>>, sx: usize, sy: usize) -> Vec<Vec<i32>> {
    let mut dist_grid = Vec::new();
    for _ in 0..grid.len() {
        dist_grid.push(vec![-1].repeat(grid[0].len()));
    }

    let mut queue = VecDeque::new();
    queue.push_back((sx, sy));
    dist_grid[sx][sy] = 0;

    while queue.len() > 0 {
        let (x, y) = queue.pop_front().unwrap();

        for (nx, ny) in [
            (x, y + 1),
            (x + 1, y),
            (x, y - 1),
            (x - 1, y),
        ] {
            if grid[nx][ny] != '#' && dist_grid[nx][ny] < 0 {
                dist_grid[nx][ny] = dist_grid[x][y] + 1;
                queue.push_back((nx, ny));
            }
        }
    }

    return dist_grid;
}

fn main() {
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
    let mut sx = 0;
    let mut sy = 0;
    let mut ex = 0;
    let mut ey = 0;
    for x in 0..n_row {
        for y in 0..n_col {
            if grid[x][y] == 'S' {
                (sx, sy) = (x, y);
            }
            if grid[x][y] == 'E' {
                (ex, ey) = (x, y);
            }
        }
    }

    let s_dist_grid = get_shortest_path(&grid, sx, sy);
    let e_dist_grid = get_shortest_path(&grid, ex, ey);
    let initial_dist = s_dist_grid[ex][ey];
    let mut result = 0;
    for x1 in 0..n_row {
        for y1 in 0..n_col {
            if s_dist_grid[x1][y1] >= 0 {
                for x2 in 0..n_row {
                    for y2 in 0..n_col {
                        let n_skip_step = x1.abs_diff(x2) + y1.abs_diff(y2);
                        if n_skip_step > CHEAT_DURATION {
                            continue;
                        }
                        if e_dist_grid[x2][y2] < 0 {
                            continue;
                        }
                        let new_dist = s_dist_grid[x1][y1]
                            + (n_skip_step as i32)
                            + e_dist_grid[x2][y2];
                        if initial_dist - new_dist >= CHEAT_THRESHOLD {
                            result += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", result);
}
