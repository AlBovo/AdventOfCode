use std::fs;
use std::error::Error;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\(\d+,\d+\)").unwrap();
    let code = fs::read_to_string("input.txt")?;

    let mut total = 0;
    let mut isOk = true;
    for string in re.find_iter(&code) {
        if isOk && string.as_str().starts_with("mul") {
            let reg = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

            if let Some(caps) = reg.captures(string.as_str()) {
                total += caps[1].parse::<i32>().unwrap() * caps[2].parse::<i32>().unwrap();
            }
        }
        else {
            isOk = string.as_str() == "do()";
        }
    }

    println!("{}", total);
    Ok(())
}