use std::fs;

fn main() {
    let x = day1();
    println!("Result: {}", x);
}

fn day1() -> i32 {
    let txt = fs::read_to_string("day1.txt").unwrap();
    let lines = txt.split("\n").collect::<Vec<_>>();
    let sum = lines.iter().map(|x| x.parse::<i32>().unwrap_or(0)).sum();
    return sum;
}
