use std::{collections::HashSet, io};

fn main() {
    let mut edges = HashSet::new();
    let mut computers: HashSet<String> = HashSet::new();
    let mut result = 0;
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line = line.trim().to_string();
        if line.len() == 0 {
            break;
        }

        let (c1, c2) = line.split_once('-').unwrap();
        let (c1, c2) = (c1.to_string(), c2.to_string());
        edges.insert((c1.clone(), c2.clone()));
        edges.insert((c2.clone(), c1.clone()));

        for c3 in computers.iter() {
            if !c1.starts_with('t') && !c2.starts_with('t') && !c3.starts_with('t') {
                continue;
            }
            if edges.contains(&(c1.clone(), c3.clone())) && edges.contains(&(c2.clone(), c3.clone())) {
                result += 1;
            }
        }

        computers.insert(c1);
        computers.insert(c2);
    }

    println!("{}", result);
}
