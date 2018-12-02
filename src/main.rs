use std::fs;

fn main() {
    let x = day1();
    println!("Result: {}", x);
}

fn day1() -> i32 {
    let txt = fs::read_to_string("inputs/day1.txt").unwrap();
    let sum = txt
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap_or(0))
        .sum();
    sum
}
