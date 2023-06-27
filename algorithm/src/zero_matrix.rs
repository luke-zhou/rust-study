/*
Zero Matrix: Write an algorithm such that if an element in an MxN matrix is 0, its entire row and
column are set to 0.
 */

pub fn run<Row: AsMut<[i8]>>(m: &mut [Row]){
    let n = m.len();
    for i in 0..n {
        for j in 0..n {
            if m[0].as_mut()[j] == 0{
                continue;
            }
            let t = m[i].as_mut()[j];
            if t == 0 {
                update_row(m, i, n);
                update_column(m, i, n);
                break;
            }
        }
    }
}

fn update_row<Row: AsMut<[i8]>>(m: &mut [Row], index: usize, size: usize){
    for i in 0..size {
        m[index].as_mut()[i] = 0;
    }
}

fn update_column<Row: AsMut<[i8]>>(m: &mut [Row], index: usize, size: usize){
    for i in 0..size {
        m[i].as_mut()[index] = 0;
    }
}