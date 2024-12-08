#![feature(test)]

use utils::test_and_bench;

type Solution = i32;
pub type ParseOutput = Vec<Vec<Solution>>;
const MAIN_INPUT: &str = include_str!("main_input");

pub fn parse(file: &str) -> ParseOutput {
    file.lines()
        .map(|line| {
            line.split(" ")
                .map(|c| c.parse::<Solution>().unwrap())
                .collect()
        })
        .collect()
}

fn part_1(output: &ParseOutput) -> Solution {
    output.iter().filter(is_safe).count() as Solution
}

fn is_safe(v: &&Vec<Solution>) -> bool {
    let all_diff_at_most_3 = v.windows(2).all(|w| w[1].abs_diff(w[0]) <= 3);
    let all_ascending = v.windows(2).all(|w| w[1] > w[0]);
    let all_descending = v.windows(2).all(|w| w[1] < w[0]);
    all_diff_at_most_3 && (all_ascending || all_descending)
}

fn part_2(output: &ParseOutput) -> Solution {
    output
        .iter()
        .filter(|v| {
            (0..v.len())
                .map(|i| {
                    v.iter()
                        .enumerate()
                        .filter_map(|(j, &x)| if i == j { None } else { Some(x) })
                        .collect::<Vec<Solution>>()
                })
                .any(|v| is_safe(&&v))
        })
        .count() as Solution
}

fn main() {
    let parse_output = &mut parse(MAIN_INPUT);
    println!("Part 1: {}", part_1(parse_output));
    println!("Part 2: {}", part_2(parse_output));
}

test_and_bench! {
    TEST_INPUT == "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
    for tests: {
        part_1: { TEST_INPUT => 2 },
        part_2: { TEST_INPUT => 4 },
    },
    bench1 == 663,
    bench2 == 692,
    bench_parse: |v: ParseOutput| v.len() => 1000,
}
