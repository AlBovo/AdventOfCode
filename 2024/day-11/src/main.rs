use std::fs::read_to_string;

fn merge(vec1: &mut Vec<(i64, i64)>, vec2: &Vec<(i64, i64)>){
    for i in 0..vec2.len() {
        for j in 0..vec2.len()+1 {
            if j == vec1.len() {
                vec1.push(vec2[i]);
                break;
            }
            else if vec1[j].0 == vec2[i].0 {
                vec1[j].1 += vec2[i].1;
                break;
            }
        }
    }
}

fn main() {
    let lines: String = read_to_string("input.txt").unwrap();
    let nums: Vec<i64> = lines.split(' ').map(|x| x.parse::<i64>().unwrap()).collect();

    let mut vec: Vec<(i64, i64)> = Vec::new(); // (value, count)
    for i in nums {
        for e in 0..vec.len()+1 {
            if e == vec.len() {
                vec.push((i, 1));
                break;
            }
            else if vec[e].0 == i {
                vec[e].1 += 1;
                break;
            }
        }
    }

    for _ in 0..25 { // set here the size
        let mut new: Vec<(i64, i64)> = Vec::new();
        for i in vec {
            if i.0 == 0 {
                new.push((1, i.1));
            }
            else if i.0.to_string().len() % 2 == 0 {
                let r = i.0.to_string();
                new.push((r[..r.len()/2].parse::<i64>().unwrap(), i.1));
                new.push((r[r.len()/2..].parse::<i64>().unwrap(), i.1));
            }
            else { new.push((i.0 * 2024, i.1)); }
        }
        vec = Vec::new();
        merge(&mut vec, &new);
        // println!("{:?} {:?}", new, vec);
    }

    let mut total = 0;
    for i in vec {
        total += i.1;
    }
    println!("{}", total);
}