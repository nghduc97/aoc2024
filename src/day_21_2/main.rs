use std::{collections::HashMap, io, usize};

use scanf::sscanf;

// const NUM_LAYERS: usize = 3;
const NUM_LAYERS: usize = 26;
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

fn out_of_bound(layer: usize, (x, y): (i32, i32)) -> bool {
    if x < 0 || y < 0 {
        return true;
    }
    if layer == 0 {
        return x >= 4 || y >= 3 || (x, y) == (0, 0);
    }
    return x >= 2 || y >= 3 || (x, y) == (1, 0);
}

fn get_numeric_of_code(code: String) -> u64 {
    let mut num: u64 = 0;
    sscanf!(&code, "{}A", num).unwrap();
    return num;
}

fn calc_len(
    cache: &mut HashMap<(usize, (i32, i32), (i32, i32)), u64>,
    layer: usize,
    (x, y): (i32, i32),
    (ex, ey): (i32, i32),
) -> u64 {
    if layer == NUM_LAYERS {
        return 1;
    }
    if (x, y) == (ex, ey) {
        return 1;
    }
    let key = (layer, (x, y), (ex, ey));
    match cache.get(&key) {
        Some(value) => return *value,
        None => {},
    }

    let x_btn = if x < ex { '^' } else { 'v' };
    let y_btn = if y < ey { '>' } else { '<' };

    if ex == x {
        let mut len = 0;
        len += calc_len(
            cache,
            layer + 1,
            get_direction_pad_position('A'),
            get_direction_pad_position(y_btn),
        );
        len += (y.abs_diff(ey) - 1) as u64;
        len += calc_len(
            cache,
            layer + 1,
            get_direction_pad_position(y_btn),
            get_direction_pad_position('A'),
        );
        return len;
    }
    if ey == y {
        let mut len = 0;
        len += calc_len(
            cache,
            layer + 1,
            get_direction_pad_position('A'),
            get_direction_pad_position(x_btn),
        );
        len += (x.abs_diff(ex) - 1) as u64;
        len += calc_len(
            cache,
            layer + 1,
            get_direction_pad_position(x_btn),
            get_direction_pad_position('A'),
        );
        return len;
    }

    let mut best_len = u64::MAX;
    if !out_of_bound(layer, (ex, y)) {
        let mut len = 0;
        len += calc_len(
            cache,
            layer + 1,
            get_direction_pad_position('A'),
            get_direction_pad_position(x_btn),
        );
        len += (x.abs_diff(ex) - 1) as u64;
        len += calc_len(
            cache,
            layer + 1,
            get_direction_pad_position(x_btn),
            get_direction_pad_position(y_btn),
        );
        len += (y.abs_diff(ey) - 1) as u64;
        len += calc_len(
            cache,
            layer + 1,
            get_direction_pad_position(y_btn),
            get_direction_pad_position('A'),
        );

        best_len = best_len.min(len);
    }
    if !out_of_bound(layer, (x, ey)) {
        let mut len = 0;
        len += calc_len(
            cache,
            layer + 1,
            get_direction_pad_position('A'),
            get_direction_pad_position(y_btn),
        );
        len += (y.abs_diff(ey) - 1) as u64;
        len += calc_len(
            cache,
            layer + 1,
            get_direction_pad_position(y_btn),
            get_direction_pad_position(x_btn),
        );
        len += (x.abs_diff(ex) - 1) as u64;
        len += calc_len(
            cache,
            layer + 1,
            get_direction_pad_position(x_btn),
            get_direction_pad_position('A'),
        );

        best_len = best_len.min(len);
    }
    assert!(best_len < u64::MAX);

    cache.insert(key, best_len);
    return best_len;
}

fn calc_complexity(code: String) -> u64 {
    let mut cache = HashMap::new();

    let mut code_len = 0;
    let (mut x, mut y) = get_numpad_position('A');
    for btn in code.chars() {
        let (nx, ny) = get_numpad_position(btn);
        let len = calc_len(&mut cache, 0, (x, y), (nx, ny));
        code_len += len;
        (x, y) = (nx, ny);
    }
    println!("{} {}", code, code_len);

    let numeric_part = get_numeric_of_code(code);
    return code_len * numeric_part;
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
