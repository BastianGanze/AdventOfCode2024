#![feature(test)]

use std::cmp::Ordering;
use std::collections::HashSet;
use utils::{get_day, get_session, test_and_bench, try_submit};

type Solution = i32;
pub type Rules = Vec<(Solution, Solution)>;
pub type Pages = Vec<Vec<Solution>>;
pub type ParseOutput = (Rules, Pages);
const MAIN_INPUT: &str = include_str!("main_input");

pub fn parse(file: &str) -> ParseOutput {
    let (rules_str, pages_str) = file.split_once("\n\n").unwrap();
    let rules = rules_str
        .lines()
        .map(|line| {
            let (a, b) = line.split_once("|").unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();

    let pages = pages_str
        .lines()
        .map(|line| line.split(",").map(|c| c.parse().unwrap()).collect())
        .collect();

    (rules, pages)
}

fn part_1((rules, batches): &ParseOutput) -> Solution {
    let mut printed = HashSet::<Solution>::new();
    let mut rules_for_batch: Rules = Vec::new();
    batches
        .iter()
        .cloned()
        .filter(|batch| is_batch_valid(rules, &mut printed, &mut rules_for_batch, &batch))
        .map(|batch| batch[((batch.len() + 1) / 2) - 1])
        .sum()
}

fn part_2((rules, batches): &ParseOutput) -> Solution {
    let mut ordered_rules: Rules = Vec::new();
    let mut printed = HashSet::<Solution>::new();
    let mut rules_for_batch: Rules = Vec::new();
    batches
        .iter()
        .filter_map(|batch| {
            if is_batch_valid(rules, &mut printed, &mut rules_for_batch, &batch) {
                return None;
            }
            put_ordered_rules_for_batch(batch, rules, &mut ordered_rules);
            let mut ordered_batch = batch.clone();
            ordered_batch.sort_by(|a, b| {
                match (
                    ordered_rules.iter().position(|(x, _)| *x == *a),
                    ordered_rules.iter().position(|(x, _)| *x == *b),
                ) {
                    (Some(a_index), Some(b_index)) => a_index.cmp(&b_index),
                    (Some(_), None) => Ordering::Less,
                    (None, Some(_)) => Ordering::Greater,
                    (None, None) => panic!("No index found"),
                }
            });
            Some(ordered_batch[((ordered_batch.len() + 1) / 2) - 1])
        })
        .sum()
}

fn put_ordered_rules_for_batch(batch: &Vec<Solution>, rules: &Rules, ordered_rules: &mut Rules) {
    ordered_rules.clear();
    let mut rules_tmp = rules
        .iter()
        .filter(|(prev, next)| batch.contains(prev) && batch.contains(next))
        .cloned()
        .collect::<Rules>();
    while !rules_tmp.is_empty() {
        for i in 0..rules_tmp.len() {
            let (a, b) = rules_tmp[i];
            let has_any_ingoing = rules_tmp.iter().any(|(_, after)| a == *after);
            if !has_any_ingoing {
                ordered_rules.push((a, b));
                rules_tmp.remove(i);
                break;
            }
        }
    }
}

fn is_batch_valid(
    rules: &Rules,
    printed: &mut HashSet<Solution>,
    rules_for_batch: &mut Rules,
    batch: &&Vec<Solution>,
) -> bool {
    printed.clear();
    rules_for_batch.clear();
    rules_for_batch.extend(
        rules
            .iter()
            .filter(|(prev, next)| batch.contains(prev) && batch.contains(next)),
    );

    let mut batch_valid = true;
    let mut i = 0;
    'outer: while i < batch.len() {
        for (_, next) in rules.iter().filter(|(p, _)| *p == batch[i]) {
            if printed.contains(next) {
                batch_valid = false;
                break 'outer;
            }
        }
        printed.insert(batch[i]);
        i += 1;
    }
    batch_valid
}

#[tokio::main]
async fn main() {
    let parse_output = &mut parse(MAIN_INPUT);
    let session = get_session(get_day());
    try_submit(&session, 1, format!("{}", part_1(parse_output))).await;
    try_submit(&session, 2, format!("{}", part_2(parse_output))).await;
}

test_and_bench! {
    TEST_INPUT == "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
    for tests: {
        part_1: { TEST_INPUT => 143 },
        part_2: { TEST_INPUT => 123 },
    },
    bench1 == 5991,
    bench2 == 5479,
    bench_parse: |(rules, pages): ParseOutput| {assert_eq!(pages.len(), 200); rules.len()} => 1176,
}
