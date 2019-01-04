#![allow(dead_code)]
#![allow(unused_imports)]

use maplit::*;
use std::fs;
use std::str::Lines;

pub fn part1() -> i32 {
    let txt = fs::read_to_string("inputs/day2.txt").unwrap();
    let lines = txt.lines();
    let mut twos = 0;
    let mut threes = 0;

    for line in lines {
        let mut has_two = false;
        let mut has_three = false;
        for c in line.chars() {
            let count = line.chars().filter(|c2| c2 == &c).count();
            match count {
                2 => has_two = true,
                3 => has_three = true,
                _ => (),
            }
        }
        if has_two {
            twos += 1;
        }
        if has_three {
            threes += 1;
        }
    }

    twos * threes
}

pub fn part2() -> Option<(String, String)> {
    let txt = fs::read_to_string("inputs/day2.txt").unwrap();
    let lines: Vec<&str> = txt.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        for line2 in &lines[i + 1..] {
            let count = line
                .chars()
                .zip(line2.chars())
                .filter(|(c1, c2)| c1 != c2)
                .count();

            if count == 1 {
                return Some((line.to_string(), line2.to_string()));
            }
        }
    }

    None
}
