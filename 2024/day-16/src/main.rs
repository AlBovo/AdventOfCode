use std::fs::read_to_string;
use std::collections::VecDeque;

fn main() {
    let lines = read_to_string("input.txt").unwrap();
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in lines.lines() {
        grid.push(line.replace("E", ".").chars().collect());
    }
    assert_eq!(grid[grid.len()-2][1], 'S');
    assert_eq!(grid[1][grid[0].len()-2], '.');

    // 0 : up, 1 : right, 2 : down, 3 : left
    let pos: (usize, usize, usize) = (grid.len()-2, 1, 1);
    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
    queue.push_back(pos);
    let mut visited = vec![vec![(1e18 as i64, 0); grid[0].len()]; grid.len()];
    visited[pos.0][pos.1] = (0, 1);

    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();

        if visited[curr.0][curr.1].1 != curr.2 { continue }

        if grid[curr.0][curr.1+1] == '.' {
            let new_pes = visited[curr.0][curr.1].0 + 1 +
                1000 * if curr.2 == 0 || curr.2 == 2 { 1 } else if curr.2 == 1 { 0 } else { 2 };

            if new_pes < visited[curr.0][curr.1+1].0 {
                visited[curr.0][curr.1+1] = (new_pes, 1);
                queue.push_back((curr.0, curr.1+1, 1));
            }
        }
        if grid[curr.0][curr.1-1] == '.' {
            let new_pes = visited[curr.0][curr.1].0 + 1 +
                1000 * if curr.2 == 0 || curr.2 == 2 { 1 } else if curr.2 == 3 { 0 } else { 2 };

            if new_pes < visited[curr.0][curr.1-1].0 {
                visited[curr.0][curr.1-1] = (new_pes, 3);
                queue.push_back((curr.0, curr.1-1, 3));
            }
        }
        if grid[curr.0+1][curr.1] == '.' {
            let new_pes = visited[curr.0][curr.1].0 + 1 +
                1000 * if curr.2 == 1 || curr.2 == 3 { 1 } else if curr.2 == 2 { 0 } else { 2 };

            if new_pes < visited[curr.0+1][curr.1].0 {
                visited[curr.0+1][curr.1] = (new_pes, 2);
                queue.push_back((curr.0+1, curr.1, 2));
            }
        }
        if grid[curr.0-1][curr.1] == '.' {
            let new_pes = visited[curr.0][curr.1].0 + 1 +
                1000 * if curr.2 == 1 || curr.2 == 3 { 1 } else if curr.2 == 0 { 0 } else { 2 };

            if new_pes < visited[curr.0-1][curr.1].0 {
                visited[curr.0-1][curr.1] = (new_pes, 0);
                queue.push_back((curr.0-1, curr.1, 0));
            }
        }
    }
    // println!("{:?}", visited);
    println!("Result : {}", visited[1][grid[0].len()-2].0);
}
