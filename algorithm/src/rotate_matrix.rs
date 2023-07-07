/*
Given an image represented by an NxN matrix, where each pixel in the image is 4
bytes, write a method to rotate the image by 90 degrees.
 */
pub fn run<Row: AsMut<[i8]>>(m: &mut [Row]) {
    let n = m.len();
    // println!("{n}");
    for i in 0..(n / 2) {
        for j in i..n - i - 1 {
            rotate_one(m, n, i, j);
        }
    }
    // print_matrix(m);
}

fn print_matrix<Row: AsMut<[i8]>>(m: &mut [Row]) {
    for i in 0..m.len() {
        let k = m[i].as_mut();
        for j in 00..k.len() {
            print!("{},", k[j]);
        }
        println!();
    }
}

fn rotate_one<Row: AsMut<[i8]>>(m: &mut [Row], n: usize, i: usize, j: usize) {
    let mut origin = m[i].as_mut()[j];
    let mut i = i;
    let mut j = j;
    for _ in 0..4 {
        let replace = m[j].as_mut()[n - 1 - i];
        // println!("origin:{origin}:{i}:{j}, target:{replace}:{j}:{}", n-1-i);
        m[j].as_mut()[n - 1 - i] = origin;
        origin = replace;
        let t = j;
        j = n - 1 - i;
        i = t;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_matrix() {
        let mut m = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        run(&mut m);
        assert_eq!([[7, 4, 1], [8, 5, 2], [9, 6, 3]], m);
        let mut m = [
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ];
        run(&mut m);
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
}
