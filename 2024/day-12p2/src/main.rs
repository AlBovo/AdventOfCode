use std::fs::read_to_string;

// i don't know if im really proud of this but it works so yay
fn visit(pos: (usize, usize), grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<(bool, bool, bool, bool, bool)>>) -> (i32, i32) {
    let c = grid[pos.0][pos.1];
    let mut flag = (0, 0);
    let mut new_pos: (usize, usize);

    if grid[pos.0-1][pos.1] != c && !visited[pos.0][pos.1].0 {
        visited[pos.0][pos.1].0 = true;
        
        new_pos = (pos.0, pos.1-1);
        while grid[new_pos.0][new_pos.1] == c &&
            !visited[new_pos.0][new_pos.1].0 &&
            grid[new_pos.0-1][new_pos.1] != c {
            
            visited[new_pos.0][new_pos.1].0 = true;
            new_pos = (new_pos.0, new_pos.1-1);
        }
        new_pos = (pos.0, pos.1+1);
        while grid[new_pos.0][new_pos.1] == c &&
            !visited[new_pos.0][new_pos.1].0 &&
            grid[new_pos.0-1][new_pos.1] != c {
            
            visited[new_pos.0][new_pos.1].0 = true;
            new_pos = (new_pos.0, new_pos.1+1);
        }
        flag.0 += 1;
    }
    if grid[pos.0+1][pos.1] != c && !visited[pos.0][pos.1].2 {
        visited[pos.0][pos.1].2 = true;
        
        new_pos = (pos.0, pos.1-1);
        while grid[new_pos.0][new_pos.1] == c &&
            !visited[new_pos.0][new_pos.1].2 &&
            grid[new_pos.0+1][new_pos.1] != c {
            
            visited[new_pos.0][new_pos.1].2 = true;
            new_pos = (new_pos.0, new_pos.1-1);
        }
        new_pos = (pos.0, pos.1+1);
        while grid[new_pos.0][new_pos.1] == c &&
            !visited[new_pos.0][new_pos.1].2 &&
            grid[new_pos.0+1][new_pos.1] != c {
            
            visited[new_pos.0][new_pos.1].2 = true;
            new_pos = (new_pos.0, new_pos.1+1);
        }
        flag.0 += 1;
    }
    if grid[pos.0][pos.1+1] != c && !visited[pos.0][pos.1].1 {
        visited[pos.0][pos.1].1 = true;
        
        new_pos = (pos.0-1, pos.1);
        while grid[new_pos.0][new_pos.1] == c &&
            !visited[new_pos.0][new_pos.1].1 &&
            grid[new_pos.0][new_pos.1+1] != c {
            
            visited[new_pos.0][new_pos.1].1 = true;
            new_pos = (new_pos.0-1, new_pos.1);
        }
        new_pos = (pos.0+1, pos.1);
        while grid[new_pos.0][new_pos.1] == c &&
            !visited[new_pos.0][new_pos.1].1 &&
            grid[new_pos.0][new_pos.1+1] != c {
            
            visited[new_pos.0][new_pos.1].1 = true;
            new_pos = (new_pos.0+1, new_pos.1);
        }
        flag.0 += 1;
    }
    if grid[pos.0][pos.1-1] != c && !visited[pos.0][pos.1].3 {
        visited[pos.0][pos.1].3 = true;
        
        new_pos = (pos.0-1, pos.1);
        while grid[new_pos.0][new_pos.1] == c &&
            !visited[new_pos.0][new_pos.1].3 &&
            grid[new_pos.0][new_pos.1-1] != c {
            
            visited[new_pos.0][new_pos.1].3 = true;
            new_pos = (new_pos.0-1, new_pos.1);
        }
        new_pos = (pos.0+1, pos.1);
        while grid[new_pos.0][new_pos.1] == c &&
            !visited[new_pos.0][new_pos.1].3 &&
            grid[new_pos.0][new_pos.1-1] != c {
            
            visited[new_pos.0][new_pos.1].3 = true;
            new_pos = (new_pos.0+1, new_pos.1);
        }
        flag.0 += 1;
    }

    if !visited[pos.0][pos.1].4 {
        flag.1 = 1;
        visited[pos.0][pos.1].4 = true;
        if grid[pos.0-1][pos.1] == c {
            let res = visit((pos.0-1, pos.1), grid, visited);
            flag.0 += res.0;
            flag.1 += res.1;
        }
        if grid[pos.0+1][pos.1] == c {
            let res = visit((pos.0+1, pos.1), grid, visited);
            flag.0 += res.0;
            flag.1 += res.1;
        }
        if grid[pos.0][pos.1-1] == c {
            let res = visit((pos.0, pos.1-1), grid, visited);
            flag.0 += res.0;
            flag.1 += res.1;
        }
        if grid[pos.0][pos.1+1] == c {
            let res = visit((pos.0, pos.1+1), grid, visited);
            flag.0 += res.0;
            flag.1 += res.1;
        }
    }

    flag
}

fn main() {
    let lines = read_to_string("input.txt").unwrap();
    let mut mat: Vec<Vec<char>> = lines.lines().map(|x| x.chars().collect()).collect();
    // (up, right, down, left)
    let mut visited = vec![vec![(false, false, false, false, false); mat[0].len()+2]; mat.len()+2];

    for i in 0..mat.len() {
        mat[i].insert(0, ' ');
        mat[i].push(' ');
    }
    mat.insert(0, vec![' '; mat[1].len()]);
    mat.push(vec![' '; mat[0].len()]);

    let mut total: i64 = 0;
    for i in 0..mat.len() {
        for j in 0..mat[0].len() {
            if mat[i][j] == ' ' { continue }
            let res = visit((i, j), &mat, &mut visited);
            // eprintln!("{:?}", res);
            total += res.0 as i64 * res.1 as i64;
        }
    }

    println!("{}", total);
}
