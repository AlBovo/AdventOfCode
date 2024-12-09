use std::fs::read_to_string;

fn main() {
    let input: String = read_to_string("input.txt").unwrap();
    let mut sections: Vec<(i32, i32, i32)> = Vec::new(); // pos, id, size
    let mut empty : Vec<(i32, i32)> = Vec::new(); // pos, size

    let mut full = true;
    let mut id = 0;
    let mut pos = 0;
    for i in 0..input.len() {
        let num = input.bytes().nth(i).unwrap() as i32 - '0' as i32;
        if full {
            assert!(num > 0);
            sections.push((pos, id, num));
            pos += num;
            id += 1;
        }
        else {
            empty.push((pos, num));
            pos += num;
        }
        full = !full;
    }

    for i in sections.iter_mut().skip(1).rev() {
        for e in empty.iter_mut() {
            if e.0 > i.0 { break; }

            if e.1 >= i.2 {
                i.0 = e.0;

                e.0 += i.2;
                e.1 -= i.2;
                break;
            }
        }
    }

    let mut total: i64 = 0;
    for i in &sections { // pos, id, size
        total += (i.0 * i.2 + (i.2 * (i.2-1))/2) as i64 * i.1 as i64;
    }
    println!("{:?}", total);
}