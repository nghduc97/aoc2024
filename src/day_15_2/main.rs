use std::{collections::VecDeque, io::{self, Read}};

fn pos_move((x, y): (usize, usize), (dx, dy): (i32, i32), step: i32) -> (usize, usize) {
    return (((x as i32) + dx * step) as usize, ((y as i32) + dy * step) as usize);
}

// bfs to collect position of all boxes that will be pushed in order of distance from starting point
// return boxes to be pushed, return None if stopped by walls
fn bfs_boxes(
    grid: &Vec<Vec<char>>,
    starting_boxes: Vec<(usize, usize)>,
    (dx, dy): (i32, i32),
) -> Option<Vec<(usize, usize)>> {
    let mut queue = VecDeque::new();
    for (x, y) in starting_boxes {
        queue.push_back((x, y));
    }

    let mut boxes = Vec::new();
    while queue.len() > 0 {
        let (x, y) = queue.pop_front().unwrap();
        boxes.push((x, y));
        assert!(grid[x][y] == '[');

        if dx != 0 {
            let (n1x, n1y) = pos_move((x, y), (dx, dy), 1);
            let (n2x, n2y) = pos_move((x, y + 1), (dx, dy), 1);

            if grid[n1x][n1y] == '#' || grid[n2x][n2y] == '#' {
                return None;
            }

            if grid[n1x][n1y] == '[' {
                queue.push_back((n1x, n1y));
            } else {
                if grid[n1x][n1y] == ']' {
                    queue.push_back((n1x, n1y - 1));
                }
                if grid[n2x][n2y] == '[' {
                    queue.push_back((n2x, n2y));
                }
            }
        } else if dy > 0 {
            let (nx, ny) = pos_move((x, y), (dx, dy), 2);
            match grid[nx][ny] {
                '[' => { queue.push_back((nx, ny)); }
                '#' => return None,
                _ => {},
            }
        } else {
            let (nx, ny) = pos_move((x, y), (dx, dy), 1);
            match grid[nx][ny] {
                ']' => { queue.push_back((nx, ny - 1)); }
                '#' => return None,
                _ => {},
            }
        }
    }

    return Some(boxes);
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
        let row = line.chars().map(|c| {
            match c {
                '#' => "##",
                'O' => "[]",
                '.' => "..",
                '@' => "@.",
                _ => panic!(),
            }
        }).collect::<Vec<&str>>().join("").chars().collect();
        grid.push(row);
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

    for mv in &move_sequence {
        assert!(grid[px][py] == '@');

        let (dx, dy) = match mv {
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => panic!(),
        };

        let mut starting_boxes = Vec::new();
        let (npx, npy) = pos_move((px, py), (dx, dy), 1);
        match grid[npx][npy] {
            '[' => starting_boxes.push((npx, npy)),
            ']' => starting_boxes.push((npx, npy - 1)),
            '#' => continue,
            '.' => {
                grid[npx][npy] = '@';
                grid[px][py] = '.';
                (px, py) = (npx, npy);
                continue;
            },
            _ => panic!(),
        }
        match bfs_boxes(&grid, starting_boxes, (dx, dy)) {
            Some(boxes) => {
                for (x, y) in boxes.into_iter().rev() {
                    let (nx, ny) = pos_move((x, y), (dx, dy), 1);
                    grid[x][y] = '.';
                    grid[x][y + 1] = '.';
                    grid[nx][ny] = '[';
                    grid[nx][ny + 1] = ']';
                }
                grid[npx][npy] = '@';
                grid[px][py] = '.';
                (px, py) = (npx, npy);
            },
            None => {
                println!("{}", mv);
                for x in 0..n_row {
                    for y in 0..n_col {
                        print!("{}", grid[x][y]);
                    }
                    println!();
                }
                println!();
            },
        }
    }

    let mut result = 0;
    for x in 0..n_row {
        for y in 0..n_col {
            if grid[x][y] == '[' {
                result += x * 100 + y;
            }
            print!("{}", grid[x][y]);
        }
        println!();
    }
    println!("{}", result);
}
