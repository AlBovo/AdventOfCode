use std::fs::read_to_string;
use std::process::exit;

fn get_combo(op: &str, a: i64, b: i64, c: i64) -> i64 {
    match op {
        "0" | "1" | "2" | "3" => op.parse().unwrap(),
        "4" => a,
        "5" => b,
        "6" => c,
        _ => panic!("Invalid instruction: {}", op)
    }
}

fn main() {
    // I'll rewrite the input parser later
    let lines = read_to_string("input.txt").unwrap();
    let line = lines.lines().next().unwrap().split(',').collect::<Vec<&str>>();
    let program = line[3..].iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");

    // I know, that's the worst crap I could write
    for i in 1e16 as i64 ..1e17 as i64 {
        let mut a: i64 = i;
        let mut b: i64 = line[1].parse().unwrap();
        let mut c: i64 = line[2].parse().unwrap();

        let mut res: Vec<usize> = Vec::new();
        let mut ip = 3;
        loop {
            if res.len() > line.len() - 3 { break; }

            assert!(ip >= 3);
            if ip == line.len() { break; }

            match line[ip] {
                "0" => { // adv
                    let x: i64 = get_combo(line[ip + 1], a, b, c);
                    a >>= x
                }
                "1" => { // bxl
                    let x: i64 = line[ip + 1].parse().unwrap();
                    b ^= x
                }
                "2" => { // bst
                    let x: i64 = get_combo(line[ip + 1], a, b, c);
                    b = x % 8
                }
                "3" => { // jnz
                    if a != 0 {
                        ip = line[ip + 1].parse::<usize>().unwrap() + 3;
                        ip -= 2; // to prevent the sum at the end
                    }
                }
                "4" => { // bxc
                    b = b ^ c
                }
                "5" => { // out
                    // print!("{} ", get_combo(line[ip + 1], a, b, c));
                    res.push(get_combo(line[ip + 1], a, b, c) as usize % 8);
                }
                "6" => { // bdv
                    let x: i64 = get_combo(line[ip + 1], a, b, c);
                    b = a >> x
                }
                "7" => { // cdv
                    let x: i64 = get_combo(line[ip + 1], a, b, c);
                    c = a >> x
                }
                _ => panic!("Invalid instruction: {}", line[ip])
            }

            ip += 2;
        }

        let string = res.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
        if program == string {
            println!("{}", i);
            exit(0);
        }
    }

}
