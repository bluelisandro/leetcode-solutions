use std::collections::HashMap;

fn valid_anagram(s:String, t:String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut s_map = HashMap::new();
    let mut t_map = HashMap::new();
    
    for i in s.chars() {
        // s_map.insert(s[i], 1 + s_map.get(s[i]));
        // t_map.insert(t[i], 1 + s_map.get(t[i]));
    }
    
    return s_map == t_map;
}

fn main() {
    let s = "aacc".to_string();
    let t = "ccac".to_string();
    println!("{}", valid_anagram(s, t));
}