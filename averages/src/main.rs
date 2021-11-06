use std::collections::HashMap;

fn mean(nums: Vec<u32>) -> u32 {
    let mut sum: u32 = 0;
    for num in nums.clone() {
        sum += num;
    }
    sum / nums.len() as u32
}

fn median(nums: Vec<u32>) -> u32 {
    let mut numsmut = nums.clone();
    numsmut.sort();
    numsmut[(numsmut.len() / 2)] as u32
}

fn mode(nums: Vec<u32>) -> u32 {
    let mut map: HashMap<u32, u32> = HashMap::new();
    for num in nums.clone() {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }
    let max = *map.values().max().unwrap();
    for (key, value) in map {
        if value == max {
            return key;
        }
    }
    0
}

fn main() {
    let nums = vec![10, 12, 12, 12, 30, 30, 14, 50, 50];
    println!("mean: {}", mean(nums.clone()));
    println!("median: {}", median(nums.clone()));
    println!("mode: {}", mode(nums));
}
