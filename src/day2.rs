#![allow(dead_code)]
#![allow(unused_imports)]

use std::fs;
use std::str::Lines;
use maplit::*;

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
                _ => ()
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
