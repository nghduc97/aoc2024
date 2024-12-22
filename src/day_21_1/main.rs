use std::{io, usize};

use scanf::sscanf;

const NUM_CODES: usize = 5;

fn get_numpad_position(btn: char) -> (i32, i32) {
    match btn {
        '0' => (0, 1),
        'A' => (0, 2),
        '1' => (1, 0),
        '2' => (1, 1),
        '3' => (1, 2),
        '4' => (2, 0),
        '5' => (2, 1),
        '6' => (2, 2),
        '7' => (3, 0),
        '8' => (3, 1),
        '9' => (3, 2),
        _ => panic!("invalid numpad button value"),
    }
}

fn get_direction_pad_position(btn: char) -> (i32, i32) {
    match btn {
        '<' => (0, 0),
        'v' => (0, 1),
        '>' => (0, 2),
        '^' => (1, 1),
        'A' => (1, 2),
        _ => panic!("invalid numpad button value"),
    }
}

fn get_pad_position(layer: usize, btn: char) -> (i32, i32) {
    if layer > 0 {
        get_direction_pad_position(btn)
    } else {
        get_numpad_position(btn)
    }
}

fn move_by_x_will_escape(layer: usize, (_, y1): (i32, i32), (x2, _): (i32, i32)) -> bool {
    if layer == 0 {
        return (x2, y1) == (0, 0);
    } else {
        return (x2, y1) == (1, 0);
    }
}

fn move_by_y_will_escape(layer: usize, (x1, _): (i32, i32), (_, y2): (i32, i32)) -> bool {
    if layer == 0 {
        return (x1, y2) == (0, 0);
    } else {
        return (x1, y2) == (1, 0);
    }
}

fn get_numeric_of_code(code: String) -> i32 {
    let mut num: i32 = 0;
    sscanf!(&code, "{}A", num).unwrap();
    return num;
}

fn calc_len(layer: usize, instructions: &Vec<char>) -> usize {
    if layer == 3 {
        return instructions.len();
    }

    let mut total_len = 0;
    let (mut x, mut y) = get_pad_position(layer, 'A');
    for btn in instructions {
        let (nx, ny) = get_pad_position(layer, *btn);
        let x_insts = if x == nx {
            Vec::new()
        } else {
            let btn = if x < nx { '^' } else { 'v' };
            let num = x.abs_diff(nx) as usize;
            vec![btn].repeat(num)
        };
        let y_insts = if y == ny {
            Vec::new()
        } else {
            let btn = if y < ny { '>' } else { '<' };
            let num = y.abs_diff(ny) as usize;
            vec![btn].repeat(num)
        };

        let mut best_len = usize::MAX;
        if !move_by_x_will_escape(layer, (x, y), (nx, ny)) {
            let mut insts = x_insts.clone();
            insts.extend(y_insts.clone());
            insts.push('A');
            best_len = best_len.min(calc_len(layer + 1, &insts));
        }
        if !move_by_y_will_escape(layer, (x, y), (nx, ny)) {
            let mut insts = y_insts.clone();
            insts.extend(x_insts.clone());
            insts.push('A');
            best_len = best_len.min(calc_len(layer + 1, &insts));
        }
        assert!(best_len < usize::MAX);

        total_len += best_len;
        (x, y) = (nx, ny);
    }

    return total_len;
}

fn calc_complexity(code: String) -> i32 {
    let instructions: Vec<char> = code.chars().collect();

    let code_len = calc_len(0, &instructions);
    println!("{} {}", code, code_len);
    let numeric_part = get_numeric_of_code(code);
    return (code_len as i32) * numeric_part;
}

fn main() {
    let mut codes = Vec::new();
    for _ in 0..NUM_CODES {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        codes.push(line.trim().to_string());
    }

    let mut result = 0;
    for code in codes {
        result += calc_complexity(code);
    }
    println!("{}", result);
}
