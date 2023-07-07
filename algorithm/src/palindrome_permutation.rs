use std::collections::HashMap;
/*
Palindrome Permutation: Given a string, write a function to check if it is a permutation of a palin­
drome. A palindrome is a word or phrase that is the same forwards and backwards. A permutation
is a rearrangement of letters. The palindrome does not need to be limited to just dictionary words.
 */
pub fn run(s: &str) -> bool {
    let mut char_count: HashMap<char, i8> = HashMap::new();
    for c in s.chars() {
        match char_count.get(&c) {
            Some(num) => char_count.insert(c, num + 1),
            None => char_count.insert(c, 1),
        };
    }
    let mut odd_count = 0;
    for (_, c) in &char_count {
        if c % 2 == 1 {
            odd_count += 1;
        }
    }

    odd_count <= 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome_permutation() {
        assert!(run("abc  ab"));
    }
}
