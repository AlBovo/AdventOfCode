use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    let lines = read_to_string("input.txt").unwrap();
    let mut matrix : Vec<Vec<char>> = lines.lines().map(|l| l.chars().collect()).collect();
    let mut positions : HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] != '.' {
                if positions.contains_key(&matrix[i][j]) {
                    positions.get_mut(&matrix[i][j]).unwrap().push((i, j));
                }
                else {
                    positions.insert(matrix[i][j], vec![(i, j)]);
                }
            }
        }
    }

    let mut total: u64 = 0;
    for i in positions {
        let vec = i.1;
        for j in 0..vec.len() {
            for e in j+1..vec.len() {
                let pos_j = (vec[j].0 as i32, vec[j].1 as i32);
                let pos_e = (vec[e].0 as i32, vec[e].1 as i32);
                println!("{:?} - {:?}", pos_e, pos_j);

                let diff_x = pos_j.0 - pos_e.0;
                let diff_y = pos_j.1 - pos_e.1;


                let pos1_x = pos_j.0 + diff_x;
                let pos1_y = pos_j.1 + diff_y;

                let pos2_x = pos_e.0 - diff_x;
                let pos2_y = pos_e.1 - diff_y;

                if pos1_x >= 0 && pos1_x < matrix[0].len() as i32 && pos1_y >= 0 && pos1_y < matrix.len() as i32 {
                    if matrix[pos1_x as usize][pos1_y as usize] != '#' {
                        matrix[pos1_x as usize][pos1_y as usize] = '#';
                        total += 1;
                    }
                }
                if pos2_x >= 0 && pos2_x < matrix[0].len() as i32 && pos2_y >= 0 && pos2_y < matrix.len() as i32 {
                    if matrix[pos2_x as usize][pos2_y as usize] != '#' {
                        matrix[pos2_x as usize][pos2_y as usize] = '#';
                        total += 1;
                    }
                }
            }
        }
    }
    for i in matrix {
        println!("{}", i.iter().collect::<String>());
    }

    println!("{}", total);
}
