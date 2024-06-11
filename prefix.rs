//Implement a function that finds the longest common prefix of a given set of strings.



fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new(); 
    }

    let first_str = &strings[0]; 

    for (i, c) in first_str.chars().enumerate() {
       
        for s in &strings[1..] {
           
            if i >= s.len() || s.chars().nth(i) != Some(c) {
              
                return first_str[..i].to_string();
            }
        }
    }

    first_str.to_string() 
}

fn main() {
    let strings = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];
    println!("Longest common prefix: {}", longest_common_prefix(&strings));

    let strings2 = vec![
        "dog".to_string(),
        "racecar".to_string(),
        "car".to_string(),
    ];
    println!("Longest common prefix: {}", longest_common_prefix(&strings2));
}
