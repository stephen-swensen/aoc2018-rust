#![allow(dead_code)]
//#![allow(unused_imports)]

use std::fs;

fn does_react(c1: char, c2: char) -> bool {
    c1 != c2 && c1.to_ascii_lowercase() == c2.to_ascii_lowercase()
}

fn chain_react(mut chars: Vec<Option<char>>, i: usize, j:usize) -> usize {
    let i = i - 1;
    let j = j + 1;
    match (chars.get(i), chars.get(j)) {
        (Some(Some(c1)), Some(Some(c2))) => {
            chars[i] = None;
            chars[j] = None;
            return chain_react(chars, i,j);
        },
        _ => j
    }
}

fn react(mut chars: Vec<Option<char>>) -> Vec<Option<char>> {
    //we use offset to give us a major optimization: don't reiterate leading part of
    //vector that we know is already collapsed
    let mut offset = 0;
    'outer: loop {
        'inner: for (i, c1) in chars[offset..].iter().enumerate() {
            let i = i + offset;
            if i == chars.len() - 1 {
                break 'outer;
            } else {
                let c2 = chars.get(i+1).unwrap();
                if does_react(c1,c2) {
                    chars[i] = None;
                    chars[i+1] None; //we use i again because of shift left above
                    offset = if i == 0 { 0 } else { i-1 };
                    break 'inner;
                }
            }
        }
    }
    return chars;
}

pub fn part1() -> usize {
    let input = fs::read_to_string("inputs/day5.txt").unwrap();
    let chars: Vec<Option<char>> = input.chars().map(|x| Some(x)).collect();
    let chars = react(chars);
    chars.iter().filter(|x| x.is_some()).count()
}

pub fn part2() -> usize {
    let pairs: Vec<(char,char)> = (b'a'..=b'z')
        .map(|c| c as char)
        .map(|c| (c,c.to_ascii_uppercase()))
        .collect();

    let input = fs::read_to_string("inputs/day5.txt").unwrap();

    let r = pairs
        .iter()
        .map(|(p1,p2)| 
            react(
                input
                .chars()
                .filter(|c| c != p1 && c!= p2)
                .map(|x| Some(x))
                .collect::<Vec<Option<char>>>()
            ).iter().filter(|x| x.is_some()).count())
        .min()
        .unwrap();

    r
}
