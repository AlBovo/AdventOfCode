use std::fs::read_to_string;
use std::collections::VecDeque;

type Pair = (usize, usize);

fn explore(start: Pair, end: Pair, grid: &Vec<Vec<char>>) -> i64 {
    let mut counts = vec![vec![(1e8 as i64, false); grid[0].len()]; grid.len()];
    let mut queue = VecDeque::new();
    queue.push_back(start);
    counts[start.0][start.1].0 = 0;

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();

        if counts[x][y].1 { continue; }
        counts[x][y].1 = true;

        if grid[x+1][y] == '.' && counts[x][y].0 + 1 < counts[x+1][y].0 {
            queue.push_back((x+1, y));
            counts[x+1][y] = (counts[x][y].0 + 1, false);
        }
        if grid[x-1][y] == '.' && counts[x][y].0 + 1 < counts[x-1][y].0 {
            queue.push_back((x - 1, y));
            counts[x - 1][y] = (counts[x][y].0 + 1, false);
        }
        if grid[x][y+1] == '.' && counts[x][y].0 + 1 < counts[x][y+1].0 {
            queue.push_back((x, y + 1));
            counts[x][y + 1] = (counts[x][y].0 + 1, false);
        }
        if grid[x][y-1] == '.' && counts[x][y].0 + 1 < counts[x][y-1].0 {
            queue.push_back((x, y - 1));
            counts[x][y - 1] = (counts[x][y].0 + 1, false);
        }
    }

    counts[end.0][end.1].0
}

fn main() {
    let lines = read_to_string("input.txt").unwrap();
    let mut grid: Vec<Vec<char>> = Vec::new();

    let mut start: Pair = (0, 0);
    let mut end: Pair = (0, 0);
    for line in lines.split('\n').enumerate() {
        grid.push(Vec::new());
        for c in line.1.chars().enumerate() {
            if c.1 == 'S' { start = (line.0, c.0) }
            if c.1 == 'E' { end = (line.0, c.0) }

            if c.1 == '#' { grid[line.0].push('#') }
            else { grid[line.0].push('.') }
        }
    }
    assert_ne!(start, (0, 0));
    assert_ne!(end, (0, 0));

    let total = explore(start, end, &grid);
    let mut result = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '#' {
                // no borders
                if i == 0 || j == 0 || i == grid.len() - 1 || j == grid[0].len() - 1 { continue }
                if !((grid[i+1][j] == '.' && grid[i-1][j] == '.') ||
                    (grid[i][j+1] == '.' && grid[i][j+1] == '.')) { continue }

                grid[i][j] = '.';
                let r = explore(start, end, &grid);
                if total - r >= 100 { result += 1 }
                grid[i][j] = '#';
            }
        }
    }

    println!("{}", result);
}