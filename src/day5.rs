#![allow(dead_code)]
#![allow(unused_imports)]

use std::fs;
use std::str::Lines;
use maplit::*;
use std::str::FromStr;
use std::collections::HashMap;
use regex::Regex;
use crate::utils::IteratorExt;

fn react(mut chars: Vec<char>) -> Vec<char> {
    for (i, c1) in chars.iter().enumerate() {
        if i == chars.len() - 1 {
            //println!("Reached end of loop");
            return chars;
        } else {
            let c2 = chars.get(i+1).unwrap();
            if c1 != c2 && c1.to_ascii_lowercase() == c2.to_ascii_lowercase() {
                //println!("Removed {} at {} and {} at {}", c1, i, c2, i+1);
                chars.remove(i);
                chars.remove(i); //we use i again because of shift left above
                return react(chars);
            }
        }
    }
    //println!("Exit function");
    return chars; //this shouldn't be possible
}

pub fn part1() -> usize {
    let input = fs::read_to_string("inputs/day5.txt").unwrap();
    let chars: Vec<char> = input.chars().collect();
    let chars = react(chars);
    //println!("{:?}", chars);
    chars.len()
}

pub fn part2() {

}
