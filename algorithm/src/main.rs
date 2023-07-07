#![allow(dead_code)]

mod duplicated_character;
mod one_step_away;
mod palindrome_permutation;
mod permutation_check;
mod rotate_matrix;
mod string_compression;
mod zero_matrix;
mod linked_list;
mod remove_dups;

fn main() {}

#[cfg(test)]
mod tests {
    use crate::linked_list::LinkedList;

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

    #[test]
    fn test_rotate_matrix() {
        let mut m = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        rotate_matrix::run(&mut m);
        assert_eq!([[7, 4, 1], [8, 5, 2], [9, 6, 3]], m);
        let mut m = [
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ];
        rotate_matrix::run(&mut m);
        assert_eq!(
            [
                [13, 9, 5, 1],
                [14, 10, 6, 2],
                [15, 11, 7, 3],
                [16, 12, 8, 4]
            ],
            m
        );
    }

    #[test]
    fn test_zero_matrix() {
        let mut m = [[1, 0, 3], [4, 5, 6], [7, 8, 9]];
        zero_matrix::run(&mut m);
        assert_eq!([[0, 0, 0], [4, 0, 6], [7, 0, 9]], m);
        let mut m = [[1, 2, 3], [4, 0, 6], [7, 8, 9]];
        zero_matrix::run(&mut m);
        assert_eq!([[1, 0, 3], [0, 0, 0], [7, 0, 9]], m);
        let mut m = [
            [1, 2, 3, 4],
            [5, 0, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 0],
        ];
        zero_matrix::run(&mut m);
        assert_eq!(
            [
                [1, 0, 3, 0],
                [0, 0, 0, 0],
                [9, 0, 11, 0],
                [0, 0, 0, 0],
            ],
            m
        );
        let mut m = [
            [1, 2, 3, 4],
            [5, 0, 7, 8],
            [9, 10, 11, 12],
            [13, 0, 15, 16],
        ];
        zero_matrix::run(&mut m);
        assert_eq!(
            [
                [1, 0, 3, 4],
                [0, 0, 0, 0],
                [9, 0, 11, 12],
                [0, 0, 0, 0],
            ],
            m
        );
    }

    #[test]
    fn test_linked_list() {
        let l: LinkedList = LinkedList::new(1);
        println!("{l:?}");

        let mut l2 = l.prepend(1).prepend(2).prepend(3).prepend(2);
        println!("{l2}");
        println!("{l2}");

        l2.append(5);
        l2.append(6);
        l2.append(7);

        println!("{l2}");
        println!("{l2}");
        // remove_dups::run(&mut l2)
    }
}
