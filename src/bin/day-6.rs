// Write a function that returns the reference to the longer string
// without any new allocations
pub fn longer_wish<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    // Your code here
    let s1_trimmed = s1.trim();
    let s2_trimmed = s2.trim();

    let s1_len = s1_trimmed.chars().count();
    let s2_len = s2_trimmed.chars().count();

    if s1_len > s2_len {
        Some(s1_trimmed)
    } else if s2_len > s1_len {
        Some(s2_trimmed)
    } else {
        None
    }
}

fn main() {
    let str1 = "  Hello, Mars!  ";
    let str2 = "Fellow Rustacean here!";

    match longer_wish(str1, str2) {
        Some(longer) => println!("The longer string is: '{}'", longer),
        None => println!("Both strings are of equal length."),
    }
}