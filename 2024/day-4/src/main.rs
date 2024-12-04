use std::fs::read_to_string;

fn main() {
    let lines = read_to_string("input.txt").unwrap();
    let matrix : Vec<Vec<char>> = lines.lines().map(|l| l.chars().collect()).collect();
    let a = matrix.len();
    let b = matrix[0].len();

    let mut total = 0;
    // horizontally
    for i in 0..a {
        for j in 0..b-3 {
            let v = format!("{}{}{}{}",
                matrix[i][j], matrix[i][j+1], matrix[i][j+2], matrix[i][j+3]);
            if v == "XMAS" || v == "SAMX" {
                total += 1;
            }
        }
    }

    // vertically
    for i in 0..a-3 {
        for j in 0..b {
            let v = format!("{}{}{}{}",
                matrix[i][j], matrix[i+1][j], matrix[i+2][j], matrix[i+3][j]);
            if v == "XMAS" || v == "SAMX" {
                total += 1;
            }
        }
    }

    // diagonally /
    for i in 0..a-3 {
        for j in 3..b {
            let v = format!("{}{}{}{}",
                matrix[i][j], matrix[i+1][j-1], matrix[i+2][j-2], matrix[i+3][j-3]);
            if v == "XMAS" || v == "SAMX" {
                total += 1;
            }
        }
    }

    // diagonally \
    for i in 0..a-3 {
        for j in 0..b-3 {
            let v = format!("{}{}{}{}",
                matrix[i][j], matrix[i+1][j+1], matrix[i+2][j+2], matrix[i+3][j+3]);
            if v == "XMAS" || v == "SAMX" {
                total += 1;
            }
        }
    }

    println!("{}", total);
}