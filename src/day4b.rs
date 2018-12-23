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
enum LogEntry {
    BeginsShift { guard_id: i32, min: i32 },
    WakesUp { min: i32 },
    FallsAsleep { min: i32 },
}

#[derive(Debug)]
struct SleepDuration {
    guard_id: i32,
    start: i32,
    duration: i32,
}

fn parse_log(filename: &str) -> Vec<LogEntry> {
    let txt = fs::read_to_string(filename).unwrap();
    let mut lines: Vec<&str> = txt.lines().collect();
    lines.sort_unstable();

    let date_regx = Regex::new(r"(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})").unwrap();
    let id_regx = Regex::new(r"#(\d*) ").unwrap();

    let log_entries: Vec<LogEntry> = lines
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split("] ").collect();

            let date_part = parts.get(0).unwrap();
            let date_caps = date_regx.captures(date_part).unwrap();
            let min = date_caps[5].parse().unwrap();

            let &event_part = parts.get(1).unwrap();
            match event_part {
                "wakes up" => LogEntry::WakesUp { min },
                "falls asleep" => LogEntry::FallsAsleep { min },
                _ => {
                    let id_caps = id_regx.captures(event_part).unwrap();
                    let guard_id = id_caps[1].parse().unwrap();
                    LogEntry::BeginsShift { guard_id, min }
                }
            }
        })
        .collect();
    log_entries
}

fn collect_sleep_durations(log_entries: &Vec<LogEntry>) -> Vec<SleepDuration> {
    let mut sleep_durations: Vec<SleepDuration> = vec![];
    let mut id = 0;
    let mut sleep_start = 0;
    for le in log_entries {
        match le {
            LogEntry::BeginsShift { guard_id, min } => id = *guard_id,
            LogEntry::FallsAsleep { min } => sleep_start = *min,
            LogEntry::WakesUp { min } => {
                let duration = min - sleep_start;
                sleep_durations.push(SleepDuration { guard_id: id, start: sleep_start, duration})
            }
        }
    }
    sleep_durations
}

pub fn part1() -> i32 {
    let log_entries = parse_log("inputs/day4.txt");
    let sleep_durations = collect_sleep_durations(&log_entries);
    let guard_sleep_durations = sleep_durations
        .iter().group_by_key(|sd| sd.guard_id);

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
    let log_entries = parse_log("inputs/day4.txt");
    let sleep_durations = collect_sleep_durations(&log_entries);
    let ((guard_id, min),_) = sleep_durations
        .iter()
        .map(|sd| (sd.start..sd.start + sd.duration).map(move |m| (sd.guard_id, m)))
        .flatten()
        .group_by_key(|x| *x)
        .into_iter()
        .max_by_key(|(_,x)| x.len())
        .unwrap();
    
    guard_id * min
}
