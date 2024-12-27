use std::{collections::HashMap, io};

use scanf::sscanf;

type WireData = (String, char, String, Option<u8>);

fn shorten_operator(op: &String) -> char {
    match op.as_str() {
        "AND" => '&',
        "OR" => '|',
        "XOR" => '^',
        _ => panic!("unknown operator \"{}\"", op),
    }
}

fn find_wire(wire_data: &HashMap<String, WireData>, w1: &str, w2: &str, op: char) -> String {
    let ori_w1 = w1;
    let ori_w2 = w2;
    let mut w1 = String::from(w1);
    let mut w2 = String::from(w2);
    if w1 > w2 {
        (w1, w2) = (w2, w1);
    }

    for (d_w3, (d_w1, d_op, d_w2, _)) in wire_data.iter() {
        if w1 == *d_w1 && op == *d_op && w2 == *d_w2 {
            return d_w3.clone();
        }
    }

    panic!("wire not found {} {} {}", ori_w1, ori_w2, op);
}

fn swap_data(
    wire_data: &mut HashMap<String, WireData>,
    w1: &String,
    w2: &String,
) {
    let data1 = wire_data.get(w1).unwrap().clone();
    let data2 = wire_data.get(w2).unwrap().clone();
    wire_data.insert(w1.clone(), data2);
    wire_data.insert(w2.clone(), data1);

    println!("swapped {} {}", w1, w2);
}

fn main() {
    /*
        This program instead of finding answer directly, it at with z{iter} the problem is at.
        The problem is solved by iteratively find where issues lie, look at data/day_24_graph.svg then hard code which pair to swap in code.
     */

    let mut wire_data = HashMap::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line = line.trim().to_string();
        if line.len() == 0 {
            break;
        }

        let mut name = String::new();
        let mut value: u8 = 0;
        sscanf!(&line, "{}: {}", name, value).unwrap();

        wire_data.entry(name).or_insert((
            String::new(),
            ' ',
            String::new(),
            Some(value),
        ));
    }
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line = line.trim().to_string();
        if line.len() == 0 {
            break;
        }

        let mut w1 = String::new();
        let mut w2 = String::new();
        let mut op = String::new();
        let mut w3 = String::new();
        sscanf!(&line, "{} {} {} -> {}", w1, op, w2, w3).unwrap();
        if w1 > w2 {
            (w1, w2) = (w2, w1);
        }

        wire_data.entry(w3).or_insert((
            w1,
            shorten_operator(&op),
            w2,
            None,
        ));
    }

    // soluion section
    let swapped = vec![
        ("hwk", "z06"),
        ("qmd", "tnt"),
        ("z31", "hpc"),
        ("z37", "cgr"),
    ];
    for (w1, w2) in swapped.iter() {
        swap_data(&mut wire_data, &String::from(*w1), &String::from(*w2));
    }

    // find next failure area
    let z0 = find_wire(&wire_data, "x00", "y00", '^');
    assert!(z0 == "z00");

    let mut carry0 = find_wire(&wire_data, "x00", "y00", '&');
    for i in 1..45 {
        println!("iter: {}", i);
        let x = format!("x{:0>2}", i);
        let y = format!("y{:0>2}", i);
        let z = format!("z{:0>2}", i);

        let sum1 = find_wire(&wire_data, x.as_str(), y.as_str(), '^');
        let z1 = find_wire(&wire_data, sum1.as_str(), carry0.as_str(), '^');
        let carry1_a = find_wire(&wire_data, sum1.as_str(), carry0.as_str(), '&');
        let carry1_b = find_wire(&wire_data, x.as_str(), y.as_str(), '&');
        let carry1 = find_wire(&wire_data, carry1_a.as_str(), carry1_b.as_str(), '|');

        assert!(z == z1, "{} is not correct output", z);

        carry0 = carry1;
    }

    println!("reached ending!");
    let mut result = Vec::new();
    for (w1, w2) in swapped {
        result.push(w1);
        result.push(w2);
    }
    result.sort();
    println!("{}", result.join(","));
}
