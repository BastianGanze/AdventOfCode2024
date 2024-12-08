#![feature(test)]

use itertools::Itertools;

use utils::test_and_bench;

type Solution = i64;
pub type ParseOutput = Vec<(Solution, Vec<Solution>)>;
const MAIN_INPUT: &str = include_str!("main_input");

pub fn parse(file: &str) -> ParseOutput {
    file.lines()
        .map(|line| {
            let (key, values) = line.split_once(": ").unwrap();
            (
                key.parse().unwrap(),
                values.split(' ').map(|v| v.parse().unwrap()).collect(),
            )
        })
        .collect()
}

fn part_1(output: &ParseOutput) -> Solution {
    output
        .iter()
        .filter_map(|(solution, numbers)| {
            let combinations = 1 << numbers.len() - 1;
            (0..combinations)
                .any(|operations| {
                    *solution
                        == numbers
                            .iter()
                            .cloned()
                            .enumerate()
                            .reduce(|(_, acc), (operation_i, n)| {
                                match (operations >> operation_i - 1) & 1 {
                                    1 => (0, acc + n),
                                    _ => (0, acc * n),
                                }
                            })
                            .map(|(_, s)| s)
                            .unwrap()
                })
                .then_some(*solution)
        })
        .sum()
}

fn part_2(output: &ParseOutput) -> Solution {
    output
        .iter()
        .filter_map(|(solution, numbers)| {
            let operator_positions = numbers.len() - 1;
            itertools::repeat_n(['+', '*', '|'], operator_positions)
                .multi_cartesian_product()
                .any(|operations_map| {
                    *solution
                        == numbers
                            .iter()
                            .cloned()
                            .enumerate()
                            .reduce(|(_, acc), (i, n)| {
                                let operation = operations_map[i - 1];
                                match operation {
                                    '+' => (0, acc + n),
                                    '*' => (0, acc * n),
                                    '|' => (0, acc * 10u64.pow(n.ilog10() + 1) as Solution + n),
                                    _ => panic!("Invalid operation"),
                                }
                            })
                            .map(|(_, sol)| sol)
                            .unwrap()
                })
                .then_some(*solution)
        })
        .sum()
}

fn main() {
    let parse_output = &mut parse(MAIN_INPUT);
    println!("Part 1: {}", part_1(parse_output));
    println!("Part 2: {}", part_2(parse_output));
}

test_and_bench! {
    TEST_INPUT == "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
    for tests: {
        part_1: { TEST_INPUT => 3749 },
        part_2: { TEST_INPUT => 11387 },
    },
    bench1 == 2501605301465,
    bench2 == 44841372855953,
    bench_parse: |v: ParseOutput| v.len() => 850,
}
