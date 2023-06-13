#![allow(dead_code)]

mod one_step_away;
mod duplicated_character;
mod permutation_check;
mod palindrome_permutation;

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
}