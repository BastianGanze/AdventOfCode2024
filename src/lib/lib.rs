use dotenv::dotenv;
use std::env;

pub mod aoc_api;

use aoc_api::Session;

pub fn get_day() -> u8 {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();
    args.get(0)
        .expect("Day must be provided")
        .split("/")
        .last()
        .expect("Path must exist")
        .parse::<u8>()
        .expect("Day must be a number")
}

pub fn get_session(day: u8) -> Session {
    Session::new(env::var("SESSION").expect("SESSION must be set"), 2024, day)
}

pub fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == b {
        return a;
    }
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    a
}

pub fn lcm(a: usize, b: usize) -> usize {
    a * (b / gcd(a, b))
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(48, 18), 6);
    assert_eq!(gcd(54, 24), 6);
    assert_eq!(gcd(7, 13), 1);
    assert_eq!(gcd(12, 0), 12);
    assert_eq!(gcd(0, 12), 12);
}

// Taken from https://git.kageru.moe/kageru/advent-of-code/src/branch/master/2024/src/teststuff.rs
#[macro_export]
macro_rules! test_and_bench {
    (
        $($tin: ident == $ti: literal),+
        for tests: {
            $($part: ident: { $($tpi: expr $(,$ati: expr)* => $to: expr),+$(,)? }),*$(,)?
        },
        $(unittests: {
            $($unittest: ident: { $($($utpi: expr),+ => $uto: expr),+$(,)? }),*$(,)?
        },)?
        bench1$(($bi1: literal))? == $b1: expr,
        bench2$(($bi2: literal))? == $b2: expr,
        bench_parse: $input_fn: expr => $it: expr$(,)?
    ) => {

    #[cfg(test)]
    mod tests {
        extern crate test;
        use super::*;
        use utils::*;

        $(
        const $tin: &str = $ti;
        )+

        $($($(paste::paste! {
            #[test]
            fn [<$unittest _test_ $uto:lower>]() {
                assert_eq!($unittest($($utpi),+), $uto);
            }
        })+)*)?
        $($(paste::paste! {
            #[test]
            fn [<$part _test_ $to:lower>]() {
                let input = parse($tpi);
                assert_eq!($part(&input, $($ati),*), $to);
            }
        })+)*
        bench!(part_1($($bi1)?) == $b1);
        bench!(part_2($($bi2)?) == $b2);
        #[bench]
        fn bench_input_parsing(b: &mut test::Bencher) {
            b.iter(|| assert_eq!($input_fn(parse(test::black_box(&MAIN_INPUT))), $it));
        }
    }
    }
}

#[macro_export]
macro_rules! bench {
    ($part: ident($($bi: literal)?) == $expected:expr) => {
        paste::paste! {
            #[bench]
            fn [<$part _bench>](b: &mut test::Bencher) {
                let input = parse(&MAIN_INPUT);
                b.iter(|| assert_eq!($part(test::black_box(&input)$(, $bi)?), $expected));
            }
        }
    };
}
