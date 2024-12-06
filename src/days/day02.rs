use crate::days::day02::Direction::{Decreasing, Increasing, Undetermined};
use crate::etc::read_input;
use crate::{Solution, SolutionPair};
use std::cmp::{max, min, PartialEq};
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let records = get_record("day_02_1");

    let sol1: u64 = sol1(&records);
    let sol2: u64 = sol2(&records);

    (Solution::from(sol1), Solution::from(sol2))
}

fn sol1(reports: &Vec<Report>) -> u64 {
    reports.iter().filter(|report| is_safe_1(report)).count() as u64
}

fn sol2(_reports: &Vec<Report>) -> u64 {
    0
}

struct Report {
    levels: Vec<u64>,
}

impl From<&str> for Report {
    fn from(s: &str) -> Self {
        s.split_whitespace()
            .map(|word| word.parse().unwrap())
            .collect()
    }
}

impl FromIterator<u64> for Report {
    fn from_iter<I: IntoIterator<Item = u64>>(iter: I) -> Self {
        let levels: Vec<u64> = iter.into_iter().collect();
        Report { levels }
    }
}

fn is_safe_1(report: &Report) -> bool {
    let mut direction = Undetermined;
    for window in report.levels.windows(2) {
        if let [left, right] = window {
            let next_direction = get_direction(*left, *right);
            if direction != Undetermined && direction != next_direction {
                return false;
            }
            direction = next_direction;
            let difference = get_difference(*left, *right);
            if difference > 3 || difference < 1 {
                return false;
            }
        }
    }
    true
}

fn get_difference(left: u64, right: u64) -> u64 {
    max(left, right) - min(left, right)
}

fn is_safe_2(report: &Report) -> bool {
    let mut _directions: Vec<Direction> = Vec::new();
    let mut _differences: Vec<Direction> = Vec::new();
    for window in report.levels.windows(2) {
        if let [left, right] = window {
            let _direction = get_direction(*left, *right);
        }
    }
    false
}

fn get_direction(left: u64, right: u64) -> Direction {
    if left > right {
        Decreasing
    } else {
        Increasing
    }
}
#[derive(Debug, PartialEq)]
enum Direction {
    Increasing,
    Decreasing,
    Undetermined,
}


fn get_record(filename: &str) -> Vec<Report> {
    read_input(filename)
        .lines()
        .map(|line| Report::from(line))
        .collect()
}
