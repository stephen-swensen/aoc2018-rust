
fn main() {
    let x = day1::part1();
    println!("Result: {}", x);
}

mod day1 {
    use std::fs;

    pub fn part1() -> i32 {
        let txt = fs::read_to_string("inputs/day1.txt").unwrap();
        let sum = txt
            .split("\n")
            .filter_map(|x| x.parse::<i32>().ok())
            .sum();
        sum
    }

    pub fn part2() -> i32 {
        let txt = fs::read_to_string("inputs/day1.txt").unwrap();
        let sum = txt
            .split("\n")
            .filter_map(|x| x.parse::<i32>().ok())
            .sum();
        sum
    }
}
