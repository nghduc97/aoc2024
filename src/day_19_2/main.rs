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

    let mut result: i64 = 0;
    for design in designs {
        let mut tbl: Vec<i64> = vec![0].repeat(design.len() + 1);
        tbl[0] = 1;

        for i in 0..design.len() {
            if tbl[i] > 0 {
                for p in patterns.iter() {
                    if i + p.len() <= design.len() && design[i..i + p.len()] == *p {
                        tbl[i + p.len()] += tbl[i];
                    }
                }
            }
        }

        result += tbl[design.len()];
    }

    println!("{}", result);
}
