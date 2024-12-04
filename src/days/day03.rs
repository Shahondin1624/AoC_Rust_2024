use crate::{Solution, SolutionPair};
use regex::Regex;
use crate::etc::read_input;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input = read_input("day_03_1");
    let sol1: u64 = sol1(&input);
    let sol2: u64 = sol2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn sol1(input: &String) -> u64 {
    let instruction_regex = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let number_regex = Regex::new(r"(\d+)").unwrap();
    input.lines().map(|line| {
        instruction_regex.find_iter(line).map(|cap| {
            let (num1, num2) = extract_numbers(cap.as_str(), &number_regex);
            num1 * num2
        }).sum::<u64>()
    }).sum()
}
fn extract_numbers(line: &str, matcher: &Regex) -> (u64, u64) {
    let mut numbers = matcher.find_iter(line).map(|cap| cap.as_str().parse().unwrap());
    (numbers.next().unwrap(), numbers.next().unwrap())
}

fn sol2(input: &str) -> u64 {
    let regex = Regex::new(r"(mul\(\d+,\d+\)|do\(\)|don't\(\))").unwrap();
    let instruction_regex = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let number_regex = Regex::new(r"(\d+)").unwrap();
    let mut mul_enabled = true;
    let mut result = 0;
    for line in input.lines() {
        for cap in regex.find_iter(line) {
            let mat = cap.as_str();
            match mat {
                _ if instruction_regex.is_match(mat) => {
                    if mul_enabled {
                        let (left, right) = extract_numbers(mat, &number_regex);
                        result += left * right;
                    }
                }
                "do()" => mul_enabled = true,
                "don't()" => mul_enabled = false,
                _ => {}
            }
        }
    }
    result
}