mod utils;
mod day1;
mod day2;
mod day3;
mod day4;

use crate::utils::VecExt;

fn main() {
    //let x = day4::part1();
    let items = vec![(1,2),(2,1),(3,2),(4,3),(1,3)];
    let grouped = items.clone().group_by(|&(_,b)| b);
    println!("Result: {:#?}", grouped);
    for item in items {
        println!("{:?}", item)
    }
}