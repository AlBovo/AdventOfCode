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

    let mut robots: Vec<(i32, i32, i32, i32)> = Vec::new();

    for i in lines {
        let Some(caps) = re.captures(i) else { println!("check the regex"); break; };
        robots.push((
            caps[1].parse::<i32>().unwrap(),
            caps[2].parse::<i32>().unwrap(),
            caps[3].parse::<i32>().unwrap(),
            caps[4].parse::<i32>().unwrap(),
        ))
    }

    let mut grid: Vec<Vec<usize>> = vec![vec![0; GRID_SIZE_X as usize]; GRID_SIZE_Y as usize];
    for i in &robots {
        grid[negative_modulo(i.1, GRID_SIZE_Y) as usize][negative_modulo(i.0, GRID_SIZE_X) as usize] += 1;
    }

    let mut total = 0;
    loop {
        for i in &grid {
            for j in i {
                print!("{}", if j >= &1 { "#" } else { "." });
            }
            println!();
        }
        println!("{}\n\n", total);

        let mut buff: &mut String = &mut String::new();
        std::io::stdin().read_line(buff).expect("skill issue");
        if buff != "\n" { break; }

        for i in &robots {
            let previous_x = negative_modulo(i.0 + i.2 * total, GRID_SIZE_X) as usize;
            let previous_y = negative_modulo(i.1 + i.3 * total, GRID_SIZE_Y) as usize;

            let new_x = negative_modulo(i.0 + i.2 * (total + 1), GRID_SIZE_X) as usize;
            let new_y = negative_modulo(i.1 + i.3 * (total + 1), GRID_SIZE_Y) as usize;

            grid[previous_y][previous_x] -= 1;
            grid[new_y][new_x] += 1;
        }
        total += 1;
    }
}