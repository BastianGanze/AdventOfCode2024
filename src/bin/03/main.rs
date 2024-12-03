#![feature(test)]

use regex::Regex;
use utils::{get_day, get_session, test_and_bench, try_submit};

type Solution = i32;
pub type ParseOutput = Vec<Vec<(Solution, Solution, bool)>>;
const MAIN_INPUT: &str = include_str!("main_input");

pub fn parse(file: &str) -> ParseOutput {
    let re = Regex::new(r"mul\(([0-9]*?,[0-9]*?)\)|don't\(\)|do\(\)").unwrap();
    let mut active = true;
    file.lines()
        .map(|line| {
            re.find_iter(line)
                .filter_map(|m| match m.as_str() {
                    "don't()" => {
                        active = false;
                        None
                    }
                    "do()" => {
                        active = true;
                        None
                    }
                    s => {
                        let nums = s[4..]
                            .split_once(",")
                            .map(|(n1, n2)| {
                                (
                                    n1.parse::<Solution>().unwrap(),
                                    n2[..n2.len() - 1].parse::<Solution>().unwrap(),
                                )
                            })
                            .unwrap();
                        Some((nums.0, nums.1, active))
                    }
                })
                .collect()
        })
        .collect()
}

fn part_1(output: &ParseOutput) -> Solution {
    output
        .iter()
        .map(|v| v.iter().map(|(a, b, _)| a * b).sum::<Solution>())
        .sum()
}

fn part_2(output: &ParseOutput) -> Solution {
    output
        .iter()
        .map(|v| {
            v.iter()
                .filter_map(|(a, b, is_active)| is_active.then_some(a * b))
                .sum::<Solution>()
        })
        .sum()
}

#[tokio::main]
async fn main() {
    let parse_output = &mut parse(MAIN_INPUT);
    let session = get_session(get_day());
    try_submit(&session, 1, format!("{}", part_1(parse_output))).await;
    try_submit(&session, 2, format!("{}", part_2(parse_output))).await;
}

test_and_bench! {
    TEST_INPUT == "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
    TEST_INPUT_2 == "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
    for tests: {
        part_1: { TEST_INPUT => 161 },
        part_2: { TEST_INPUT_2 => 48 },
    },
    bench1 == 164730528,
    bench2 == 70478672,
    bench_parse: |v: ParseOutput| v.len() => 6,
}
