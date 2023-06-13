fn main() {
    for i in 1..=20 {
        println!("{}",fizzbuzz(i));
    }
}

fn fizzbuzz(n: u32) -> String {
    let fizz = if n % 3 == 0 {"fizz"} else {""};
    let buzz = if n % 5 == 0 {"buzz"} else {""}; 

    if fizz.is_empty() && buzz.is_empty(){format!("{n}")} else {format!("{fizz}{buzz}")}
}
