#![allow(dead_code)]
//#![allow(unused_imports)]

use std::fs;

fn react(mut chars: Vec<char>) -> Vec<char> {
    'outer: loop {
        'inner: for (i, c1) in chars.iter().enumerate() {
            if i == chars.len() - 1 {
                break 'outer;
            } else {
                let c2 = chars.get(i+1).unwrap();
                if c1 != c2 && c1.to_ascii_lowercase() == c2.to_ascii_lowercase() {
                    chars.remove(i);
                    chars.remove(i); //we use i again because of shift left above
                    break 'inner;
                }
            }
        }
    }
    return chars;
}

pub fn part1() -> usize {
    let input = fs::read_to_string("inputs/day5.txt").unwrap();
    let chars: Vec<char> = input.chars().collect();
    let chars = react(chars);
    chars.len()
}

pub fn part2() -> usize {
    let pairs: Vec<(char,char)> = (b'a'..=b'z')
        .map(|c| c as char)
        .map(|c| (c,c.to_ascii_uppercase()))
        .collect();

    let input = fs::read_to_string("inputs/day5.txt").unwrap();

    let r = pairs
        .iter()
        .map(|(p1,p2)| react(input.chars().filter(|c| c != p1 && c!= p2).collect::<Vec<char>>()).len())
        .min()
        .unwrap();

    r
}
