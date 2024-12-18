use std::fs::read_to_string;
use std::collections::VecDeque;

fn main() {
    let lines: String = read_to_string("input.txt").unwrap();
    let mut matrix = vec![vec![false; 71]; 71];

    let mut i = 0;
    for line in lines.lines() {
        if i >= 1024 { break }
        let x = line.split(',').collect::<Vec<&str>>();
        let a = x[0].parse::<usize>().unwrap();
        let b = x[1].parse::<usize>().unwrap();
        matrix[a][b] = true;
        i += 1;
    }

    let mut visited = vec![vec![(1e8 as i64, false); 71]; 71];
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back((0, 0)); visited[0][0] = (0, false);

    while !q.is_empty() {
        let (x, y) = q.pop_front().unwrap();
        if visited[x][y].1 { continue }
        visited[x][y].1 = true;

        if x > 0 && !matrix[x-1][y] {
            let cost = visited[x][y].0 + 1;
            if cost < visited[x-1][y].0 {
                visited[x-1][y] = (cost, false);
                q.push_back((x-1, y));
            }
        }
        if x < 70 && !matrix[x+1][y] {
            let cost = visited[x][y].0 + 1;
            if cost < visited[x+1][y].0 {
                visited[x+1][y] = (cost, false);
                q.push_back((x+1, y));
            }
        }
        if y > 0 && !matrix[x][y-1] {
            let cost = visited[x][y].0 + 1;
            if cost < visited[x][y-1].0 {
                visited[x][y-1] = (cost, false);
                q.push_back((x, y-1));
            }
        }
        if y < 70 && !matrix[x][y+1] {
            let cost = visited[x][y].0 + 1;
            if cost < visited[x][y+1].0 {
                visited[x][y+1] = (cost, false);
                q.push_back((x, y+1));
            }
        }
    }
    println!("{}", visited[70][70].0);
}