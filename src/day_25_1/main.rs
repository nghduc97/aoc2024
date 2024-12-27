use std::io;

fn encode_grid(grid: &Vec<Vec<char>>) -> (bool, Vec<usize>) {
    let n_row = grid.len();
    let n_col = grid[0].len();
    let is_lock = grid[0][0] == '#';
    let mut code = Vec::new();
    'col_loop: for y in 0..n_col {
        if is_lock {
            for x in 0..n_row {
                if grid[x][y] != '#' {
                    code.push(x);
                    continue 'col_loop;
                }
            }
        } else {
            for x in (0..n_row).rev() {
                if grid[x][y] != '#' {
                    code.push(n_row - x - 1);
                    continue 'col_loop;
                }
            }
        }
        panic!("code not recognized");
    }

    return (is_lock, code);
}

fn main() {
    let mut n_row = 0;
    let mut n_col = 0;
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    loop {
        let mut grid = Vec::new();
        loop {
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();
            line = line.trim().to_string();
            if line.len() == 0 {
                break;
            }
            grid.push(line.chars().collect::<Vec<char>>());
        }
        if grid.len() == 0 {
            break;
        }

        let (is_lock, code) = encode_grid(&grid);
        if is_lock {
            locks.push(code);
        } else {
            keys.push(code);
        }

        n_row = grid.len();
        n_col = grid[0].len();
    }

    let mut result = 0;
    for lock in locks.iter() {
        'key_loop: for key in keys.iter() {
            for i in 0..n_col {
                if lock[i] + key[i] > n_row {
                    continue 'key_loop;
                }
            }
            result += 1;
        }
    }
    println!("{}", result);
}