#![feature(test)]

use std::collections::HashMap;
use utils::test_and_bench;

type Solution = i64;
pub type ParseOutput = Vec<Solution>;
const MAIN_INPUT: &str = include_str!("main_input");

pub fn parse(file: &str) -> ParseOutput {
    file.split(' ').map(|s| s.parse().unwrap()).collect()
}

fn part_1(output: &ParseOutput) -> Solution {
    let mut evolution_paths = HashMap::new();
    let s: Solution = output
        .iter()
        .map(|n| calc_stones_for_number_memo(*n, 25, &mut evolution_paths))
        .sum();
    s
}

type StoneEvolutionMap = HashMap<(Solution, usize), Solution>;

fn part_2(output: &ParseOutput) -> Solution {
    let mut evolution_paths = HashMap::new();
    let s: Solution = output
        .iter()
        .map(|n| calc_stones_for_number_memo(*n, 75, &mut evolution_paths))
        .sum();
    s
}

fn calc_stones_for_number_memo(
    n: Solution,
    steps: usize,
    stone_evolution: &mut StoneEvolutionMap,
) -> Solution {
    if steps == 0 {
        return 1;
    }
    if !stone_evolution.contains_key(&(n, steps)) {
        let stones = match n {
            0 => calc_stones_for_number_memo(1, steps - 1, stone_evolution),
            n => {
                let digit_count = ((n as f64).log10().floor() as Solution) + 1;
                match digit_count % 2 {
                    0 => {
                        let divisor = 10_i64.pow((digit_count / 2) as u32);
                        let first_half = n / divisor;
                        let second_half = n % divisor;
                        calc_stones_for_number_memo(first_half, steps - 1, stone_evolution)
                            + calc_stones_for_number_memo(second_half, steps - 1, stone_evolution)
                    }
                    _ => calc_stones_for_number_memo(n * 2024, steps - 1, stone_evolution),
                }
            }
        };
        stone_evolution.insert((n, steps), stones);
        return stones;
    }

    *stone_evolution.get(&(n, steps)).unwrap()
}

fn main() {
    let parse_output = &mut parse(MAIN_INPUT);
    println!("Part 1: {}", part_1(parse_output));
    println!("Part 2: {}", part_2(parse_output));
}

test_and_bench! {
    TEST_INPUT == "125 17"
    for tests: {
        part_1: { TEST_INPUT => 55312 },
        part_2: { TEST_INPUT => 65601038650482 },
    },
    bench1 == 209412,
    bench2 == 248967696501656,
    bench_parse: |v: ParseOutput| v.len() => 8,
}
