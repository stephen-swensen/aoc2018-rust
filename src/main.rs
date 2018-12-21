mod utils;
mod day1;
mod day2;
mod day3;
mod day4;

use crate::utils::group_by;

fn main() {
    //let x = day4::part1();
    let items = vec![(1,2),(2,1),(3,2),(4,3),(1,3)];
    let grouped = group_by(items, |&(_,b)| b);
    println!("Result: {:#?}", grouped);
}