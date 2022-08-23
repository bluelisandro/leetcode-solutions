use std::collections::HashMap;

fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut seen_map = HashMap::new();
    
    for (i, &val) in nums.iter().enumerate() {
        
        if seen_map.contains_key(&val) {
            return true;
        }

        seen_map.insert(val, i);
    }
    
    return false;
}

fn main() {
    // let nums = vec![1,2,3,1];
    let nums = vec![1,2,3,4];
    println!("{}", contains_duplicate(nums));
}