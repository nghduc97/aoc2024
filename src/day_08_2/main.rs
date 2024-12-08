use std::{collections::HashMap, io};
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

    let mut mark_grid = vec![".".repeat(n_col).chars().collect::<Vec<char>>()];
    for _ in 1..n_row {
        mark_grid.push(mark_grid[0].clone());
    }

    let mut pos_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for x in 0..n_row {
        for y in 0..n_col {
            if grid[x][y] != '.' {
                match pos_map.get_mut(&grid[x][y]) {
                    Some(positions) => {
                        for (px, py) in positions.iter() {
                            let dx = (*px as i32) - (x as i32);
                            let dy = (*py as i32) - (y as i32);

                            let (mut nx, mut ny) = (x as i32, y as i32);
                            loop {
                                if nx < 0 || ny < 0 || nx >= (n_row as i32) || ny >= (n_col as i32) {
                                    break;
                                }
                                mark_grid[nx as usize][ny as usize] = '#';
                                nx -= dx;
                                ny -= dy;
                            }

                            let (mut nx, mut ny) = (*px as i32, *py as i32);
                            loop {
                                if nx < 0 || ny < 0 || nx >= (n_row as i32) || ny >= (n_col as i32) {
                                    break;
                                }
                                mark_grid[nx as usize][ny as usize] = '#';
                                nx += dx;
                                ny += dy;
                            }
                        }
                        positions.push((x, y));
                    },
                    None => {
                        pos_map.insert(grid[x][y], vec![(x, y)]);
                    },
                }
            }
        }
    }

    let mut result = 0;
    for x in 0..n_row {
        for y in 0..n_col {
            if mark_grid[x][y] == '#' {
                result += 1;
            }
        }
    }
    println!("{}", result);
}