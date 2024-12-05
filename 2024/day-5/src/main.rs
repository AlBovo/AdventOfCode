use std::fs::read_to_string;

fn main() {
    let lines = read_to_string("input.txt").unwrap();
    let mut rules : Vec<Vec<i32>> = vec![vec![]; 100];
    let mut bool = false;
    let mut total = 0;

    for line in lines.lines() {
        if line == "" {
            bool = true;
        }
        else if !bool {
            let line_vec = line.split("|").collect::<Vec<&str>>();
            let a = line_vec[0].parse::<i32>().unwrap();
            let b = line_vec[1].parse::<i32>().unwrap();

            assert!(0 <= a && a < 100 && 0 <= b && b < 100);
            rules[b as usize].push(a);
        }
        else {
            let line_vec = line.split(",").collect::<Vec<&str>>();
            let mut flag = false;
            for i in 0..line_vec.len() {
                for e in i+1..line_vec.len() {
                    let a = line_vec[i].parse::<i32>().unwrap();
                    let b = line_vec[e].parse::<i32>().unwrap();

                    if rules[a as usize].contains(&b) {
                        flag = true;
                        break;
                    }
                }
                if flag { break; }
            }

            if !flag {
                let a = line_vec[line_vec.len()/2].parse::<i32>().unwrap();
                println!("{}", a);
                total += a;
            }
        }
    }
    println!("{}", total);
}