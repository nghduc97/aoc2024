use std::io;

const DIRECTION_DELTAS: [(i32, i32); 4] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
];

struct RegionData {
    label: i32,
    perimeter: i32,
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
            if grid[nx][ny] == grid[x][y] {
                if region_grid[nx][ny] < 0 {
                    explore_region(grid, region_grid, nx, ny, data);
                }
            } else {
                data.perimeter += 1;
            }
        } else {
            data.perimeter += 1;
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
    let mut label: i32 = 0;
    let mut result: i64 = 0;
    for x in 0..n_row {
        for y in 0..n_col {
            if region_grid[x][y] < 0 {
                let mut data = RegionData{
                    label,
                    perimeter: 0,
                    area: 0,
                };
                explore_region(&grid, &mut region_grid, x, y, &mut data);
                label += 1;

                result += (data.area * data.perimeter) as i64;
            }
        }
    }

    println!("{}", result);
}
