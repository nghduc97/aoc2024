use std::io;

fn explore_score(score_grid: &mut Vec<Vec<i32>>, grid: &Vec<Vec<u32>>, x: usize, y: usize) -> i32 {
    if grid[x][y] == 9 {
        return 1;
    }
    if score_grid[x][y] >= 0 {
        return score_grid[x][y];
    }

    let n_row = grid.len();
    let n_col = grid[0].len();
    let mut score = 0;
    if x + 1 < n_row && grid[x + 1][y] == grid[x][y] + 1 {
        score += explore_score(score_grid, grid, x + 1, y);
    }
    if x > 0 && grid[x - 1][y] == grid[x][y] + 1 {
        score += explore_score(score_grid, grid, x - 1, y);
    }
    if y + 1 < n_col && grid[x][y + 1] == grid[x][y] + 1 {
        score += explore_score(score_grid, grid, x, y + 1);
    }
    if y > 0 && grid[x][y - 1] == grid[x][y] + 1 {
        score += explore_score(score_grid, grid, x, y - 1);
    }

    score_grid[x][y] = score;
    return score;
}

fn main() {
    let mut grid: Vec<Vec<u32>> = Vec::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line = line.trim().to_string();
        if line.len() == 0 {
            break;
        }
        grid.push(line.chars().map(|x| x.to_digit(10).unwrap()).collect());
    }

    let n_row = grid.len();
    let n_col = grid[0].len();

    let mut score_grid = Vec::new();
    for _ in 0..n_row {
        score_grid.push(vec![-1].repeat(n_col));
    }

    let mut result = 0;
    for x in 0..n_row {
        for y in 0..n_col {
            if grid[x][y] == 0 {
                result += explore_score(&mut score_grid, &grid, x, y);
            }
        }
    }
    println!("{}", result);
}
