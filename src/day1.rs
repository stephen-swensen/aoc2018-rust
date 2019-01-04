#![allow(dead_code)]

use std::fs;
use maplit::*;

pub fn part1() -> i32 {
    let txt = fs::read_to_string("inputs/day1.txt").unwrap();
    let sum = txt
        .split("\n")
        .filter_map(|x| x.parse::<i32>().ok())
        .sum();
    sum
}

pub fn part2() -> Option<(usize,i32)> {
    let txt = fs::read_to_string("inputs/day1.txt").unwrap();

    let changes = txt
        .split("\n")
        .filter_map(|x| x.parse::<i32>().ok());

    let mut prev = 0;
    let mut freqs = hashset![0];

    for (i,x) in changes.cycle().enumerate() {
        let next = prev + x;
        if freqs.contains(&next) {
            return Some((i,next));
        }
        prev = next;
        freqs.insert(next);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 400);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), Some((143679, 232)));
    }
}
