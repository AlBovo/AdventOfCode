use std::fs::read_to_string;

/// 0 -> up
/// 1 -> right
/// 2 -> down
/// 3 -> left
fn move_robot(pos: (usize, usize), direction: usize, grid: &mut Vec<Vec<char>>) -> bool {
    let new_pos =
        match direction {
            0 => (pos.0-1, pos.1),
            1 => (pos.0, pos.1+1),
            2 => (pos.0+1, pos.1),
            3 => (pos.0, pos.1-1),
            _ => { panic!("Unknown direction: {}", direction); }
        };

    match grid[new_pos.0][new_pos.1] {
        '.' => {
            grid[new_pos.0][new_pos.1] = 'O';
            grid[pos.0][pos.1] = '.';
            true
        }
        '#' => false,
        'O' => {
            if move_robot(new_pos, direction, grid) {
                grid[new_pos.0][new_pos.1] = 'O';
                grid[pos.0][pos.1] = '.';
                return true
            }
            false
        }
        _ => { panic!("Unknown move: {}", grid[new_pos.0][new_pos.1]); }
    }
}

fn main() {
    let lines = read_to_string("input.txt").unwrap();
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut moves: String = String::new();
    let mut pos: (usize, usize) = (0, 0);

    let mut flag = true;
    for line in lines.split("\n").enumerate() {
        if line.1.is_empty() { flag = false; continue; }
        if flag {
            let res;
            if pos == (0, 0) && { res = line.1.find('@'); res }.is_some() {
                pos = (line.0, res.unwrap());
            }
            grid.push(line.1.chars().collect());
        }
        else{
            moves += line.1;
        }
    }

    for i in moves.chars() {
        let new_pos = match i {
            '^' => ((pos.0-1, pos.1), 0),
            '>' => ((pos.0, pos.1+1), 1),
            'v' => ((pos.0+1, pos.1), 2),
            '<' => ((pos.0, pos.1-1), 3),
            _ => { panic!("Unknown move: {}", i); }
        };
        match grid[new_pos.0.0][new_pos.0.1] {
            '.' => {
                grid[new_pos.0.0][new_pos.0.1] = '@';
                grid[pos.0][pos.1] = '.';
                pos = new_pos.0;
            }
            '#' => { continue }
            'O' => {
                if move_robot(new_pos.0, new_pos.1, &mut grid) {
                    grid[pos.0][pos.1] = '.';
                    grid[new_pos.0.0][new_pos.0.1] = '@';
                    pos = new_pos.0;
                }
            }
            _ => { panic!("Unknown cell: {}", grid[new_pos.0.0][new_pos.0.1]); }
        }
        // for i in &grid {
        //     for j in i {
        //         print!("{}", j);
        //     }
        //     println!();
        // }
        // println!();
    }

    let mut total: i64 = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'O' {
                total += (i as i64 * 100) + j as i64;
            }
        }
    }
    println!("{}", total);
}
