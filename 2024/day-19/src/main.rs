use std::fs::read_to_string;

fn is_possible(s: &str, patterns: &Vec<&str>) -> bool {
    if s.is_empty() { return true }

    let mut res = false;
    for pattern in patterns {
        if res { break }
        if s.starts_with(pattern) {
            res |= is_possible(&s[pattern.len()..], patterns)
        }
    }
    res
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let lines = input.split('\n').collect::<Vec<&str>>();
    let patterns = lines[0].split(", ").collect::<Vec<&str>>();

    let mut total = 0;
    for line in lines.into_iter().skip(2) {
        total += is_possible(line, &patterns) as i32;
    }
    println!("{}", total);
}
