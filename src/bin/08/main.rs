#![feature(test)]

use fnv::FnvHashMap as HashMap;
use itertools::Itertools;
use utils::test_and_bench;

type Solution = i32;
type Antenna = (isize, isize);
pub type ParseOutput = (HashMap<char, Vec<Antenna>>, (isize, isize));
const MAIN_INPUT: &str = include_str!("main_input");

pub fn parse(file: &str) -> ParseOutput {
    let mut antennas: HashMap<char, Vec<Antenna>> = HashMap::default();
    let max_y = file.lines().count() as isize;
    let max_x = file.lines().next().unwrap().chars().count() as isize;
    for (y, line) in file.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            antennas
                .entry(c)
                .or_default()
                .push((y as isize, x as isize));
        }
    }
    (antennas, (max_y, max_x))
}

fn part_1((output, (max_y, max_x)): &ParseOutput) -> Solution {
    let mut antinode_locations = Vec::with_capacity(1000);
    for (_, antennas) in output.iter() {
        for i in 0..antennas.len() {
            for i_1 in i + 1..antennas.len() {
                let (y_1, x_1) = antennas[i];
                let (y_2, x_2) = antennas[i_1];
                let (y_dist, x_dist) = (y_2 - y_1, x_2 - x_1);
                let antinode_1 = (y_1 - y_dist, x_1 - x_dist);
                let antinode_2 = (y_2 + y_dist, x_2 + x_dist);
                antinode_locations.push(antinode_1);
                antinode_locations.push(antinode_2);
            }
        }
    }
    antinode_locations
        .iter()
        .sorted()
        .dedup()
        .filter(|(y, x)| *y >= 0 && y < max_y && *x >= 0 && x < max_x)
        .count() as Solution
}

fn part_2((output, (max_y, max_x)): &ParseOutput) -> Solution {
    let mut antinode_locations = Vec::with_capacity(1000);
    for (_, antennas) in output.iter() {
        for i in 0..antennas.len() {
            for i_1 in i + 1..antennas.len() {
                let (y_1, x_1) = antennas[i];
                let (y_2, x_2) = antennas[i_1];
                antinode_locations.push((y_1, x_1));
                antinode_locations.push((y_2, x_2));
                let (y_dist, x_dist) = (y_2 - y_1, x_2 - x_1);
                let mut a_1 = (y_1 - y_dist, x_1 - x_dist);
                let mut a_2 = (y_2 + y_dist, x_2 + x_dist);
                while a_1.0 >= 0 && a_1.1 >= 0 && a_1.0 < *max_y && a_1.1 < *max_x {
                    antinode_locations.push(a_1);
                    a_1 = (a_1.0 - y_dist, a_1.1 - x_dist);
                }
                while a_2.0 >= 0 && a_2.1 >= 0 && a_2.0 < *max_y && a_2.1 < *max_x {
                    antinode_locations.push(a_2);
                    a_2 = (a_2.0 + y_dist, a_2.1 + x_dist);
                }
            }
        }
    }
    antinode_locations.iter().sorted().dedup().count() as Solution
}

fn main() {
    let parse_output = &mut parse(MAIN_INPUT);
    println!("Part 1: {}", part_1(parse_output));
    println!("Part 2: {}", part_2(parse_output));
}

test_and_bench! {
    TEST_INPUT == "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
    for tests: {
        part_1: { TEST_INPUT => 14 },
        part_2: { TEST_INPUT => 34 },
    },
    bench1 == 228,
    bench2 == 766,
    bench_parse: |(a, _): ParseOutput| a.len() => 36,
}
