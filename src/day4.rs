#![allow(dead_code)]
#![allow(unused_imports)]

use std::fs;
use std::str::Lines;
use maplit::*;
use std::str::FromStr;
use std::collections::HashSet;
use regex::Regex;

#[derive(Debug)]
pub enum LogEvent {
    BeginsShift(String),
    WakesUp,
    FallsAsleep,
}

#[derive(Debug)]
pub struct LogEntry {
    year: i32,
    month: i32,
    day: i32,
    hrs: i32,
    mins: i32,
    log_event: LogEvent
}

fn parse_log() -> Vec<LogEntry> {
    let txt = fs::read_to_string("inputs/day4.txt").unwrap();
    let lines = txt.lines();
    let date_regx = Regex::new(r"(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})").unwrap();
    let mut log_entries = vec![];
    for line in lines {
        let parts: Vec<&str> = line.split("] ").collect();
        let date_part = parts.get(0).unwrap();
        let event_part = parts.get(1).unwrap();
        let log_event = 
            match *event_part {
                "wakes up" => LogEvent::WakesUp,
                "falls asleep" => LogEvent::FallsAsleep,
                _ => LogEvent::BeginsShift(event_part.to_string())
            };
        
        let year = 0;
        let month = 0;
        let day = 0;
        let hrs = 0;
        let mins = 0;
        log_entries.push(LogEntry { year, month, day, hrs, mins, log_event });
    }
    log_entries
}

pub fn part1() -> Vec<LogEntry> {
    parse_log()
}

pub fn part2() -> i32 {
    0
}
