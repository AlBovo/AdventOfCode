use std::fs::read_to_string;
use regex::Regex;

const GRID_SIZE_X: i32 = 101;
const GRID_SIZE_Y: i32 = 103;

fn negative_modulo(x: i32, y: i32) -> i32 {
    ((x % y) + y) % y
}

fn main() {
    let lines = read_to_string("input.txt").unwrap();
    let lines = lines.lines().collect::<Vec<&str>>();
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

    let mut safety = (0, 0, 0, 0);

    for i in lines {
        let Some(caps) = re.captures(i) else { println!("check the regex"); break; };
        let mut vals: Vec<i32> = Vec::new();
        for e in 1..5 {
            vals.push(caps[e].parse::<i32>().unwrap());
        }

        let x = negative_modulo(vals[0] + vals[2] * 100, GRID_SIZE_X);
        let y = negative_modulo(vals[1] + vals[3] * 100, GRID_SIZE_Y);

        if x < GRID_SIZE_X / 2 && y < GRID_SIZE_Y / 2 {
            safety.0 += 1;
        }
        else if x > GRID_SIZE_X / 2 && y < GRID_SIZE_Y / 2 {
            safety.1 += 1;
        }
        else if x < GRID_SIZE_X / 2 && y > GRID_SIZE_Y / 2 {
            safety.2 += 1;
        }
        else if x > GRID_SIZE_X / 2 && y > GRID_SIZE_Y / 2 {
            safety.3 += 1;
        }
    }
    println!("{}", safety.0 * safety.1 * safety.2 * safety.3);
}