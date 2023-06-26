/*
Given an image represented by an NxN matrix, where each pixel in the image is 4
bytes, write a method to rotate the image by 90 degrees.
 */
pub fn run<Row: AsMut<[i8]>>(m: &mut [Row]){
    let n = m.len();
    // println!("{n}");
    for i in 0..(n / 2) {
        for j in i..n-i-1 {
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
