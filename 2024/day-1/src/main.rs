use std::fs::read_to_string;
use std::i32;
use std::str::FromStr;

fn main() {
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    let mut idx = 0;
    for line in read_to_string("input.txt").unwrap().lines() {
        let vals = line.split("   ").collect::<Vec<&str>>();

        vec1.push((i32::from_str(vals[0]).unwrap(), idx));
        vec2.push((i32::from_str(vals[1]).unwrap(), idx));
        idx += 1;
    }

    vec1.sort();
    vec2.sort();

    let mut total = 0;
    for i in 0..vec1.len() {
        total += i32::abs(vec1[i].0 - vec2[i].0);
    }
    println!("{}", total);
    // vec1.iter().for_each(|x| println!("{:?}", x));
}
