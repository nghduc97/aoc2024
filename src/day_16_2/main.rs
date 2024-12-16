use std::{collections::{BTreeSet, VecDeque}, io::{self}};

// 4 directions in clock-wise order
const DIRECTIONS: [(i32, i32); 4] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
];

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

    let (mut sx, mut sy) = (0, 0);
    for x in 0..n_row {
        for y in 0..n_col {
            if grid[x][y] == 'S' {
                (sx, sy) = (x, y);
            }
        }
    }

    let mut score_grid = Vec::new();
    for _ in 0..n_row {
        let mut vec = Vec::new();
        for _ in 0..n_col {
            vec.push(vec![i32::MAX].repeat(4));
        }
        score_grid.push(vec);
    }
    score_grid[sx][sy][0] = 0;

    let mut score_set = BTreeSet::new();
    score_set.insert((0, sx, sy, 0));

    let mut last_state = (0, 0, 0);
    while score_set.len() > 0 {
        let (current_score, x, y, dir_idx) = score_set.pop_first().unwrap();
        let (dx, dy) = DIRECTIONS[dir_idx];

        if grid[x][y] == 'E' {
            last_state = (x, y, dir_idx);
            break;
        }

        // go forward
        let nx = ((x as i32) + dx) as usize;
        let ny = ((y as i32) + dy) as usize;
        let old_score = score_grid[nx][ny][dir_idx];
        let new_score = current_score + 1;
        if grid[nx][ny] != '#' && old_score > new_score {
            score_set.remove(&(old_score, nx, ny, dir_idx));
            score_grid[nx][ny][dir_idx] = new_score;
            score_set.insert((new_score, nx, ny, dir_idx));
        }

        // change directions
        for d_dir in [-1, 1] {
            let next_dir = (((dir_idx as i32) + d_dir + 4) % 4) as usize;
            let old_score = score_grid[x][y][next_dir];
            let new_score = current_score + 1000;
            if grid[x][y] != '#' && old_score > new_score {
                score_set.remove(&(old_score, x, y, next_dir));
                score_grid[x][y][next_dir] = new_score;
                score_set.insert((new_score, x, y, next_dir));
            }
        }
    }

    let (ex, ey, e_dir) = last_state;
    assert!(score_grid[ex][ey][e_dir] < i32::MAX, "cannot reach ending");

    // retrace
    let mut visited_grid: Vec<Vec<bool>> = (0..n_row).map(|_| vec![false].repeat(n_col)).collect();
    let mut result = 0;
    let mut queue = VecDeque::new();
    queue.push_back((ex, ey, e_dir));

    while queue.len() > 0 {
        let (x, y, dir) = queue.pop_front().unwrap();
        if !visited_grid[x][y] {
            result += 1;
            visited_grid[x][y] = true;
        }
        if grid[x][y] == 'S' {
            continue;
        }

        // go backward
        let (dx, dy) = DIRECTIONS[(dir + 2) % 4];
        let px = ((x as i32) + dx) as usize;
        let py = ((y as i32) + dy) as usize;
        if grid[px][py] != '#' && score_grid[px][py][dir] == score_grid[x][y][dir] - 1 {
            queue.push_back((px, py, dir));
        }

        // change directions
        for d_dir in [-1, 1] {
            let p_dir = (((dir as i32) + d_dir + 4) % 4) as usize;
            if grid[x][y] != '#' && score_grid[x][y][p_dir] == score_grid[x][y][dir] - 1000 {
                queue.push_back((x, y, p_dir));
            }
        }
    }

    println!("{}", result);
}
