use std::collections::HashMap;

// a function that takes a vector of integers and returns the mean
fn mean(nums: Vec<u32>) -> u32 {
    let mut sum: u32 = 0;
    // add all of the numbers in the vector
    for num in nums.clone() {
        sum += num;
    }
    // divide the sum by the length
    sum / nums.len() as u32
}

// a function that takes a vector of integers and returns the median
fn median(nums: Vec<u32>) -> u32 {
    let mut numsmut = nums.clone();
    // sort the numbers
    numsmut.sort();
    // get the middle number
    numsmut[(numsmut.len() / 2)] as u32
}

// a function that takes a vector of integers and returns the mode
fn mode(nums: Vec<u32>) -> u32 {
    // create a key: value object to hold the amount of times each number appears
    let mut map: HashMap<u32, u32> = HashMap::new();
    // loop through the numbers
    for num in nums.clone() {
        // get the number already in the map or create one as 0
        let count = map.entry(num).or_insert(0);
        // add 1 to the value
        *count += 1;
    }
    // get the number with the highest value
    let max = *map.values().max().unwrap();
    // loop through the map and get the key with the highest value
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
