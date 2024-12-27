use std::{collections::HashMap, io, ops::Shl};

use scanf::sscanf;

struct WireData {
    value: Option<u8>,
    inputs: (String, String),
    operator: String,
}

fn get_wire_value(wire_data: &mut HashMap<String, WireData>, key: String) -> u8 {
    match wire_data.get_mut(&key) {
        None => panic!("key not found \"{}\"", key),
        Some(data) => {
            if data.value.is_some() {
                return data.value.unwrap();
            }
            let op = data.operator.clone();
            let (w0, w1) = data.inputs.clone();
            let v0 = get_wire_value(wire_data, w0);
            let v1 = get_wire_value(wire_data, w1);
            if op == "OR" {
                return v0 | v1;
            }
            if op == "XOR" {
                return v0 ^ v1;
            }
            if op == "AND" {
                return v0 & v1;
            }
            panic!("unknown operator \"{}\"", op);
        }
    }
}

fn main() {
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

        wire_data.entry(name).or_insert(WireData{
            value: Some(value),
            inputs: (String::new(), String::new()),
            operator: String::new(),
        });
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

        wire_data.entry(w3).or_insert(WireData{
            value: None,
            inputs: (w1, w2),
            operator: op,
        });
    }

    let mut result: u64 = 0;
    for i in 0..64 {
        let key = format!("z{:0>2}", i);
        if !wire_data.contains_key(&key) {
            break;
        }

        let value = get_wire_value(&mut wire_data, key) as u64;
        result |= value.shl(i);
    }
    println!("{}", result);
}
