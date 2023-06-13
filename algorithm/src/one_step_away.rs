/*
One Away: There are three types of edits that can be performed on strings: insert a character,
remove a character, or replace a character. Given two strings, write a function to check if they are
one edit (or zero edits) away.
EXAMPLE
pale, ple -> true
pales, pale -> true
pale,bale -> true
pale,bae -> false
 */
pub fn run(s1: &str, s2: &str) -> bool {
    let s1_len = s1.len() as isize;
    let s2_len = s2.len() as isize;
    if s1_len == s2_len {
        replace_check(s1, s2)
    } else if s1_len - s2_len == 1 {
        insert_check(s2, s1)
    } else if s2_len - s1_len == 1 {
        insert_check(s1, s2)
    } else {
        false
    }
}

fn replace_check(s1: &str, s2: &str) -> bool {
    let diff_chars_count = s1.chars().zip(s2.chars()).filter(|(a,b)| a != b).count();
    diff_chars_count == 1
}

fn insert_check(short: &str, long: &str) -> bool {
    let len = short.len();
    let mut offset = 0;
    for i in 0..len {
        if short.chars().nth(i) != long.chars().nth(i+offset){
            offset +=1;
        }
    };
    // println!("{offset}");
    offset <=1
}
