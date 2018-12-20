#![allow(dead_code)]
#![allow(unused_imports)]

use std::fs;
use std::str::Lines;
use maplit::*;
use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::HashSet;

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

impl Claim {
    fn coords(&self) -> Vec<(i32,i32)> {
        let mut v: Vec<(i32,i32)> = vec![];
        for x in self.left..(self.left+self.width) {
            for y in self.top..(self.top+self.height) {
                let c = (x,y);
                v.push(c);
            }
        }
        v
    }
}

fn parse_claims() -> Vec<Claim> {
    let txt = fs::read_to_string("inputs/day3.txt").unwrap();
    let lines = txt.lines();
    let claims: Vec<Claim> = lines.map(|line| line.parse().unwrap()).collect();
    claims
}

fn get_overlap(claims: &Vec<Claim>) -> HashSet<(i32,i32)> {
    let mut all = HashSet::<(i32,i32)>::new();
    let mut overlap = HashSet::<(i32,i32)>::new();
    for c in claims {
        for coord in c.coords() {
            if all.contains(&coord) {
                overlap.insert(coord);
            } else {
                all.insert(coord);
            }
        }
    }
    overlap
}

pub fn part1() -> usize {
    let claims = parse_claims();
    let overlap = get_overlap(&claims);
    overlap.len()
}

pub fn part2() -> Option<String> {
    let claims = parse_claims();
    let overlap = get_overlap(&claims);
    for c in &claims {
        let mut has_overlap = false;
        for coord in c.coords() {
            if overlap.contains(&coord) {
                has_overlap = true;
                break;
            } 
        }
        if !has_overlap {
            return Some(c.id.to_string());
        }
    }
    None
}
