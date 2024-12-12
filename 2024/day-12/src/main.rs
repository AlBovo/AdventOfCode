use std::fs::read_to_string;

// returns (num-cells, num-fences)
fn visit(pos: (usize, usize), mat: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>) -> (i64, i64) {
    assert!(!visited[pos.0][pos.1]);
    visited[pos.0][pos.1] = true;
    let mut result = (1, 4);

    if mat[pos.0-1][pos.1] == mat[pos.0][pos.1] {
        if !visited[pos.0-1][pos.1] {
            let (a, b) = visit((pos.0-1, pos.1), mat, visited);
            result.0 += a;
            result.1 += b;
        }
        result.1 -= 1;
    }

    if mat[pos.0][pos.1-1] == mat[pos.0][pos.1] {
        if !visited[pos.0][pos.1-1] {
            let (a, b) = visit((pos.0, pos.1-1), mat, visited);
            result.0 += a;
            result.1 += b;
        }
        result.1 -= 1;
    }

    if mat[pos.0+1][pos.1] == mat[pos.0][pos.1] {
        if !visited[pos.0+1][pos.1] {
            let (a, b) = visit((pos.0+1, pos.1), mat, visited);
            result.0 += a;
            result.1 += b;
        }
        result.1 -= 1;
    }

    if mat[pos.0][pos.1+1] == mat[pos.0][pos.1] {
        if !visited[pos.0][pos.1+1] {
            let (a, b) = visit((pos.0, pos.1+1), mat, visited);
            result.0 += a;
            result.1 += b;
        }
        result.1 -= 1;
    }

    result
}

fn main() {
    let lines = read_to_string("input.txt").unwrap();
    let mut mat: Vec<Vec<char>> = lines.lines().map(|l| l.chars().collect()).collect();

    // add side Xs to pad and remove side cases
    mat.push(vec!['x'; mat[0].len()]);
    mat.insert(0, vec!['x'; mat[0].len()]);
    for i in 0..mat.len() {
        mat[i].push('x');
        mat[i].insert(0, 'x');
    }

    let mut visited: Vec<Vec<bool>> = vec![vec![false; mat[0].len()]; mat.len()];
    let mut total: i64 = 0;

    for i in 0..mat.len() {
        for j in 0..mat[0].len() {
            if mat[i][j] == 'x' || visited[i][j] {
                continue;
            }
            let (a, b) = visit((i, j), &mat, &mut visited);
            total += a * b;
        }
    }

    println!("{}", total);
}
