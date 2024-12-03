use crate::etc::read_input;
use crate::{Solution, SolutionPair};
use hashbrown::HashMap;
use std::cmp::{max, min};
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let mut left_list: Vec<u64> = Vec::new();
    let mut right_list: Vec<u64> = Vec::new();

    read_input("day_01_1").split("\n").for_each(|line| {
        let split_line: Vec<&str> = line.split_whitespace().collect();
        left_list.push(
            split_line[0]
                .parse::<u64>()
                .expect("Could not extract left number"),
        );
        right_list.push(
            split_line[1]
                .parse::<u64>()
                .expect("Could not extract right number"),
        );
    });

    left_list.sort();
    right_list.sort();

    let sol1: u64 = sol1(&left_list, &right_list);
    let sol2: u64 = sol2(&left_list, &right_list);

    (Solution::from(sol1), Solution::from(sol2))
}

fn sol1(left_list: &Vec<u64>, right_list: &Vec<u64>) -> u64 {
    left_list
        .iter()
        .zip(right_list.iter())
        .map(|(left, right)| max(left, right) - min(left, right))
        .sum()
}

fn sol2(left_list: &Vec<u64>, right_list: &Vec<u64>) -> u64 {
    let left_map = transform(left_list);
    let right_map = transform(right_list);
    let mut similarity_score = 0;
    for entry in left_map.iter() {
        let right_occurrences = match right_map.get(entry.0) {
            None => 0,
            Some(val) => *val,
        };
        similarity_score += entry.0 * entry.1 * right_occurrences;
    }
    similarity_score
}

fn transform(vec: &Vec<u64>) -> HashMap<u64, u64> {
    let mut map: HashMap<u64, u64> = HashMap::new();
    for num in vec {
        *map.entry(*num).or_insert(0) += 1;
    }
    map
}
