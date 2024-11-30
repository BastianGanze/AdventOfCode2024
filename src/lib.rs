use aoc_api::Session;
use dotenv::dotenv;
use std::env;

pub fn get_session() -> Session {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();
    let day = args
        .get(0)
        .expect("Day must be provided")
        .split("/")
        .last()
        .expect("Path must exist")
        .parse::<u8>()
        .expect("Day must be a number");
    Session::new(env::var("SESSION").expect("SESSION must be set"), 2023, day)
}

pub async fn try_submit(session: &Session, part: u8, solution: String) {
    println!("Solution to part {} is {}", part, solution);
    let args: Vec<String> = env::args().collect();
    if args.get(1).is_none() || args.get(2).is_none() {
        return;
    }
    assert_eq!(args.get(1).unwrap(), "submit");
    let part_input = args
        .get(2)
        .expect("part must be provided")
        .parse::<u8>()
        .expect("Day must be a number");
    if (part_input != part) {
        return;
    }
    match session.submit_anwer(part, &solution).await {
        Ok(r) => {
            match r.cooldown {
                Some(c) => {
                    println!("Cooldown: {}", c);
                    return;
                }
                None => {}
            }
            match r.success {
                Some(s) => {
                    if s {
                        println!("This is it!");
                    } else {
                        println!("That is not it!");
                    }
                }
                None => {}
            }
        }
        Err(e) => eprintln!("Failed to submit: {}", e),
    }
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
            b.iter(|| assert_eq!($input_fn(&parse(test::black_box(&MAIN_INPUT))), $it));
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
