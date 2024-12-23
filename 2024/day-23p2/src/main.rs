use std::collections::HashMap;
use std::fs::read_to_string;

static mut MAXIMUM: Vec<String> = Vec::new();

// bruteforce to run on release lol
fn find(pos: usize, taken: &mut Vec<String>, map: &HashMap<String, Vec<String>>) {
    unsafe {
        if MAXIMUM.len() > taken.len() + map.keys().len() - pos { return }
    }
    if pos == map.keys().len() {
        unsafe {
            if MAXIMUM.len() < taken.len() {
                MAXIMUM = taken.clone();
            }
        }
        return;
    }
    if taken.iter().all(|x| {
        let key = map.keys().nth(pos).unwrap(); // Get the key at position `pos`
        map.get(key).unwrap().contains(&x)
    }) {
        taken.push((&map.keys().collect::<Vec<_>>()[pos]).to_string());
        find(pos+1, taken, &map);
        taken.pop();
    }
    find(pos+1, taken, &map);
}

fn main() {
    let lines = read_to_string("input.txt").unwrap();
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines.lines() {
        let ab = line.split('-').collect::<Vec<&str>>();

        let a = ab[0].to_string();
        let b = ab[1].to_string();

        map.entry(a.clone()).or_insert_with(Vec::new).push(b.clone());
        map.entry(b).or_insert_with(Vec::new).push(a);
    }

    find(0, &mut Vec::new(), &map);
    unsafe {
        MAXIMUM.sort();
        for val in &MAXIMUM {
            print!("{},", val);
        }
    }
}