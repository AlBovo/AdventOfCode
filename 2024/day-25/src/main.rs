use std::fs::read_to_string;

fn main() {
    let lines = read_to_string("input.txt").unwrap();

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    let mut buff: Vec<&str> = Vec::new();
    for line in lines.lines() {
        if line.is_empty() {
            buff = Vec::new();
            continue
        }
        buff.push(line);
        if buff.len() == 7 {
            if buff[0] == "#".repeat(5) {
                locks.push(buff.clone());
            } else {
                keys.push(buff.clone());
            };
        }
    }

    let mut locks_vals = Vec::new();
    let mut keys_vals = Vec::new();

    for key in keys {
        let mut val = vec![0; 5];
        for i in key.iter().rev().skip(1) {
            for j in i.to_string().chars().enumerate() {
                val[j.0] += (j.1 == '#') as usize;
            }
        }
        keys_vals.push(val);
    }
    for lock in locks {
        let mut val = vec![0; 5];
        for i in lock.iter().skip(1) {
            for j in i.to_string().chars().enumerate() {
                val[j.0] += (j.1 == '#') as usize;
            }
        }
        locks_vals.push(val);
    }

    let mut total = 0;
    for lock in locks_vals.iter() {
        for key in keys_vals.iter() {
            let mut flag = true;
            for i in 0..5 {
                flag &= lock[i] + key[i] <= 5
            }
            if flag { total += 1 }
        }
    }
    println!("{}", total);
}