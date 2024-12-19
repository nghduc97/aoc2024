use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let patterns: Vec<String> = line.trim().split(", ").map(|tk| tk.to_string()).collect();

    let mut designs = Vec::new();
    io::stdin().read_line(&mut line).unwrap();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line = line.trim().to_string();
        if line.len() == 0 {
            break;
        }
        designs.push(line);
    }

    let mut result = 0;
    for design in designs {
        let mut match_tbl = vec![false].repeat(design.len() + 1);
        match_tbl[0] = true;

        for i in 0..design.len() {
            if match_tbl[i] {
                for p in patterns.iter() {
                    if i + p.len() <= design.len() && design[i..i + p.len()] == *p {
                        match_tbl[i + p.len()] = true;
                    }
                }
            }
        }

        if match_tbl[design.len()] {
            result += 1;
        }
    }

    println!("{}", result);
}
