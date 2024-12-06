use std::fs::read_to_string;

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

fn main() {
    let lines = read_to_string("input.txt").unwrap();
    let matrix : Vec<Vec<char>> = lines.lines().map(|l| l.chars().collect()).collect();
    let mut visited : Vec<Vec<bool>> = vec![vec![false; matrix[0].len()]; matrix.len()];

    let mut x = 0;
    let mut y = 0;
    let mut direction = 0; // 0 : up, 1 : right, 2 : down, 3 : left
    let mut total : i64 = 0;

    for i in 0..matrix.len() {
        for e in 0..matrix[i].len() {
            if matrix[i][e] == '^' {
                x = i;
                y = e;
            }
        }
    }

    while !going_out((x as i32, y as i32), direction) {
        let mut next = '#';
        if !visited[x][y] {
            visited[x][y] = true;
            total += 1;
        }

        while next == '#' {
            next = match direction {
                0 => matrix[x - 1][y],
                1 => matrix[x][y + 1],
                2 => matrix[x + 1][y],
                3 => matrix[x][y - 1],
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
    println!("{}", total+1);
}