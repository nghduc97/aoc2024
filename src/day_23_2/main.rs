use std::{collections::{HashMap, HashSet}, io, sync::{mpsc::channel, Arc}};

use threadpool::ThreadPool;

fn find_largest_clique(
    result: &mut HashSet<usize>,
    adj_list: &Vec<HashSet<usize>>,
    members: HashSet<usize>,
    candidates: HashSet<usize>,
) {
    if members.len() > result.len() {
        *result = members.clone();
    }
    if candidates.len() + members.len() < result.len() {
        return;
    }

    for x in candidates.iter() {
        let mut next_members = members.clone();
        next_members.insert(*x);

        let next_candidates = candidates.intersection(&adj_list[*x])
            .map(|y| y.clone())
            .filter(|y| *y > *x)
            .collect::<HashSet<usize>>();

        find_largest_clique(
            result,
            adj_list,
            next_members,
            next_candidates,
        );
    }
}

fn main() {
    let mut adj_list = Vec::new();
    let mut computer_to_idx = HashMap::new();
    let mut idx_to_computer = Vec::new();
    let mut n_computer = 0;
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line = line.trim().to_string();
        if line.len() == 0 {
            break;
        }

        let (c1, c2) = line.split_once('-').unwrap();
        let (c1, c2) = (c1.to_string(), c2.to_string());
        if !computer_to_idx.contains_key(&c1) {
            computer_to_idx.insert(c1.clone(), n_computer);
            idx_to_computer.push(c1.clone());
            adj_list.push(HashSet::new());
            n_computer += 1;
        }
        if !computer_to_idx.contains_key(&c2) {
            computer_to_idx.insert(c2.clone(), n_computer);
            idx_to_computer.push(c2.clone());
            adj_list.push(HashSet::new());
            n_computer += 1;
        }

        let c1 = computer_to_idx.get(&c1).unwrap().clone();
        let c2 = computer_to_idx.get(&c2).unwrap().clone();
        adj_list[c1].insert(c2);
        adj_list[c2].insert(c1);
    }

    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();
    let adj_list = Arc::new(adj_list);
    for x in 0..adj_list.len() {
        let tx = tx.clone();
        let adj_list = adj_list.clone();
        pool.execute(move || {
            let mut largest_clique = HashSet::new();
            find_largest_clique(
                &mut largest_clique,
                &adj_list,
                HashSet::from([x]),
                adj_list[x].clone(),
            );
            tx.send(largest_clique).unwrap();
        });
    }
    pool.join();
    drop(tx);

    let mut best_clique = HashSet::new();
    for clique in rx {
        if best_clique.len() < clique.len() {
            best_clique = clique;
        }
    }

    let mut computers = Vec::new();
    for x in best_clique {
        computers.push(idx_to_computer.get(x).unwrap().clone());
    }
    computers.sort();

    println!("{}", computers.join(","));
}
