use std::io;

fn combine_chars(c1: char, c2: char, c3: char, c4: char) -> String {
    let mut s = String::new();
    s.push(c1);
    s.push(c2);
    s.push(c3);
    s.push(c4);
    s
}

fn main() {
    let target_keyword = String::from("XMAS");

    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in io::stdin().lines() {
        let row = line.unwrap();
        if row.len() == 0 {
            continue;
        }
        grid.push(row.chars().collect());
    }

    let n_row = grid.len();
    let n_col = grid[0].len();
    let mut result: i32 = 0;
    for i in 0..n_row {
        for j in 0..n_col {
            if grid[i][j] != 'X' {
                continue;
            }

            let h_fwd = if j + 3 < n_col {
                combine_chars(
                    grid[i][j],
                    grid[i][j + 1],
                    grid[i][j + 2],
                    grid[i][j + 3]
                )
            } else {String::new()};

            let h_rev = if j >= 3 {
                combine_chars(
                    grid[i][j],
                    grid[i][j - 1],
                    grid[i][j - 2],
                    grid[i][j - 3]
                )
            } else {String::new()};

            let v_fwd = if i + 3 < n_row {
                combine_chars(
                    grid[i][j],
                    grid[i + 1][j],
                    grid[i + 2][j],
                    grid[i + 3][j]
                )
            } else {String::new()};

            let v_rev = if i >= 3 {
                combine_chars(
                    grid[i][j],
                    grid[i - 1][j],
                    grid[i - 2][j],
                    grid[i - 3][j]
                )
            } else {String::new()};

            let d_down_right = if i + 3 < n_row && j + 3 < n_col {
                combine_chars(
                    grid[i][j],
                    grid[i + 1][j + 1],
                    grid[i + 2][j + 2],
                    grid[i + 3][j + 3]
                )
            } else {String::new()};

            let d_down_left = if i + 3 < n_row && j >= 3 {
                combine_chars(
                    grid[i][j],
                    grid[i + 1][j - 1],
                    grid[i + 2][j - 2],
                    grid[i + 3][j - 3]
                )
            } else {String::new()};

            let d_up_right = if i >= 3 && j + 3 < n_col {
                combine_chars(
                    grid[i][j],
                    grid[i - 1][j + 1],
                    grid[i - 2][j + 2],
                    grid[i - 3][j + 3]
                )
            } else {String::new()};

            let d_up_left = if i >= 3 && j >= 3 {
                combine_chars(
                    grid[i][j],
                    grid[i - 1][j - 1],
                    grid[i - 2][j - 2],
                    grid[i - 3][j - 3]
                )
            } else {String::new()};

            for phrase in [h_fwd, h_rev, v_fwd, v_rev, d_down_left, d_down_right, d_up_left, d_up_right] {
                if phrase == target_keyword {
                    result += 1;
                }
            }
        }
    }

    println!("{result}");
}
