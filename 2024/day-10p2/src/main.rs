use std::fs::read_to_string;

// idk how to set a global value
fn find_paths(pos: (usize, usize), dp: &mut Vec<Vec<i32>>, matrix: &Vec<Vec<u8>>) -> i32 {
    if matrix[pos.0][pos.1] == 9 { return 1; }
    if dp[pos.0][pos.1] != -1 { return dp[pos.0][pos.1]; }

    let mut total = 0;
    if pos.0 < matrix.len() - 1 &&
        matrix[pos.0+1][pos.1] == matrix[pos.0][pos.1] + 1 {
        total += find_paths((pos.0+1, pos.1), dp, matrix);
    }
    if pos.0 > 0 &&
        matrix[pos.0-1][pos.1] == matrix[pos.0][pos.1] + 1 {
        total += find_paths((pos.0-1, pos.1), dp, matrix);
    }
    if pos.1 < matrix[0].len() - 1 &&
        matrix[pos.0][pos.1+1] == matrix[pos.0][pos.1] + 1 {
        total += find_paths((pos.0, pos.1+1), dp, matrix);
    }
    if pos.1 > 0 &&
        matrix[pos.0][pos.1-1] == matrix[pos.0][pos.1] + 1 {
        total += find_paths((pos.0, pos.1-1), dp, matrix);
    }
    dp[pos.0][pos.1] = total;

    total
}

fn main() {
    let lines = read_to_string("input.txt").unwrap();
    let matrix: Vec<Vec<u8>> = lines
        .lines()
        .map(|l| l.bytes().map(|b| b - b'0').collect())
        .collect();
    let mut pos: Vec<(usize, usize)> = Vec::new();
    let mut dp: Vec<Vec<i32>> = Vec::new();

    for i in 0..matrix.len() {
        dp.push(Vec::new());
        for j in 0..matrix[0].len() {
            if matrix[i][j] == 0 {
                pos.push((i, j));
            }
            dp[i].push(-1);
        }
    }

    let mut total: i64 = 0;
    for i in pos {
        let ris = find_paths(i, &mut dp, &matrix) as i64;
        total += ris;
    }
    println!("{}", total);
}
