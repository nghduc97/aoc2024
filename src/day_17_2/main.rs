use std::io;

fn recursive_search(outputs: &Vec<u32>, idx: usize, a: u64) -> Option<u64> {
    if idx == 0 {
        return Some(a);
    }

    let output = outputs[idx - 1];
    for pa in a * 8..a * 8 + 8 {
        if pa == 0 {
            continue;
        }
        let mut pb = pa % 8;
        pb ^= 7;
        let pc = pa / 2u64.pow(pb as u32);
        pb ^= 7;
        // a = pa / 8
        pb ^= pc;
        if (pb % 8) as u32 == output {
            match recursive_search(outputs, idx - 1, pa) {
                Some(result) => return Some(result),
                _ => {},
            }
        }
    }

    return None;
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

    let opcodes_str = line4.trim().split_at("Program: ".len()).1;
    let opcodes: Vec<u32> = opcodes_str.split(",")
        .map(|tk| tk.parse::<u32>().unwrap())
        .collect();

    println!("{}", recursive_search(&opcodes, opcodes.len(), 0).unwrap());
}
