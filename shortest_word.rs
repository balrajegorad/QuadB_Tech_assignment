//Given a string of words, implement a function that returns the shortest word in the string.




fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|word| word.len())
}

fn main() {
    let input = "This is a string of words";
    if let Some(shortest) = shortest_word(input) {
        println!("The shortest word is: {}", shortest);
    } else {
        println!("No words found in the input string");
    }
}
