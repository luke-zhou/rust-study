// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    let cc_number = remove_whitespace(cc_number);
    // println!("{}", cc_number.len());
    if cc_number.len() >1 && cc_number.chars().all(char::is_numeric){
        return luhn_check(&cc_number);
    }
    false
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c: &char| !c.is_whitespace()).collect()
}

fn luhn_check(cc: &str) -> bool {
    let mut sum = 0;
    for (i,c) in cc.chars().rev().enumerate(){
        let mut r = c.to_digit(10).unwrap();
        if i % 2 == 1{
            r = r*2;
            r = r%10+r/10;
        }
        sum +=r;
    }
    sum % 10 == 0
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {}