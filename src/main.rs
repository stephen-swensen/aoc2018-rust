mod utils;
mod day1;
mod day2;
mod day3;
mod day4;
mod day4b;
mod day5;
mod day5b;
mod day5c;

use time::PreciseTime;

fn main() {
    let start = PreciseTime::now();

    let x = day5::part2();
    //let x: Vec<i32> = vec![1,2,3,4];
    //let x = &x[1..];
    println!("Result: {:#?}", x);
    let end = PreciseTime::now();
    println!("Elapsed: {} seconds", start.to(end));
}