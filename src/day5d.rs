#![allow(dead_code)]
//#![allow(unused_imports)]

use std::fs;

fn does_react(c1: char, c2: char) -> bool {
    c1 != c2 && c1.to_ascii_lowercase() == c2.to_ascii_lowercase()
}

fn react(chars: &mut Vec<Option<char>>) {
    let mut i = 0;
    let mut j = 1;
    while j < chars.len() {
        let c1 = chars[i];
        match (c1, i) {
            (None, 0) => {
                //no chain reaction, continue advancing from last reaction
                i = j;
                j = i + 1;
            },
            (None, _) => {
                //find first non-None candidate for chain reactin (j is fixed)
                i -= 1;
            },
            (Some(c1), _) => {
                let c2 = chars[j].unwrap();
                if does_react(c1,c2) {
                    chars[i] = None;
                    chars[j] = None;
                    if i > 0 {
                        //chain react
                        i -= 1;
                        j += 1;
                    } else if i == 0 {
                        //can't chain react, advance to next candidate pairs
                        i = j + 1;
                        j = i + 1;
                    }
                } else {
                    //no reaction, advance to next candiate pairs from j
                    i = j;
                    j = i + 1;
                }
            }
        }
    }
}

pub fn part1() -> usize {
    let input = fs::read_to_string("inputs/day5.txt").unwrap();
    //let input = String::from("dabAcCaCBAcCcaDA");
    let mut chars: Vec<Option<char>> = input.chars().map(|x| Some(x)).collect();
    react(&mut chars);
    println!("{:?}", chars);
    chars.iter().filter(|x| x.is_some()).count()
}

pub fn part2() -> usize {
    let input = fs::read_to_string("inputs/day5.txt").unwrap();

    let pairs: Vec<(char,char)> = (b'a'..=b'z')
        .map(|c| c as char)
        .map(|c| (c,c.to_ascii_uppercase()))
        .collect();

    pairs
    .iter()
    .map(|(p1,p2)| {
        let mut chars =
            input
            .chars()
            .filter(|c| c != p1 && c!= p2)
            .map(|x| Some(x))
            .collect::<Vec<Option<char>>>();
        react(&mut chars);
        chars.iter().filter(|x| x.is_some()).count()
    })
    .min()
    .unwrap()
}
