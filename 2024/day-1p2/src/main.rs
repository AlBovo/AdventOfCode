use std::fs::read_to_string;
use std::i32;
use std::str::FromStr;

fn main() {
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    let mut _idx = 0;
    for line in read_to_string("input.txt").unwrap().lines() {
        let vals = line.split("   ").collect::<Vec<&str>>();

        vec1.push(i32::from_str(vals[0]).unwrap());
        vec2.push(i32::from_str(vals[1]).unwrap());
        _idx += 1;
    }

    let mut total : i64 = 0;
    for i in 0..vec1.len() {
        let mut tot = 0;
        for e in 0..vec2.len() {
            tot += (vec2[e] == vec1[i]) as i64;
        }
        total += vec1[i] as i64 * tot;
    }
    println!("{}", total);
}
