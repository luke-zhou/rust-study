/*
Is Unique: Implement an algorithm to determine if a string has all unique characters. What if you
cannot use additional data structures?
*/
pub fn run(s: &str) -> bool {
    if s.len() > 128 {
        true
    } else {
        let mut a = [false; 128];
        for c in s.chars() {
            let index = c as usize - 65;
            if a[index] {
                return true;
            } else {
                a[index] = true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duplicated_character() {
        assert!(run("abaa"));
    }
}
