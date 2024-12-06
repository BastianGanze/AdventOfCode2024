#![feature(test)]

use std::collections::HashSet;
use utils::{get_day, get_session, test_and_bench, try_submit};

type Solution = i32;
pub type ParseOutput = Vec<Vec<char>>;
const MAIN_INPUT: &str = include_str!("main_input");

pub fn parse(file: &str) -> ParseOutput {
    file.lines().map(|line| line.chars().collect()).collect()
}
fn part_1(output: &ParseOutput) -> Solution {
    let mut visited = HashSet::<(isize, isize)>::new();
    let width = output[0].len() as isize;
    let height = output.len() as isize;
    let mut guard_position = output
        .iter()
        .enumerate()
        .find_map(|(y, l)| {
            l.contains(&'^').then(|| {
                (
                    y as isize,
                    l.iter()
                        .enumerate()
                        .find_map(|(x, c)| (c == &'^').then_some(x as isize))
                        .unwrap(),
                )
            })
        })
        .unwrap();
    let mut guard_direction = (-1, 0);
    while guard_position.0 < width
        && guard_position.1 < height
        && guard_position.0 >= 0
        && guard_position.1 >= 0
    {
        visited.insert(guard_position);
        let (next_y, next_x) = (
            guard_position.0 + guard_direction.0,
            guard_position.1 + guard_direction.1,
        );
        if is_wall(output, next_y, next_x) {
            guard_direction = match guard_direction {
                (-1, 0) => (0, 1),
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                _ => unreachable!(),
            };
        } else {
            guard_position = (next_y, next_x);
        }
    }
    visited.len() as Solution
}

fn is_wall(output: &ParseOutput, y: isize, x: isize) -> bool {
    if x < 0 || y < 0 {
        return false;
    }
    output
        .get(y as usize)
        .and_then(|l| l.get(x as usize))
        .map_or(false, |&v| v == '#')
}

type Visited = ((isize, isize), (isize, isize));

fn part_2(output: &ParseOutput) -> Solution {
    let mut changing_output = output.clone();
    let width = output[0].len() as isize;
    let height = output.len() as isize;
    let mut loops = 0;
    let mut visited_fields = HashSet::<Visited>::new();
    let guard_position = output
        .iter()
        .enumerate()
        .find_map(|(y, l)| {
            l.contains(&'^').then(|| {
                (
                    y as isize,
                    l.iter()
                        .enumerate()
                        .find_map(|(x, c)| (c == &'^').then_some(x as isize))
                        .unwrap(),
                )
            })
        })
        .unwrap();

    for y in 0..height {
        for x in 0..width {
            if changing_output[y as usize][x as usize] != '.' {
                continue;
            }
            visited_fields.clear();
            changing_output[y as usize][x as usize] = '#';
            let mut guard_pos = guard_position;
            let mut guard_direction = (-1, 0);
            while guard_pos.0 < width
                && guard_pos.1 < height
                && guard_pos.0 >= 0
                && guard_pos.1 >= 0
            {
                if visited_fields.contains(&(guard_pos, guard_direction)) {
                    loops += 1;
                    break;
                }
                let (next_y, next_x) = (
                    guard_pos.0 + guard_direction.0,
                    guard_pos.1 + guard_direction.1,
                );
                if is_wall(&changing_output, next_y, next_x) {
                    guard_direction = match guard_direction {
                        (-1, 0) => (0, 1),
                        (0, 1) => (1, 0),
                        (1, 0) => (0, -1),
                        (0, -1) => (-1, 0),
                        _ => unreachable!(),
                    };
                } else {
                    visited_fields.insert((guard_pos, guard_direction));
                    guard_pos = (next_y, next_x);
                }
            }
            changing_output[y as usize][x as usize] = '.';
        }
    }
    loops
}

#[tokio::main]
async fn main() {
    let parse_output = &mut parse(MAIN_INPUT);
    let session = get_session(get_day());
    try_submit(&session, 1, format!("{}", part_1(parse_output))).await;
    try_submit(&session, 2, format!("{}", part_2(parse_output))).await;
}

test_and_bench! {
    TEST_INPUT == "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
    for tests: {
        part_1: { TEST_INPUT => 41 },
        part_2: { TEST_INPUT => 6 },
    },
    bench1 == 5404,
    bench2 == 0,
    bench_parse: |v: ParseOutput| v.len() => 130,
}
