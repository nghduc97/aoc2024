use std::io::{self, Read};

fn pos_move((x, y): (usize, usize), (dx, dy): (i32, i32), step: i32) -> (usize, usize) {
    return (((x as i32) + dx * step) as usize, ((y as i32) + dy * step) as usize);
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

    let mut move_sequence = String::new();
    io::stdin().read_to_string(&mut move_sequence).unwrap();
    let move_sequence: Vec<char> = move_sequence.chars().filter(|x| {
        match x {
            '<' => true,
            '>' => true,
            '^' => true,
            'v' => true,
            _ => false,
        }
    }).collect();

    let n_row = grid.len();
    let n_col = grid[0].len();
    let (mut px, mut py) = (0, 0);
    'init_robot_search: for x in 0..n_row {
        for y in 0..n_col {
            if grid[x][y] == '@' {
                (px, py) = (x, y);
                break 'init_robot_search;
            }
        }
    }

    for mv in move_sequence {
        let (dx, dy) = match mv {
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => panic!(),
        };

        // find the number of boxes to be pushed
        let mut n_box = 0;
        loop {
            let (x, y) = pos_move((px, py), (dx, dy), n_box + 1);
            match grid[x][y] {
                '.' => break,
                '#' => break,
                'O' => { n_box += 1; continue; },
                _ => panic!(),
            }
        }

        // move and push boxes if not hindered by wall
        let (ex, ey) = pos_move((px, py), (dx, dy), n_box + 1);
        match grid[ex][ey] {
            '#' => {},
            '.' => {
                for i in (0..=n_box).rev() {
                    let (ix, iy) = pos_move((px, py), (dx, dy), i);
                    let (jx, jy) = pos_move((px, py), (dx, dy), i + 1);
                    grid[jx][jy] = grid[ix][iy];
                }
                grid[px][py] = '.';
                (px, py) = pos_move((px, py), (dx, dy), 1);
            },
            _ => panic!(),
        }
    }

    let mut result = 0;
    for x in 0..n_row {
        for y in 0..n_col {
            if grid[x][y] == 'O' {
                result += x * 100 + y;
            }
            print!("{}", grid[x][y]);
        }
        println!();
    }
    println!("{}", result);
}
