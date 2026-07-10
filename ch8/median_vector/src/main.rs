use std::collections::HashMap;

fn main() {
    let nums = [0, 5, 6, 1, 6, 7, 1, 3, 2, 2, 8, 5, 0, 3, 4, 4, 1];

    let mut v1 = nums.to_vec();
    v1.sort();

    let v_len = v1.len();
    let median = v1[v_len / 2];
    println!("Median: {median}");


    let mut elementCounts = HashMap::new();
    for elm in &nums{
        let count = elementCounts.entry(*elm).or_insert(0);
        *count += 1;

    }

    let mut maxCount = 0;
    let mut maxCountvalue = 0;
    
    for (key, value) in &elementCounts{
        if *value > maxCount{
            maxCount = *value;
            maxCountvalue = *key;
        }
    }


    
    //Iterate through hashmap, save highest value, dont need to sort
    println!("Hashmap: {elementCounts:?}");
    println!("Mode: {}", maxCountvalue)
}