#![feature(test)]

use utils::test_and_bench;

type Solution = i32;
pub type ParseOutput = (Vec<i32>, Vec<i32>);
const MAIN_INPUT: &str = include_str!("main_input");

pub fn parse(file: &str) -> ParseOutput {
    let mut num_1 = Vec::new();
    let mut num_2 = Vec::new();
    for line in file.lines() {
        let (n1, n2) = line.split_once("   ").unwrap();
        num_1.push(n1.parse().unwrap());
        num_2.push(n2.parse().unwrap());
    }
    num_1.sort();
    num_2.sort();
    (num_1, num_2)
}

fn part_1((num_1, num_2): &ParseOutput) -> Solution {
    num_1
        .iter()
        .enumerate()
        .fold(0, |acc, (i, n)| acc + (num_2[i] - n).abs() as Solution)
}

fn part_2((num_1, num_2): &ParseOutput) -> Solution {
    num_1
        .iter()
        .fold(0, |acc, n| acc + n * get_count_in_list(num_2, *n))
}

fn get_count_in_list(list: &Vec<Solution>, n: Solution) -> Solution {
    match list.binary_search(&n) {
        Ok(i) => {
            let mut left = i;
            let mut right = i;

            while left > 0 && list[left - 1] == n {
                left -= 1;
            }

            while right < list.len() - 1 && list[right + 1] == n {
                right += 1;
            }

            (right - left + 1) as Solution
        }
        Err(_) => 0,
    }
}

fn main() {
    let parse_output = &mut parse(MAIN_INPUT);
    println!("Part 1: {}", part_1(parse_output));
    println!("Part 2: {}", part_2(parse_output));
}

test_and_bench! {
    TEST_INPUT == "3   4
4   3
2   5
1   3
3   9
3   3"
    for tests: {
        part_1: { TEST_INPUT => 11 },
        part_2: { TEST_INPUT => 31 },
    },
    bench1 == 1341714,
    bench2 == 27384707,
    bench_parse: |(v1, v2): (Vec<Solution>, Vec<Solution>)| {v1.len() + v2.len()} => 2000,
}
