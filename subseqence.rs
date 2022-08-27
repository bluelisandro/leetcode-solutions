fn is_subsequence(s: String, t: String) -> bool {
    let mut z = String::new();

    // do something with index 'i' and char 'c'
    for (_i1, c1) in t.chars().enumerate() {
        for (_i2, c2) in s.chars().enumerate() {
            println!("c1: {}, c2: {}", c1, c2);
            if c1 == c2 {
                z.push(c1);
            }
        }
        // println!("{:?}", i);
    }

    if s == z {
        return true;
    }
    
    return false;
}

fn main() {
    let s = "bb".to_string();
    let t = "ahbgdc".to_string();
    // is_subsequence(s, t);
    println!("{}", is_subsequence(s, t));
}