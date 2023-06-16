#![allow(dead_code)]

mod one_step_away;
mod duplicated_character;
mod permutation_check;
mod palindrome_permutation;
mod string_compression;

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duplicated_character() {
        assert!(duplicated_character::run("abaa"));
    }

    #[test]
    fn test_permutation_check() {
        assert!(permutation_check::run("abcd", "dbca"));
    }

    #[test]
    fn test_palindrome_permutation() {
        assert!(palindrome_permutation::run("abc  ab"));
    }

    #[test]
    fn test_one_step_away() {
        assert!(one_step_away::run("adda", "addab"));
        assert!(one_step_away::run("adda", "abda"));
        assert!(one_step_away::run("abdda", "adda"));
    }

    #[test]
    fn test_string_comprssion() {
        assert_eq!("a2b1c5a3", string_compression::run("aabcccccaaa"));
        assert_eq!("aa", string_compression::run("aa"));
        assert_eq!("aab", string_compression::run("aab"));
        assert_eq!("", string_compression::run(""));
        assert_eq!("a", string_compression::run("a"));
    }
}