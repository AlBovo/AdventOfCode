use std::fs::read_to_string;

fn main() {
    let input: String = read_to_string("input.txt").unwrap();
    let mut output: Vec<i64> = Vec::new();

    let mut full = true;
    let mut id: i64 = 0;
    for i in 0..input.len() {
        if full {
            let num = input.as_bytes()[i] - '0' as u8;
            for _ in 0..num{
                output.push(id);
            }
            id += 1;
        }
        else {
            let num = input.as_bytes()[i] - '0' as u8;
            for _ in 0..num{
                output.push(-1);
            }
        }
        full = !full;
    }

    let mut last = output.len() - 1;
    for i in 0..output.len() {
        while output[last] == -1 { last -= 1; }
        if i >= last { break; }
        if output[i] == -1 {
            output[i] = output[last];
            output[last] = -1;

            last -= 1;
        }
    }

    let mut total: i64 = 0;
    for i in 0..output.len() {
        if output[i] == -1 { break; }
        total += i as i64 * output[i]
    }

    println!("{:?}", total);
}
