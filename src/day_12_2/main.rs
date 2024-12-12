use std::io;

const DIRECTION_DELTAS: [(i32, i32); 4] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
];

struct RegionData {
    label: i32,
    n_side: i32,
    area: i32,
}

fn cell_inbound(n_row: usize, n_col: usize, x: i32, y: i32) -> bool {
    return x >= 0 && y >= 0 && (x as usize) < n_row && (y as usize) < n_col;
}

fn explore_region(
    grid: &Vec<Vec<char>>,
    region_grid: &mut Vec<Vec<i32>>,
    x: usize,
    y: usize,
    data: &mut RegionData,
) {
    region_grid[x][y] = data.label;
    data.area += 1;

    let n_row = grid.len();
    let n_col = grid[0].len();
    for (dx, dy) in DIRECTION_DELTAS {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if cell_inbound(n_row, n_col, nx, ny) {
            let (nx, ny) = (nx as usize, ny as usize);
            if grid[nx][ny] == grid[x][y] && region_grid[nx][ny] < 0 {
                explore_region(grid, region_grid, nx, ny, data);
            }
        }
    }
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

    let mut region_grid: Vec<Vec<i32>> = (0..n_row).map(|_| vec![-1].repeat(n_col)).collect();
    let mut regions = Vec::new();
    let mut label: i32 = 0;
    for x in 0..n_row {
        for y in 0..n_col {
            if region_grid[x][y] < 0 {
                let mut data = RegionData{
                    label,
                    n_side: 0,
                    area: 0,
                };
                explore_region(&grid, &mut region_grid, x, y, &mut data);
                label += 1;
                regions.push(data);
            }
        }
    }

    // horizontal sides
    for x in 0..=n_row {
        let mut r1 = -1;
        let mut r2 = -1;
        for y in 0..n_col {
            if x == 0 || x == n_row || region_grid[x - 1][y] != region_grid[x][y] {
                if x > 0 && r1 != region_grid[x - 1][y] {
                    r1 = region_grid[x - 1][y];
                    regions[r1 as usize].n_side += 1;
                }
                if x < n_row && r2 != region_grid[x][y] {
                    r2 = region_grid[x][y];
                    regions[r2 as usize].n_side += 1;
                }
            } else {
                r1 = -1;
                r2 = -1;
            }
        }
    }

    // vertical sides
    for y in 0..=n_col {
        let mut r1 = -1;
        let mut r2 = -1;
        for x in 0..n_row {
            if y == 0 || y == n_col || region_grid[x][y - 1] != region_grid[x][y] {
                if y > 0 && r1 != region_grid[x][y - 1] {
                    r1 = region_grid[x][y - 1];
                    regions[r1 as usize].n_side += 1;
                }
                if y < n_col && r2 != region_grid[x][y] {
                    r2 = region_grid[x][y];
                    regions[r2 as usize].n_side += 1;
                }
            } else {
                r1 = -1;
                r2 = -1;
            }
        }
    }

    // result
    let mut result: i32 = 0;
    for region in regions {
        result += region.area * region.n_side;
    }

    println!("{}", result);
}
