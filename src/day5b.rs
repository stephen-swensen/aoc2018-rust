#![allow(dead_code)]
//#![allow(unused_imports)]

use std::fs;

fn does_react(c1: char, c2: char) -> bool {
    c1 != c2 && c1.to_ascii_lowercase() == c2.to_ascii_lowercase()
}

fn chain_react(chars: &mut Vec<Option<char>>, i: usize, j:usize) -> usize {
    let mut i = i;
    let mut j = j;
    loop {
        if i == 0 {
            return j + 1;
        } 

        i = i - 1;
        j = j + 1;

        match (chars.get(i), chars.get(j)) {
            (Some(Some(c1)), Some(Some(c2))) => {
                if does_react(*c1,*c2) {
                    chars[i] = None;
                    chars[j] = None;
                    continue
                } else {
                    return j
                }
            },
            (Some(None), Some(Some(c2))) => {
                while i != 0 && chars.get(i-1).unwrap().is_none() {
                    i = i - 1;
                }
                if i == 0 {
                    return j;
                } else {
                    i = i - 1;
                    //println!("{}", i);
                    //println!("{:?}", &chars[0..6]);
                    let c1 = chars[i].unwrap();
                    if does_react(c1, *c2) {
                        chars[i] = None;
                        chars[j] = None;
                        continue
                    } else {
                        return j;
                    }
                }

            }
            _ => return j
        }
    }
}

fn react(chars: &mut Vec<Option<char>>) {
    let mut i = 0;
    while i < chars.len() - 1 {
        let c1 = chars[i].unwrap();
        let c2 = chars[i+1].unwrap();
        if does_react(c1,c2) {
            chars[i] = None;
            chars[i+1] = None;
            i = chain_react(chars, i, i+1);
        } else {
            i += 1;
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
