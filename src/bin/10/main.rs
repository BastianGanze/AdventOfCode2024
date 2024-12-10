#![feature(test)]

use itertools::Itertools;
use utils::test_and_bench;

type Solution = i32;
pub type ParseOutput = Vec<Vec<Solution>>;
const MAIN_INPUT: &str = include_str!("main_input");

pub fn parse(file: &str) -> ParseOutput {
    file.lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as Solution)
                .collect()
        })
        .collect()
}

fn part_1(output: &ParseOutput) -> Solution {
    let mut open_trails: Vec<(isize, isize, Solution)> = Vec::new();
    let mut confirmed_trails: Solution = 0;
    let mut reached_ends = Vec::new();
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    for y in 0..output.len() {
        for x in 0..output[y].len() {
            let current_height = get_at_pos(output, y as isize, x as isize).unwrap();
            if current_height == 0 {
                open_trails.push((y as isize, x as isize, current_height));
            }
            reached_ends.clear();
            while let Some(current_trail) = open_trails.pop() {
                let mut current_position = Some(current_trail);
                while let Some((y, x, current)) = current_position {
                    if current == 9 {
                        reached_ends.push((y, x));
                        break;
                    }
                    let mut used_for_current_trail = false;
                    for (dy, dx) in directions.iter() {
                        if let Some(next) = get_at_pos(output, y + dy, x + dx) {
                            if next == current + 1 {
                                if used_for_current_trail {
                                    open_trails.push((y + dy, x + dx, next));
                                } else {
                                    current_position = Some((y + dy, x + dx, next));
                                    used_for_current_trail = true;
                                }
                            }
                        }
                    }

                    if !used_for_current_trail {
                        current_position = None;
                    }
                }
            }
            let rank = reached_ends.iter().sorted().dedup().count() as Solution;
            confirmed_trails += rank;
        }
    }
    confirmed_trails
}

fn part_2(output: &ParseOutput) -> Solution {
    let mut open_trails: Vec<(isize, isize, Solution)> = Vec::new();
    let mut possible_paths: Solution = 0;
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    for y in 0..output.len() {
        for x in 0..output[y].len() {
            let current_height = get_at_pos(output, y as isize, x as isize).unwrap();
            if current_height == 0 {
                open_trails.push((y as isize, x as isize, current_height));
            }
            while let Some(current_trail) = open_trails.pop() {
                let mut current_position = Some(current_trail);
                while let Some((y, x, current)) = current_position {
                    if current == 9 {
                        possible_paths += 1;
                        break;
                    }
                    let mut used_for_current_trail = false;
                    for (dy, dx) in directions.iter() {
                        if let Some(next) = get_at_pos(output, y + dy, x + dx) {
                            if next == current + 1 {
                                if used_for_current_trail {
                                    open_trails.push((y + dy, x + dx, next));
                                } else {
                                    current_position = Some((y + dy, x + dx, next));
                                    used_for_current_trail = true;
                                }
                            }
                        }
                    }

                    if !used_for_current_trail {
                        current_position = None;
                    }
                }
            }
        }
    }
    possible_paths
}

fn get_at_pos(output: &ParseOutput, y: isize, x: isize) -> Option<Solution> {
    output
        .get(y as usize)
        .and_then(|l| l.get(x as usize))
        .cloned()
}

fn main() {
    let parse_output = &mut parse(MAIN_INPUT);
    println!("Part 1: {}", part_1(parse_output));
    println!("Part 2: {}", part_2(parse_output));
}

test_and_bench! {
    TEST_INPUT == "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
    for tests: {
        part_1: { TEST_INPUT => 36 },
        part_2: { TEST_INPUT => 0 },
    },
    bench1 == 535,
    bench2 == 1186,
    bench_parse: |v: ParseOutput| v.len() => 41,
}
