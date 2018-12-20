#![allow(dead_code)]
#![allow(unused_imports)]

use std::fs;
use std::str::Lines;
use maplit::*;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug)]
struct Claim {
    id: String,
    left: i32,
    top: i32,
    width: i32,
    height: i32,
}

impl FromStr for Claim {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let delimiters = vec!['#','@',' ',':','x',','];
        let parts: Vec<&str> = s
            .split(|c| delimiters.contains(&c))
            .filter(|s| s != &"")
            .collect();
        println!("{:?}", parts);
        let parse_at = |i: usize| { 
            let s: &str = parts.get(i).unwrap();
            s.parse::<i32>()
        };
        let id = parts.get(0).unwrap().to_string();
        let left = parse_at(1)?;
        let top = parse_at(2)?;
        let width = parse_at(3)?;
        let height = parse_at(4)?;

        Ok(Claim { id,left,top,width,height })
    }
}

pub fn part1() -> Result<i32, ParseIntError> {
    let txt = fs::read_to_string("inputs/day3.txt").unwrap();
    let lines = txt.lines();
    for line in lines {
        let c: Claim = line.parse()?;
        println!("{:?}", c)
    }
    Ok(0)
}
