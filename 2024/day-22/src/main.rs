use std::fs::read_to_string;

fn mix(number: i64, secret: &mut i64) { *secret ^= number }

fn prune(secret: &mut i64) { *secret = *secret % 16777216 }

fn main() {
    let lines = read_to_string("input.txt").unwrap();
    let mut total: i64 = 0;

    for line in lines.split('\n') {
        let mut secret: i64 = line.parse().unwrap();
        for _i in 0..2000 {
            mix(secret * 64, &mut secret);
            prune(&mut secret);
            mix(secret / 32, &mut secret);
            prune(&mut secret);
            mix(secret * 2048, &mut secret);
            prune(&mut secret);
        }
        total += &secret;
    }
    println!("{}", total);
}
