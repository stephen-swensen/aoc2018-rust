#![allow(dead_code)]
//#![allow(unused_imports)]

use std::fs;

fn does_react(c1: char, c2: char) -> bool {
    c1 != c2 && c1.to_ascii_lowercase() == c2.to_ascii_lowercase()
}

fn get_at_i32(chars: &Vec<Option<char>>, i: i32) -> Option<&Option<char>> {
    if i >= 0 && i < chars.len() as i32 {
        chars.get(i as usize)
    } else {
        None
    }
}

fn react(chars: &mut Vec<Option<char>>) {
    let mut i: i32 = 0;
    let mut j: i32 = 1;
    loop {
        let c1 = get_at_i32(chars, i);
        let c2 = get_at_i32(chars, j);
        match (c1,c2) {
            (_,None) => return,
            (_,Some(None)) => return,
            (None,_) => {
                i = j;
                j = j+1;
            },
            (Some(None), _) => {
                i -= 1;
            },
            (Some(&Some(c1)), Some(&Some(c2))) => {
                if does_react(c1,c2) {
                    chars[i as usize] = None;
                    chars[j as usize] = None;
                    j += 1;
                    i -= 1;
                } else {
                    i = j;
                    j += 1;
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
    //println!("{:?}", chars);
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
