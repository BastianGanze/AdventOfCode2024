#![feature(test)]

use std::collections::{HashMap, HashSet};
use utils::test_and_bench;

type Solution = i32;
pub type ParseOutput = Vec<Vec<char>>;
const MAIN_INPUT: &str = include_str!("main_input");

pub fn parse(file: &str) -> ParseOutput {
    file.lines().map(|l| l.chars().collect()).collect()
}

type PlotMap = HashMap<char, (Solution, Solution)>;

fn part_1(output: &ParseOutput) -> Solution {
    let mut plots_used = HashSet::new();
    let directions: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut total_price = 0;
    for y in 0..output.len() {
        for x in 0..output[y].len() {
            if plots_used.contains(&(y as isize, x as isize)) {
                continue;
            }
            let mut open_plots = vec![(y as isize, x as isize)];
            let current_plot = output[y][x];
            let mut area = 0;
            let mut perimeter = 0;
            while let Some((current_y, current_x)) = open_plots.pop() {
                if plots_used.contains(&(current_y, current_x)) {
                    continue;
                }
                plots_used.insert((current_y, current_x));
                area += 1;
                for (dy, dx) in directions.iter() {
                    let next_field = (current_y + dy, current_x + dx);
                    perimeter += match get_char_at_pos(output, next_field.0, next_field.1) {
                        Some(next_plot) => {
                            if next_plot == current_plot {
                                open_plots.push(next_field);
                                0
                            } else {
                                1
                            }
                        }
                        None => 1,
                    };
                }
            }
            total_price += area * perimeter;
        }
    }
    total_price
}

fn get_char_at_pos(output: &ParseOutput, y: isize, x: isize) -> Option<char> {
    output
        .get(y as usize)
        .and_then(|l| l.get(x as usize).copied())
}

fn part_2(output: &ParseOutput) -> Solution {
    let mut plots_used = HashSet::new();
    let directions: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut total_price = 0;
    for y in 0..output.len() {
        for x in 0..output[y].len() {
            if plots_used.contains(&(y as isize, x as isize)) {
                continue;
            }
            let mut distinct_surfaces: Vec<((isize, isize), (isize, isize))> = vec![];
            let mut area = 0;
            let mut open_plots = vec![(y as isize, x as isize)];
            let current_plot = output[y][x];
            while let Some((current_y, current_x)) = open_plots.pop() {
                if plots_used.contains(&(current_y, current_x)) {
                    continue;
                }
                plots_used.insert((current_y, current_x));
                area += 1;
                for (dy, dx) in directions.iter() {
                    let next_field = (current_y + dy, current_x + dx);
                    if let Some(next_plot) = get_char_at_pos(output, next_field.0, next_field.1) {
                        if next_plot == current_plot {
                            open_plots.push(next_field);
                        } else {
                            distinct_surfaces.push(((current_y, current_x), (*dy, *dx)));
                        }
                    } else {
                        distinct_surfaces.push(((current_y, current_x), (*dy, *dx)));
                    }
                }
            }
            let mut sides_n = 0;
            while let Some((surface_pos, surface_normal)) = distinct_surfaces.pop() {
                let ((dy_1, dx_1), (dy_2, dx_2)) = if surface_normal.0 == 0 {
                    ((1, 0), (-1, 0))
                } else {
                    ((0, 1), (0, -1))
                };
                for (dy, dx) in [(dy_1, dx_1), (dy_2, dx_2)].iter() {
                    let mut removed = true;
                    let mut next_pos = surface_pos;
                    while removed {
                        let prev_len = distinct_surfaces.len();
                        next_pos = (next_pos.0 + dy, next_pos.1 + dx);
                        distinct_surfaces.retain(|s| s != &(next_pos, surface_normal));
                        removed = prev_len != distinct_surfaces.len();
                    }
                }
                sides_n += 1;
            }
            total_price += sides_n * area;
        }
    }
    total_price
}

fn main() {
    let parse_output = &mut parse(MAIN_INPUT);
    println!("Part 1: {}", part_1(parse_output));
    println!("Part 2: {}", part_2(parse_output));
}

test_and_bench! {
    TEST_INPUT == "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
    TEST_INPUT_2 == "AAAA
BBCD
BBCC
EEEC"
    for tests: {
        part_1: { TEST_INPUT => 1930 },
        part_2: { TEST_INPUT_2 => 80 },
    },
    bench1 == 1421958,
    bench2 == 885394,
    bench_parse: |v: ParseOutput| v.len() => 140,
}
