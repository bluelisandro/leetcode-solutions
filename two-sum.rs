use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut prev_map = HashMap::new();

    for (i, &n) in nums.iter().enumerate() {
        let diff = target - n;
        
        if prev_map.contains_key(&diff) {
            return vec![prev_map[&diff], i as i32];
        }
        
        prev_map.insert(n, i as i32);
    }    

    return vec![];
}
    
fn main() {
    let nums = vec![2,7,11,15];
    // let result = two_sum(&nums, 9);
    // println!("Result: {:?}",)
    println!("{:?}", two_sum(nums, 9));
}