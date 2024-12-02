use std::fs::read_to_string;

fn main() {
    let mut total = 0;
    for line in read_to_string("input.txt").unwrap().lines() {
        let vals = line.split(" ").collect::<Vec<&str>>();

        let mut prec = i32::MIN;
        let mut diff = i32::MIN;
        let mut done = false;
        for val in vals {
            let i = val.parse::<i32>().unwrap();

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
        if !done {
            total += 1;
        }
    }

    println!("{}", total);
}
