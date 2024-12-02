use std::fs::read_to_string;

fn check(vals: Vec<&str>, idx: i32) -> bool {
    let mut prec = i32::MIN;
    let mut diff = i32::MIN;
    let mut done = false;

    for val in 0..vals.len() as i32 {
        if val == idx { continue; }
        let i = vals[val as usize].parse::<i32>().unwrap();

        if prec == i32::MIN {
            prec = i;
        }
        else {
            if (prec - i).abs() > 3 || prec == i {
                done = true;
                break;
            }

            if diff == i32::MIN {
                diff = prec - i;
            }
            else {
                if diff * (prec - i) < 0 {
                    done = true;
                    break;
                }
                diff = prec - i;
            }
        }
        prec = i;
    }
    return !done;
}

fn main() {
    let mut total = 0;
    for line in read_to_string("input.txt").unwrap().lines() {
        let vals = line.split(" ").collect::<Vec<&str>>();

        for i in -1..vals.len() as i32 {
            if check(vals.clone(), i) {
                total += 1;
                break;
            }
        }
    }

    println!("{}", total);
}
