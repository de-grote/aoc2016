use std::time;

mod day01;

fn main() {
    let before = time::SystemTime::now();
    day01::part2::solve().expect("invalid input");
    let after = time::SystemTime::now();
    println!("solved in: {:?}", after.duration_since(before).unwrap());
}