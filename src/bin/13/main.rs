#![feature(test)]

use utils::{solve_linear_equation_2, test_and_bench};

type Solution = i64;
pub type ParseOutput = Vec<(Solution, Solution, Solution, Solution, Solution, Solution)>;
const MAIN_INPUT: &str = include_str!("main_input");

pub fn parse(file: &str) -> ParseOutput {
    let mut out = Vec::new();
    for l in file.split("\n\n") {
        let mut lines = l.lines();
        let next_line = lines.next().unwrap();
        let (a_x, a_y) = (
            next_line[12..14].parse().unwrap(),
            next_line[18..20].parse().unwrap(),
        );
        let next_line = lines.next().unwrap();
        let (b_x, b_y) = (
            next_line[12..14].parse().unwrap(),
            next_line[18..20].parse().unwrap(),
        );
        let (p_x_s, p_y_s) = lines.next().unwrap()[8..].split_once(", ").unwrap();
        out.push((
            a_x,
            a_y,
            b_x,
            b_y,
            p_x_s[1..].parse().unwrap(),
            p_y_s[2..].parse().unwrap(),
        ));
    }
    out
}

fn solve_part(output: &ParseOutput, addition: Solution) -> Solution {
    output
        .iter()
        .filter_map(|(a_x, a_y, b_x, b_y, p_x, p_y)| {
            let (a, b) = solve_linear_equation_2(
                (*p_x + addition) as f64,
                (*p_y + addition) as f64,
                *a_x as f64,
                *a_y as f64,
                *b_x as f64,
                *b_y as f64,
            )?;

            if a.fract() != 0.0f64 || b.fract() != 0.0f64 {
                return None;
            }

            Some((a * 3f64 + b) as Solution)
        })
        .sum()
}

fn part_1(output: &ParseOutput) -> Solution {
    solve_part(output, 0)
}
fn part_2(output: &ParseOutput) -> Solution {
    solve_part(output, 10000000000000)
}

fn main() {
    let parse_output = &mut parse(MAIN_INPUT);
    println!("Part 1: {}", part_1(parse_output));
    println!("Part 2: {}", part_2(parse_output));
}

test_and_bench! {
    TEST_INPUT == "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"
    for tests: {
        part_1: { TEST_INPUT => 480 },
        part_2: { TEST_INPUT => 0 },
    },
    bench1 == 27157,
    bench2 == 104015411578548,
    bench_parse: |v: ParseOutput| v.len() => 320,
}
