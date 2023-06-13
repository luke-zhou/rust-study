/*
Check Permutation: Given two strings, write a method to decide if one is a permutation of the
other.
*/

pub fn run(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut a = [0; 128];
    for c in s1.chars() {
        let index = c as usize - 65;
        a[index] += 1;
    }
    for c in s2.chars() {
        let index = c as usize - 65;
        a[index] -= 1;
        if a[index] < 0 {
            return false;
        }
    }
    true
}