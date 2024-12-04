#![feature(test)]

use utils::{get_day, get_session, test_and_bench, try_submit};

type Solution = i32;
pub type ParseOutput = Vec<Vec<char>>;
const MAIN_INPUT: &str = include_str!("main_input");

pub fn parse(file: &str) -> ParseOutput {
    file.lines().map(|line| line.chars().collect()).collect()
}

fn part_1(output: &ParseOutput) -> Solution {
    let mut c = 0;
    let width = output[0].len();
    let height = output.len();
    let xmas = ['X', 'M', 'A', 'S'];
    for y in 0..height {
        for x in 0..width {
            c += is_xmas(output, xmas, y, x, [[0, 1, 2, 3], [0, 0, 0, 0]]);
            c += is_xmas(output, xmas, y, x, [[0, -1, -2, -3], [0, 0, 0, 0]]);
            c += is_xmas(output, xmas, y, x, [[0, 0, 0, 0], [0, -1, -2, -3]]);
            c += is_xmas(output, xmas, y, x, [[0, 0, 0, 0], [0, 1, 2, 3]]);
            c += is_xmas(output, xmas, y, x, [[0, 1, 2, 3], [0, -1, -2, -3]]);
            c += is_xmas(output, xmas, y, x, [[0, -1, -2, -3], [0, -1, -2, -3]]);
            c += is_xmas(output, xmas, y, x, [[0, 1, 2, 3], [0, 1, 2, 3]]);
            c += is_xmas(output, xmas, y, x, [[0, -1, -2, -3], [0, 1, 2, 3]]);
        }
    }
    c
}

fn is_xmas(
    output: &ParseOutput,
    xmas: [char; 4],
    y: usize,
    x: usize,
    dir: [[isize; 4]; 2],
) -> Solution {
    (0..4).all(|i| {
        char_equals(
            output,
            x as isize + dir[0][i],
            y as isize + dir[1][i],
            xmas[i],
        )
    }) as Solution
}

fn char_equals(output: &ParseOutput, x: isize, y: isize, c: char) -> bool {
    if x < 0 || y < 0 {
        return false;
    }
    output
        .get(y as usize)
        .and_then(|l| l.get(x as usize))
        .map_or(false, |&v| v == c)
}

fn part_2(output: &ParseOutput) -> Solution {
    let mut c = 0;
    let width = output[0].len();
    let height = output.len();
    for y in 0..height {
        for x in 0..width {
            if !char_equals(output, x as isize, y as isize, 'A') {
                continue;
            }
            if char_equals(output, x as isize - 1, y as isize - 1, 'M')
                && char_equals(output, x as isize + 1, y as isize + 1, 'S')
                && char_equals(output, x as isize + 1, y as isize - 1, 'S')
                && char_equals(output, x as isize - 1, y as isize + 1, 'M')
            {
                c += 1;
            }

            if char_equals(output, x as isize - 1, y as isize - 1, 'M')
                && char_equals(output, x as isize + 1, y as isize + 1, 'S')
                && char_equals(output, x as isize + 1, y as isize - 1, 'M')
                && char_equals(output, x as isize - 1, y as isize + 1, 'S')
            {
                c += 1;
            }

            if char_equals(output, x as isize - 1, y as isize - 1, 'S')
                && char_equals(output, x as isize + 1, y as isize + 1, 'M')
                && char_equals(output, x as isize + 1, y as isize - 1, 'S')
                && char_equals(output, x as isize - 1, y as isize + 1, 'M')
            {
                c += 1;
            }

            if char_equals(output, x as isize - 1, y as isize - 1, 'S')
                && char_equals(output, x as isize + 1, y as isize + 1, 'M')
                && char_equals(output, x as isize + 1, y as isize - 1, 'M')
                && char_equals(output, x as isize - 1, y as isize + 1, 'S')
            {
                c += 1;
            }
        }
    }
    c
}

#[tokio::main]
async fn main() {
    let parse_output = &mut parse(MAIN_INPUT);
    let session = get_session(get_day());
    try_submit(&session, 1, format!("{}", part_1(parse_output))).await;
    try_submit(&session, 2, format!("{}", part_2(parse_output))).await;
}

test_and_bench! {
    TEST_INPUT == "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
    for tests: {
        part_1: { TEST_INPUT => 18 },
        part_2: { TEST_INPUT => 9 },
    },
    bench1 == 2358,
    bench2 == 1737,
    bench_parse: |v: ParseOutput| v.len() => 140,
}
