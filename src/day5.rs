#![allow(dead_code)]
#![allow(unused_imports)]

use std::fs;
use std::str::Lines;
use maplit::*;
use std::str::FromStr;
use std::collections::HashMap;
use regex::Regex;
use crate::utils::IteratorExt;

pub fn part1() -> String {
    let input = String::from("dabAcCaCBAcCcaDA");
    let chars: Vec<char> = input.chars().collect();
    let windowed = chars.windows(2);
    println!("{:?}", windowed);
    input
}

pub fn part2() {

}
