use crate::days::day05::Order::{Ordered, Unordered};
use crate::etc::read_input;
use crate::{Solution, SolutionPair};
use std::cmp::Ordering;
use std::cmp::Ordering::Greater;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let (ruleset, instructions) = parse_input(read_input("day_05_1"));
    let sol1: u64 = sol1(&ruleset, &instructions);
    let sol2: u64 = sol2(&ruleset, &instructions);

    (Solution::from(sol1), Solution::from(sol2))
}

fn sol1(ruleset: &Ruleset, instructions: &Vec<InstructionUpdate>) -> u64 {
    instructions
        .iter()
        .filter(|instruction| instruction.is_ordered(ruleset))
        .map(|instruction| instruction.middle_page_number())
        .sum()
}

fn sol2(ruleset: &Ruleset, instructions: &Vec<InstructionUpdate>) -> u64 {
    instructions
        .iter()
        .filter(|instruction| !instruction.is_ordered(ruleset))
        .map(|instruction| instruction.order(ruleset))
        .map(|instruction| instruction.middle_page_number())
        .sum()
}

fn parse_input(input: String) -> (Ruleset, Vec<InstructionUpdate>) {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let ruleset = Ruleset::parse(parts.first().unwrap());
    let instructions = parts
        .last()
        .unwrap()
        .lines()
        .map(|line| InstructionUpdate::parse(line))
        .collect();
    (ruleset, instructions)
}

struct Ruleset {
    rules: Vec<Rule>,
}

impl Ruleset {
    fn parse(input: &str) -> Self {
        Ruleset {
            rules: input.split("\n").map(|line| Rule::parse(line)).collect(),
        }
    }
}

#[derive(Clone)]
struct Rule {
    first: u64,
    second: u64,
}

impl Rule {
    fn parse(input: &str) -> Self {
        let parts = input.split("|").collect::<Vec<&str>>();
        let first = parts[0].parse::<u64>().expect("Couldn't parse first");
        let second = parts[1].parse::<u64>().expect("Couldn't parse second");
        Rule { first, second }
    }

    fn are_in_order(&self, first: u64, second: u64) -> Order {
        if first == self.first && second == self.second {
            Ordered
        } else if first == self.second && second == self.first {
            Unordered
        } else {
            Order::Unspecified
        }
    }
}

#[derive(Debug, PartialEq)]
enum Order {
    Unordered,
    Ordered,
    Unspecified,
}

struct InstructionUpdate {
    pages: Vec<u64>,
}

impl InstructionUpdate {
    fn parse(input: &str) -> Self {
        let parts = input.split(",").collect::<Vec<&str>>();
        InstructionUpdate {
            pages: parts
                .iter()
                .map(|num| num.parse::<u64>().unwrap())
                .collect(),
        }
    }

    fn is_ordered(&self, ruleset: &Ruleset) -> bool {
        !self.pages.windows(2).into_iter().any(|window| {
            let first = window[0];
            let second = window[1];
            ruleset
                .rules
                .iter()
                .any(|rule| rule.are_in_order(first, second) == Unordered)
        })
    }

    fn middle_page_number(&self) -> u64 {
        self.pages[self.pages.len() / 2]
    }

    fn filter_rules(&self, ruleset: &Ruleset) -> Vec<Rule> {
        ruleset
            .rules
            .iter()
            .filter(|rule| self.pages.contains(&rule.first) || self.pages.contains(&rule.second))
            .map(|rule| rule.clone())
            .collect()
    }

    fn order(&self, ruleset: &Ruleset) -> InstructionUpdate {
        let rules = self.filter_rules(ruleset);
        let mut pages = self.pages.clone();
        pages.sort_by(|a, b| order(&rules, *a, *b));
        InstructionUpdate { pages }
    }
}

fn order(rules: &Vec<Rule>, first: u64, second: u64) -> Ordering {
    for rule in rules {
        match rule.are_in_order(first, second) {
            Ordered => return Greater,
            Unordered => return Ordering::Less,
            Order::Unspecified => continue,
        }
    }
    Ordering::Equal
}
