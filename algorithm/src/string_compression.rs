/*
String Compression: Implement a method to perform basic string compression using the counts
of repeated characters. For example, the string aabcccccaaa would become a2blc5a3. If the
"compressed" string would not become smaller than the original string, your method should return
the original string. You can assume the string has only uppercase and lowercase letters (a - z).
 */
pub fn run(s: &str) -> String {
    if s.len() == 0 {
        return String::new();
    }
    let mut result = String::new();
    let mut c = s.chars().nth(0).unwrap();
    let mut count = 1;
    for i in s[1..].chars() {
        if i == c {
            count += 1;
        } else {
            result.push(c);
            result.push_str(&count.to_string());
            count = 1;
            c = i;
        }
    }
    result.push(c);
    result.push_str(&count.to_string());

    if result.len() >= s.len() {
        String::from(s)
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_comprssion() {
        assert_eq!("a2b1c5a3", run("aabcccccaaa"));
        assert_eq!("aa", run("aa"));
        assert_eq!("aab", run("aab"));
        assert_eq!("", run(""));
        assert_eq!("a", run("a"));
    }
}
