#![feature(test)]

use utils::{test_and_bench};

type Solution = i32;
pub type ParseOutput = Vec<String>;
const MAIN_INPUT: &str = include_str!("main_input");

pub fn parse(file: &str) -> ParseOutput {
    file.lines().map(|line| line.to_string()).collect()
}

fn part_1(output: &ParseOutput) -> Solution {
    0
}

fn part_2(output: &ParseOutput) -> Solution {
    0
}

fn main() {
    let parse_output = &mut parse(MAIN_INPUT);
    println!("Part 1: {}", part_1(parse_output));
    println!("Part 2: {}", part_2(parse_output));
}

test_and_bench! {
    TEST_INPUT == ""
    for tests: {
        part_1: { TEST_INPUT => 0 },
        part_2: { TEST_INPUT => 0 },
    },
    bench1 == 0,
    bench2 == 0,
    bench_parse: |v: ParseOutput| v.len() => 0,
}
