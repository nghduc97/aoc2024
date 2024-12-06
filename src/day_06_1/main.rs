use std::io;

fn main() {
    let d_up = (-1, 0);
    let d_right = (0, 1);
    let d_down = (1, 0);
    let d_left = (0, -1);

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

    let mut result = 0;
    loop {
        let (x, y) = current_position;
        if grid[x as usize][y as usize] == '.' || grid[x as usize][y as usize] == '^' {
            grid[x as usize][y as usize] = 'X';
            result += 1;
        }

        let (dx, dy) = current_direction;
        let (nx, ny) = (x + dx, y + dy);
        if nx < 0 || ny < 0 || nx >= n_row as i32 || ny >= n_col as i32 {
            break;
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

    println!("{}", result);
}
