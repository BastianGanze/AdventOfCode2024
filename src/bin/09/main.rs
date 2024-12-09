#![feature(test)]

use std::iter;
use utils::test_and_bench;

type Solution = u64;
pub type ParseOutput = Vec<usize>;
const MAIN_INPUT: &str = include_str!("main_input");

pub fn parse(file: &str) -> ParseOutput {
    file.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

fn part_1(output: &ParseOutput) -> Solution {
    let mut written_out = Vec::<Option<Solution>>::new();
    let mut id = 0;
    let mut min_i = 0;
    for w in output.chunks(2) {
        match w {
            [a, b] => {
                written_out.extend(iter::repeat_n(Some(id), *a));
                written_out.extend(iter::repeat_n(None, *b));
                min_i += *b;
            }
            [a] => {
                written_out.extend(iter::repeat_n(Some(id), *a));
            }
            _ => panic!("Invalid chunk"),
        }
        id += 1;
    }
    for i in (min_i..written_out.len()).rev() {
        let next_i = written_out.iter().position(|x| x.is_none()).unwrap();
        written_out.swap(i, next_i);
    }

    written_out
        .iter()
        .filter_map(|a| *a)
        .enumerate()
        .map(|(i, a)| i as u64 * a)
        .sum::<Solution>() as Solution
}

fn part_2(output: &ParseOutput) -> Solution {
    let mut written_out = Vec::<Option<Solution>>::new();
    let mut id = 0;
    let mut id_to_blocksize = Vec::<usize>::new();
    for w in output.chunks(2) {
        match w {
            [a, b] => {
                written_out.extend(iter::repeat_n(Some(id), *a));
                id_to_blocksize.push(*a);
                written_out.extend(iter::repeat_n(None, *b));
            }
            [a] => {
                written_out.extend(iter::repeat_n(Some(id), *a));
                id_to_blocksize.push(*a);
            }
            _ => panic!("Invalid chunk"),
        }
        id += 1;
    }
    for i in (0..written_out.len()).rev() {
        if let Some(current_block_id) = written_out[i] {
            let block_size = id_to_blocksize[current_block_id as usize];
            if let Some(next_free_space_i) = find_next_free_fitting_space(&written_out, block_size)
            {
                if next_free_space_i > i {
                    continue;
                }
                for j in 0..block_size {
                    written_out.swap(i - j, next_free_space_i + j);
                }
            }
        }
    }

    written_out
        .iter()
        .enumerate()
        .map(|(i, a)| i as u64 * a.unwrap_or(0))
        .sum::<Solution>() as Solution
}

fn find_next_free_fitting_space(
    written_out: &Vec<Option<Solution>>,
    min_space: usize,
) -> Option<usize> {
    for i in 0..written_out.len() {
        if written_out[i..].iter().take_while(|x| x.is_none()).count() >= min_space {
            return Some(i);
        }
    }
    None
}

fn main() {
    let parse_output = &mut parse(MAIN_INPUT);
    println!("Part 1: {}", part_1(parse_output));
    println!("Part 2: {}", part_2(parse_output));
}

test_and_bench! {
    TEST_INPUT == "2333133121414131402"
    for tests: {
        part_1: { TEST_INPUT => 1928 },
        part_2: { TEST_INPUT => 2858 },
    },
    bench1 == 6398608069280,
    bench2 == 0,
    bench_parse: |v: ParseOutput| v.len() => 0,
}
