use std::io;

fn combine_chars(c1: char, c2: char, c3: char) -> String {
    let mut s = String::new();
    s.push(c1);
    s.push(c2);
    s.push(c3);
    s
}

fn main() {
    let target_keyword = String::from("MAS");

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
    for i in 1..n_row - 1 {
        for j in 1..n_col - 1 {
            if grid[i][j] != 'A' {
                continue;
            }

            let mut p1 = combine_chars(
                grid[i - 1][j - 1],
                grid[i][j],
                grid[i + 1][j + 1]
            );
            let mut p2 = combine_chars(
                grid[i + 1][j - 1],
                grid[i][j],
                grid[i - 1][j + 1]
            );
            if p1 != target_keyword {
                p1 = p1.chars().rev().collect::<String>();
            }
            if p2 != target_keyword {
                p2 = p2.chars().rev().collect::<String>();
            }
            if p1 == target_keyword && p2 == target_keyword {
                result += 1;
            }
        }
    }

    println!("{result}");
}
