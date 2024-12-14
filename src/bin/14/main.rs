#![feature(test)]

use std::collections::HashSet;
use utils::test_and_bench;

type Solution = i32;
pub type ParseOutput = Vec<Robot>;
const MAIN_INPUT: &str = include_str!("main_input");
pub type Robot = (Solution, Solution, Solution, Solution);
pub fn parse(file: &str) -> ParseOutput {
    file.lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let (p_x, p_y) = parts.next().unwrap().split_once(',').unwrap();
            let (v_x, v_y) = parts.next().unwrap().split_once(',').unwrap();
            (
                p_x[2..].parse().unwrap(),
                p_y.parse().unwrap(),
                v_x[2..].parse().unwrap(),
                v_y.parse().unwrap(),
            )
        })
        .collect()
}

fn solve_1(output: &ParseOutput, w: Solution, h: Solution, steps: Solution) -> Solution {
    let quadrants = output
        .iter()
        .map(|(p_x, p_y, v_x, v_y)| {
            (
                (((p_x + v_x * steps) % w) + w) % w,
                (((p_y + v_y * steps) % h) + h) % h,
            )
        })
        .fold([0, 0, 0, 0], |mut acc, (x, y)| {
            if x >= 0 && x < w / 2 && y >= 0 && y < h / 2 {
                acc[0] += 1;
            }
            if x > w / 2 && x <= w && y >= 0 && y < h / 2 {
                acc[1] += 1;
            }
            if x >= 0 && x < w / 2 && y > h / 2 && y <= h {
                acc[2] += 1;
            }
            if x > w / 2 && x <= w && y > h / 2 && y <= h {
                acc[3] += 1;
            }
            acc
        });

    quadrants.iter().product()
}

fn get_christmas_tree(output: &mut ParseOutput, w: Solution, h: Solution) -> Solution {
    let mut steps = 0;
    let mut robot_start_hashset: HashSet<(Solution, Solution)> = HashSet::new();
    while !find_straight_line_of_robots_one_y_coordinate_apart_of_n_robots(
        &robot_start_hashset,
        w,
        h,
        10,
    ) {
        robot_start_hashset.clear();
        for (p_x, p_y, v_x, v_y) in output.iter_mut() {
            *p_x = (((*p_x + *v_x) % w) + w) % w;
            *p_y = (((*p_y + *v_y) % h) + h) % h;
            robot_start_hashset.insert((*p_x, *p_y));
        }
        steps += 1;
    }
    steps
}

fn find_straight_line_of_robots_one_y_coordinate_apart_of_n_robots(
    output: &HashSet<(Solution, Solution)>,
    w: Solution,
    h: Solution,
    n: Solution,
) -> bool {
    for x in 0..w {
        let mut y = 0;
        let mut count = 0;
        while y < h {
            if output.contains(&(x, y)) {
                count += 1;
                if count == n {
                    return true;
                }
                y += 1;
            } else {
                count = 0;
                y += 1;
            }
        }
    }
    false
}
fn all_robots_unique(output: &ParseOutput) -> bool {
    let mut robots = output.clone();
    robots.sort();
    robots.dedup();
    robots.len() == output.len()
}

fn print_map(output: &ParseOutput, w: Solution, h: Solution) {
    let mut map = vec![vec![' '; w as usize]; h as usize];
    for (p_x, p_y, _, _) in output {
        map[*p_y as usize][*p_x as usize] = '#';
    }
    for row in map {
        println!("{}", row.iter().collect::<String>());
    }
}

fn part_1(output: &ParseOutput) -> Solution {
    solve_1(output, 101, 103, 100)
}

fn part_1_test(output: &ParseOutput) -> Solution {
    solve_1(output, 11, 7, 100)
}

fn part_2(output: &ParseOutput) -> Solution {
    let mut robots = output.clone();
    get_christmas_tree(&mut robots, 101, 103)
}

fn main() {
    let parse_output = &mut parse(MAIN_INPUT);
    println!("Part 1: {}", part_1(parse_output));
    println!("Part 2: {}", part_2(parse_output));
}

test_and_bench! {
    TEST_INPUT == "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"
    for tests: {
        part_1_test: { TEST_INPUT => 12 },
        part_2: { TEST_INPUT => 8149 },
    },
    bench1 == 214400550,
    bench2 == 8149,
    bench_parse: |v: ParseOutput| v.len() => 0,
}
