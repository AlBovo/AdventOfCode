use std::fs::read_to_string;
use std::collections::HashMap;

const SIZES : (i32, i32) = (130, 130);

fn going_out(position : (i32, i32), direction : i32) -> bool {
    match direction {
        0 => position.0 == 0,
        1 => position.1 == SIZES.1-1,
        2 => position.0 == SIZES.0-1,
        3 => position.1 == 0,
        _ => true
    }
}

fn is_loop(position : (i32, i32), mut direction : i32, matrix: &Vec<Vec<char>>) -> bool {
    let mut x = position.0;
    let mut y = position.1;
    let mut visited : HashMap<(i32,i32,i32), bool> = HashMap::new();

    while !going_out((x, y), direction) {
        let mut next = '#';
        if !visited.contains_key(&(x, y, direction)) {
            visited.insert((x, y, direction), true);
        }
        else { return true; }

        while next == '#' {
            next = match direction {
                0 => matrix[(x - 1) as usize][y as usize],
                1 => matrix[x as usize][(y + 1) as usize],
                2 => matrix[(x + 1) as usize][y as usize],
                3 => matrix[x as usize][(y - 1) as usize],
                _ => continue
            };
            if next == '#' {
                direction = (direction + 1) % 4;
            }
        }
        match direction {
            0 => x -= 1,
            1 => y += 1,
            2 => x += 1,
            3 => y -= 1,
            _ => continue
        };
    }
    false
}

fn main() {
    let lines = read_to_string("input.txt").unwrap();
    let mut matrix : Vec<Vec<char>> = lines.lines().map(|l| l.chars().collect()).collect();

    let mut x : i32 = 0;
    let mut y : i32 = 0;
    let mut total : i64 = 0;

    for i in 0..matrix.len() {
        for e in 0..matrix[i].len() {
            if matrix[i][e] == '^' {
                x = i as i32;
                y = e as i32;
                break;
            }
        }
    }

    // horrendous bruteforce
    for i in 0..matrix.len() {
        for e in 0..matrix[i].len() {
            if matrix[i][e] == '.' {
                matrix[i][e] = '#';
                total += if is_loop((x,y), 0, &matrix) { 1 } else { 0 };
                matrix[i][e] = '.';
            }
        }
    }

    println!("{}", total);
}