use std::fs::read_to_string;

// idk how to set a global value
fn find_paths(pos: (usize, usize), matrix: &Vec<Vec<u8>>, vis: &mut Vec<Vec<bool>>) -> i32 {
    if vis[pos.0][pos.1] { return 0; }
    if matrix[pos.0][pos.1] == 9 {
        if vis[pos.0][pos.1] { return 0; }
        vis[pos.0][pos.1] = true;
        return 1;
    }

    vis[pos.0][pos.1] = true;
    let mut total = 0;
    if pos.0 < matrix.len() - 1 &&
        matrix[pos.0+1][pos.1] == matrix[pos.0][pos.1] + 1 {
        total += find_paths((pos.0+1, pos.1), matrix, vis);
    }
    if pos.0 > 0 &&
        matrix[pos.0-1][pos.1] == matrix[pos.0][pos.1] + 1 {
        total += find_paths((pos.0-1, pos.1), matrix, vis);
    }
    if pos.1 < matrix[0].len() - 1 &&
        matrix[pos.0][pos.1+1] == matrix[pos.0][pos.1] + 1 {
        total += find_paths((pos.0, pos.1+1), matrix, vis);
    }
    if pos.1 > 0 &&
        matrix[pos.0][pos.1-1] == matrix[pos.0][pos.1] + 1 {
        total += find_paths((pos.0, pos.1-1), matrix, vis);
    }

    total
}

fn main() {
    let lines = read_to_string("input.txt").unwrap();
    let matrix: Vec<Vec<u8>> = lines
        .lines()
        .map(|l| l.bytes().map(|b| b - b'0').collect())
        .collect();
    let mut pos: Vec<(usize, usize)> = Vec::new();
    let mut vis: Vec<Vec<bool>> = Vec::new();

    for i in 0..matrix.len() {
        vis.push(Vec::new());
        for j in 0..matrix[0].len() {
            if matrix[i][j] == 0 {
                pos.push((i, j));
            }
            vis[i].push(false);
        }
    }

    let mut total: i64 = 0;
    for i in pos {
        let mut vec = vis.clone();
        let ris = find_paths(i, &matrix, &mut vec) as i64;
        total += ris;
    }
    println!("{}", total);
}
