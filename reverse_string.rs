//Reverse a string in Rust


fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let s = "hello";
    let reversed = reverse_string(s);
    println!("Original: {}", s);
    println!("Reversed: {}", reversed);
}
