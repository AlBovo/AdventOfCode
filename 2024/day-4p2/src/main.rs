use std::fs::read_to_string;

fn main() {
    let lines = read_to_string("input.txt").unwrap();
    let matrix : Vec<Vec<char>> = lines.lines().map(|l| l.chars().collect()).collect();

    let mut total = 0;

    for i in 1..matrix.len()-1 {
        for j in 1..matrix[0].len()-1 {
            if matrix[i][j] == 'A'{
                let sidel = format!("{}{}", matrix[i-1][j-1], matrix[i+1][j-1]);
                let sider = format!("{}{}", matrix[i-1][j+1], matrix[i+1][j+1]);

                if (sidel == "MM" && sider == "SS") ||
                    (sidel == "SM" && sider == "SM") ||
                    (sidel == "SS" && sider == "MM") ||
                    (sidel == "MS" && sider == "MS") {
                    total += 1;
                }
            }
        }
    }

    println!("{}", total);
}