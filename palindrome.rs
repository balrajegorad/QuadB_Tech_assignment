//Implement a function that checks whether a given string is a palindrome or not.



fn is_palindrome(s: &str) -> bool {
    let s = s.to_lowercase(); 
    let s = s.chars().filter(|c| c.is_alphanumeric()).collect::<String>(); 
    let reversed = s.chars().rev().collect::<String>(); g

    s == reversed 
}

fn main() {
    let input = "A man, a plan, a canal, Panama!";
    println!("Is '{}' a palindrome? {}", input, is_palindrome(input));
}
