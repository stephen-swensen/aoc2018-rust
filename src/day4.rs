#![allow(dead_code)]
#![allow(unused_imports)]

use std::fs;
use std::str::Lines;
use maplit::*;
use std::str::FromStr;
use std::collections::HashMap;
use regex::Regex;
use crate::utils::IteratorExt;

#[derive(Debug)]
enum LogEvent {
    BeginsShift(i32),
    WakesUp,
    FallsAsleep,
}

#[derive(Debug)]
struct LogEntry {
    raw: String,
    year: i32,
    month: i32,
    day: i32,
    hrs: i32,
    mins: i32,
    log_event: LogEvent
}

#[derive(Debug)]
pub struct SleepDuration {
    guard_id: i32,
    start: i32,
    duration: i32,
}

fn parse_log() -> Vec<LogEntry> {
    let txt = fs::read_to_string("inputs/day4.txt").unwrap();
    let lines = txt.lines();
    let date_regx = Regex::new(r"(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})").unwrap();
    let id_regx = Regex::new(r"#(\d*) ").unwrap();
    let mut log_entries = vec![];
    for line in lines {
        let parts: Vec<&str> = line.split("] ").collect();
        let date_part = parts.get(0).unwrap();
        let event_part = parts.get(1).unwrap();
        let log_event = 
            match *event_part {
                "wakes up" => LogEvent::WakesUp,
                "falls asleep" => LogEvent::FallsAsleep,
                _ => {
                    let caps = id_regx.captures(event_part).unwrap();
                    let id = caps[1].parse().unwrap();
                    LogEvent::BeginsShift(id)
                }
            };
        
        let caps = date_regx.captures(date_part).unwrap();

        let year = caps[1].parse().unwrap();
        let month = caps[2].parse().unwrap();
        let day = caps[3].parse().unwrap();
        let hrs = caps[4].parse().unwrap();
        let mins = caps[5].parse().unwrap();
        log_entries.push(LogEntry { raw:line.to_string(), year, month, day, hrs, mins, log_event });
    }
    log_entries.sort_by(|a,b| a.raw.cmp(&b.raw));
    log_entries
}

fn collect_sleep_durations(log_entries: &Vec<LogEntry>) -> Vec<SleepDuration> {
    let mut sleep_durations: Vec<SleepDuration> = vec![];
    let mut guard_id = 0;
    let mut sleep_start = 0;
    for le in log_entries {
        match le.log_event {
            LogEvent::BeginsShift(id) => guard_id = id,
            LogEvent::FallsAsleep => sleep_start = le.mins,
            LogEvent::WakesUp => {
                let duration = le.mins - sleep_start;
                sleep_durations.push(SleepDuration { guard_id, start: sleep_start, duration})
            }
        }
    }
    sleep_durations
}

pub fn part1() -> i32 {
    let log_entries = parse_log();
    let sleep_durations = collect_sleep_durations(&log_entries);
    let guard_sleep_durations = sleep_durations.iter().group_by_key(|sd| sd.guard_id);
    let best_guard = guard_sleep_durations
        .iter()
        .max_by_key(|(_,value)| value.iter().sum_by_key(|sd| sd.duration))
        .unwrap();
    let best_min = best_guard.1
        .iter()
        .map(|sd| (sd.start..sd.start + sd.duration))
        .flatten()
        .group_by_key(|m| *m)
        .into_iter()
        .max_by_key(|(_,v)| v.len())
        .unwrap().0;

    println!("guard_id = {}, best_min = {}", best_guard.0, best_min);
    best_guard.0 * best_min
}

pub fn part2() -> i32 {
    0
}
