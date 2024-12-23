use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let lines = read_to_string("input.txt").unwrap();
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in lines.lines() {
        let ab = line.split("-").collect::<Vec<&str>>();
        if !map.contains_key(ab[0]) {
            map.insert(ab[0], Vec::new());
        }
        map.get_mut(ab[0]).unwrap().push(ab[1]);
        if !map.contains_key(ab[1]) {
            map.insert(ab[1], Vec::new());
        }
        map.get_mut(ab[1]).unwrap().push(ab[0]);
    }

    let mut total = 0;
    for key in map.keys() {
        let v = map.get(key).unwrap();
        for i in 0..v.len() {
            if v[i] > key { continue }
            for e in i+1..v.len() {
                if v[e] > key { continue }
                if map[v[i]].contains(&v[e]) &&
                    (key.starts_with('t') || v[i].starts_with('t') || v[e].starts_with('t')) {
                    total += 1;
                }
            }
        }
    }

    println!("{}", total);
}