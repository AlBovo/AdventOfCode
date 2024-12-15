use std::fs::read_to_string;

fn get_direction(direction: usize) -> (i32, bool) {
    match direction {
        0 => (-1, true),
        1 => (1, false),
        2 => (1, true),
        3 => (-1, false),
        _ => panic!("Unknown direction: {}", direction)
    }
}

fn move_all(pos: (usize, usize), direction: usize, grid: &mut Vec<Vec<char>>) {
    if grid[pos.0][pos.1] == '#' { return; }
    if grid[pos.0][pos.1] == '.' { return; }
    let offset = get_direction(direction);
    if offset.1 { // vertical movements
        if grid[pos.0][pos.1] == ']' {
            move_all(((pos.0 as i32 + offset.0) as usize, pos.1), direction, grid);
            move_all(((pos.0 as i32 + offset.0) as usize, pos.1 - 1), direction, grid);

            grid[pos.0][pos.1] = '.';
            grid[pos.0][pos.1-1] = '.';
            grid[(pos.0 as i32 + offset.0) as usize][pos.1] = ']';
            grid[(pos.0 as i32 + offset.0) as usize][pos.1-1] = '[';
        }
        else if grid[pos.0][pos.1] == '[' {
            move_all(((pos.0 as i32 + offset.0) as usize, pos.1), direction, grid);
            move_all(((pos.0 as i32 + offset.0) as usize, pos.1 + 1), direction, grid);

            grid[pos.0][pos.1] = '.';
            grid[pos.0][pos.1+1] = '.';
            grid[(pos.0 as i32 + offset.0) as usize][pos.1] = '[';
            grid[(pos.0 as i32 + offset.0) as usize][pos.1+1] = ']';
        }
    }
    else { // horizontal movements
        move_all((pos.0, (pos.1 as i32 + offset.0*2) as usize), direction, grid);
        grid[pos.0][pos.1] = '.';
        grid[pos.0][(pos.1 as i32 + offset.0) as usize] = if offset.0 == 1 { '[' } else { ']' };
        grid[pos.0][(pos.1 as i32 + offset.0 * 2) as usize] = if offset.0 == 1 { ']' } else { '[' };
    }
}

/// 0 -> up
/// 1 -> right
/// 2 -> down
/// 3 -> left
fn move_robot(pos: (usize, usize), direction: usize, grid: &mut Vec<Vec<char>>) -> bool {
    if grid[pos.0][pos.1] == '#' { return false; }
    if grid[pos.0][pos.1] == '.' { return true; }
    let offset = get_direction(direction);

    if offset.1 { // vertical movements
        if grid[pos.0][pos.1] == ']' {
            move_robot(((pos.0 as i32 + offset.0) as usize, pos.1), direction, grid) &&
            move_robot(((pos.0 as i32 + offset.0) as usize, pos.1 - 1), direction, grid)
        } else {
            move_robot(((pos.0 as i32 + offset.0) as usize, pos.1), direction, grid) &&
            move_robot(((pos.0 as i32 + offset.0) as usize, pos.1 + 1), direction, grid)
        }
    } else { // horizontal movements
        move_robot((pos.0, (pos.1 as i32 + offset.0 * 2) as usize), direction, grid)
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
            grid.push(Vec::new());
            for i in line.1.chars().enumerate() {
                match i.1 {
                    '.' => {
                        grid[line.0].push('.');
                        grid[line.0].push('.');
                    }
                    '#' => {
                        grid[line.0].push('#');
                        grid[line.0].push('#');
                    }
                    'O' => {
                        grid[line.0].push('[');
                        grid[line.0].push(']');
                    }
                    '@' => {
                        grid[line.0].push('@');
                        grid[line.0].push('.');
                        pos = (line.0, i.0 * 2);
                    }
                    _ => { panic!("Unknown cell: {:?}", i); }
                }
            }
        }
        else{
            moves += line.1;
        }
    }
    assert_eq!(grid[pos.0][pos.1], '@');
    for i in &grid {
        for j in i {
            print!("{}", j);
        }
        println!();
    }
    println!();
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
            '[' | ']' => {
                if move_robot(new_pos.0, new_pos.1, &mut grid) {
                    move_all(new_pos.0, new_pos.1, &mut grid);
                    grid[pos.0][pos.1] = '.';
                    grid[new_pos.0.0][new_pos.0.1] = '@';
                    pos = new_pos.0;
                }
            }
            _ => { panic!("Unknown cell: {}", grid[new_pos.0.0][new_pos.0.1]); }
        }
    }
    for i in &grid {
        for j in i {
            print!("{}", j);
        }
        println!();
    }
    println!();
    let mut total: i64 = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '[' {
                total += (i as i64 * 100) + j as i64;
            }
        }
    }
    println!("{}", total);
}