use std::{collections::VecDeque, io};

const CHEAT_THRESHOLD: i32 = 100;

fn get_shortest_path(grid: &Vec<Vec<char>>, sx: usize, sy: usize) -> i32 {
    let mut visited = Vec::new();
    for _ in 0..grid.len() {
        visited.push(vec![false].repeat(grid[0].len()));
    }

    let mut queue = VecDeque::new();
    queue.push_back((0, sx, sy));
    visited[sx][sy] = true;

    while queue.len() > 0 {
        let (dist, x, y) = queue.pop_front().unwrap();
        if grid[x][y] == 'E' {
            return dist;
        }

        for (nx, ny) in [
            (x, y + 1),
            (x + 1, y),
            (x, y - 1),
            (x - 1, y),
        ] {
            if grid[nx][ny] != '#' && !visited[nx][ny] {
                queue.push_back((dist + 1, nx, ny));
                visited[nx][ny] = true;
            }
        }
    }

    panic!("cannot reach endpoint");
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
    'find_starting_point: for x in 0..n_row {
        for y in 0..n_col {
            if grid[x][y] == 'S' {
                (sx, sy) = (x, y);
                break 'find_starting_point;
            }
        }
    }

    let initial_dist = get_shortest_path(&grid, sx, sy);
    let mut result = 0;
    for x in 1..n_row - 1 {
        for y in 1..n_col - 1 {
            if grid[x][y] == '#' {
                grid[x][y] = '.';
                let new_dist = get_shortest_path(&grid, sx, sy);
                if initial_dist - new_dist >= CHEAT_THRESHOLD {
                    result += 1;
                }
                grid[x][y] = '#';
            }
        }
    }

    println!("{}", result);
}
