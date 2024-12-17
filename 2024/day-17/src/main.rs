use std::fs::read_to_string;

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
    println!("{:?}", line);

    let mut a: i64 = line[0].parse().unwrap();
    let mut b: i64 = line[1].parse().unwrap();
    let mut c: i64 = line[2].parse().unwrap();

    let mut ip = 3;
    loop {
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
                print!("{},", get_combo(line[ip + 1], a, b, c) % 8);
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
}
