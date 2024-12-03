use std::fs;
use std::error::Error;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let code = fs::read_to_string("input.txt")?;

    let mut total = 0;
    for (_, [mul1, mul2]) in re.captures_iter(&code).map(|c| c.extract()) {
        total += mul1.parse::<i32>().unwrap() * mul2.parse::<i32>().unwrap();
    }

    println!("{}", total);
    Ok(())
}