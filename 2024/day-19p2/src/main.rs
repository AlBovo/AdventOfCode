use std::collections::HashMap;
use std::fs::read_to_string;

fn is_possible(s: &str, patterns: &Vec<&str>, dp: &mut HashMap<String, i64>) -> i64 {
    if s.is_empty() { return 1 }
    if dp.contains_key(s) { return dp[s] }

    let mut res = 0;
    for pattern in patterns {
        if s.starts_with(pattern) {
            res += is_possible(&s[pattern.len()..], patterns, dp)
        }
    }
    dp.insert(s.to_string(), res);
    res
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let lines = input.split('\n').collect::<Vec<&str>>();
    let patterns = lines[0].split(", ").collect::<Vec<&str>>();
    let mut dp: HashMap<String, i64> = HashMap::new();

    let mut total: i64 = 0;
    for line in lines.into_iter().skip(2) {
        total += is_possible(line, &patterns, &mut dp);
    }
    println!("{}", total);
}
