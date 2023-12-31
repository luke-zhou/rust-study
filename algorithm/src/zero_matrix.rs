/*
Zero Matrix: Write an algorithm such that if an element in an MxN matrix is 0, its entire row and
column are set to 0.
 */

pub fn run<Row: AsMut<[i8]>>(m: &mut [Row]) {
    let n = m.len();
    let first_row_zero = m[0].as_mut().iter().any(|&x| x == 0);
    let mut first_column_zero = false;
    for i in 0..n {
        if m[i].as_mut()[0] == 0 {
            first_column_zero = true;
            break;
        }
    }

    for i in 1..n {
        for j in 1..n {
            if m[0].as_mut()[j] == 0 && m[i].as_mut()[0] == 0 {
                continue;
            }
            let t = m[i].as_mut()[j];
            if t == 0 {
                m[0].as_mut()[j] = 0;
                m[i].as_mut()[0] = 0;
            }
        }
    }

    for i in 1..n {
        if m[0].as_mut()[i] == 0 {
            update_column(m, i, n);
        }
    }
    for i in 1..n {
        if m[i].as_mut()[0] == 0 {
            update_row(m, i, n);
        }
    }
    if first_row_zero {
        update_row(m, 0, n);
    }
    if first_column_zero {
        update_column(m, 0, n);
    }
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

fn update_row<Row: AsMut<[i8]>>(m: &mut [Row], index: usize, size: usize) {
    for i in 0..size {
        m[index].as_mut()[i] = 0;
    }
}

fn update_column<Row: AsMut<[i8]>>(m: &mut [Row], index: usize, size: usize) {
    for i in 0..size {
        m[i].as_mut()[index] = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_matrix() {
        let mut m = [[1, 0, 3], [4, 5, 6], [7, 8, 9]];
        run(&mut m);
        assert_eq!([[0, 0, 0], [4, 0, 6], [7, 0, 9]], m);
        let mut m = [[1, 2, 3], [4, 0, 6], [7, 8, 9]];
        run(&mut m);
        assert_eq!([[1, 0, 3], [0, 0, 0], [7, 0, 9]], m);
        let mut m = [[1, 2, 3, 4], [5, 0, 7, 8], [9, 10, 11, 12], [13, 14, 15, 0]];
        run(&mut m);
        assert_eq!(
            [[1, 0, 3, 0], [0, 0, 0, 0], [9, 0, 11, 0], [0, 0, 0, 0],],
            m
        );
        let mut m = [[1, 2, 3, 4], [5, 0, 7, 8], [9, 10, 11, 12], [13, 0, 15, 16]];
        run(&mut m);
        assert_eq!(
            [[1, 0, 3, 4], [0, 0, 0, 0], [9, 0, 11, 12], [0, 0, 0, 0],],
            m
        );
    }
}
