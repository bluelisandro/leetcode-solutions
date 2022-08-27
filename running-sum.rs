fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut running_sum_vec = Vec::new();
    let mut sum = 0;
    
    for i in nums.iter() {
        sum += i;
        running_sum_vec.push(sum);
    }
    
    return running_sum_vec;
}

fn main() {
    let vec1 = vec![1,2,3,4];
    let vec2 = running_sum(vec1);
    println!("{:?}", vec2)
}