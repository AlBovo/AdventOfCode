use std::fs::read_to_string;

// a memoization may be a lil overkill
fn dp(pos: usize, value: i64, nums: &Vec<i64>) -> bool {
    if pos == 0 {
        return value == nums[0];
    }

    let mut result = false;
    if value % nums[pos] == 0 {
        result |= dp(pos - 1, value / nums[pos], nums);
    }
    if !result {
        result |= dp(pos - 1, value - nums[pos], nums);
    }

    result
}

fn main() {
    let mut total = 0;
    for line in read_to_string("input.txt").unwrap().lines() {
        let number : i64;
        let mut num : i64 = 0;
        let mut nums : Vec<i64> = Vec::new();
        {
            let split = line.split(": ").collect::<Vec<&str>>();
            num = split[0].parse().unwrap();
            number = num;
            for i in split[1].split(" ") {
                nums.push(i.parse().unwrap());
            }
        }

        if dp(nums.len()-1, number, &nums) {
            total += number;
        }
    }
    println!("Total: {}", total);
}