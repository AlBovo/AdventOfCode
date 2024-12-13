use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let lines = read_to_string("input.txt").unwrap();
    let lines = lines.lines().collect::<Vec<&str>>();
    let xys = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
    let prize = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let mut i = 0;
    let mut total: i64 = 0;
    while i < lines.len() {
        let Some(caps) = xys.captures(lines[i]) else { println!("check the regex"); break; };
        let (x1, y1) = (caps[1].parse::<i64>().unwrap(), caps[2].parse::<i64>().unwrap());
        let Some(caps) = xys.captures(lines[i+1]) else { println!("check the regex"); break; };
        let (x2, y2) = (caps[1].parse::<i64>().unwrap(), caps[2].parse::<i64>().unwrap());

        let Some(caps) = prize.captures(lines[i+2]) else { println!("check the regex"); break; };
        let (x3, y3) = (caps[1].parse::<i64>().unwrap() + 10000000000000, caps[2].parse::<i64>().unwrap() + 10000000000000);

        let x = (y3 * x2 - x3 * y2) / (y1 * x2 - x1 * y2);
        let y = (x3 - x1 * x) / x2;

        if x * x1 + y * x2 == x3 && x * y1 + y * y2 == y3 {
            total += x * 3 + y;
        }

        i += 4;
    }
    println!("{}", total);
}
