use std::env;

fn palindrome(word: &str) -> bool {
    let chars: Vec<char> = word.chars().collect();
    let len = chars.len();

    for i in 0..len / 2 {
        if chars[i] != chars[len - 1 - i] {
            return false;
        }
    }
    true
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: ./palindrome <word>");
        return;
    }

    let word = &args[1];
    let result = palindrome(word);
    println!("palindrome ? {}", result);
}
