use std::io;
use scanf::sscanf;

fn get_combo_operand(operand: u32, a: u32, b: u32, c: u32) -> u32 {
    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => a,
        5 => b,
        6 => c,
        _ => panic!("invalid operand"),
    }
}

fn main() {
    let mut line1 = String::new();
    let mut line2 = String::new();
    let mut line3 = String::new();
    let mut line4 = String::new();
    io::stdin().read_line(&mut line1).unwrap();
    io::stdin().read_line(&mut line2).unwrap();
    io::stdin().read_line(&mut line3).unwrap();
    io::stdin().read_line(&mut String::new()).unwrap();
    io::stdin().read_line(&mut line4).unwrap();

    let mut a: u32 = 0;
    let mut b: u32 = 0;
    let mut c: u32 = 0;
    sscanf!(&line1, "Register A: {}", a).unwrap();
    sscanf!(&line2, "Register B: {}", b).unwrap();
    sscanf!(&line3, "Register C: {}", c).unwrap();

    let opcodes_str = line4.trim().split_at("Program: ".len()).1;
    let opcodes: Vec<u32> = opcodes_str.split(",")
        .map(|tk| tk.parse::<u32>().unwrap())
        .collect();

    let mut output = Vec::new();
    let mut intruction_ptr = 0;
    while intruction_ptr + 1 < opcodes.len() {
        let current_code = opcodes[intruction_ptr];
        let next_code = opcodes[intruction_ptr + 1];

        match current_code {
            0 => {
                let operand = get_combo_operand(next_code, a, b, c);
                a /= 2u32.pow(operand);
                intruction_ptr += 2;
            },
            1 => {
                b ^= next_code;
                intruction_ptr += 2;
            },
            2 => {
                let operand = get_combo_operand(next_code, a, b, c);
                b = operand % 8;
                intruction_ptr += 2;
            },
            3 => {
                if a != 0 {
                    intruction_ptr = next_code as usize;
                } else {
                    intruction_ptr += 2;
                }
            },
            4 => {
                b = b ^ c;
                intruction_ptr += 2;
            },
            5 => {
                let operand = get_combo_operand(next_code, a, b, c);
                output.push(operand % 8);
                intruction_ptr += 2;
            },
            6 => {
                let operand = get_combo_operand(next_code, a, b, c);
                b = a / 2u32.pow(operand);
                intruction_ptr += 2;
            },
            7 => {
                let operand = get_combo_operand(next_code, a, b, c);
                c = a / 2u32.pow(operand);
                intruction_ptr += 2;
            },
            _ => panic!(),
        }
    }

    let result = output.iter()
        .map(|out| out.to_string())
        .collect::<Vec<String>>()
        .join(",");
    println!("{}", result);
}
