pub use paste::paste;
use std::fmt::Display;

pub mod utils;

#[macro_export]
macro_rules! aoc_setup {
    ($type:ident $(, $test_type:ident $index:literal: $test_result:expr)*) => {
        // TODO: pub use another macro that creates benchmarks?

        fn main() {
            aoc_lib::run($type);
        }

        $(
            aoc_setup!(test_impl $test_type $type $index: $test_result);
        )*
    };
    (test_impl sample $type:ident $index:literal: $test_result:expr) => {
        aoc_lib::paste! {
            #[test]
            fn [<solve_sample_part_ $index>]() {
                let input = include_str!("../sample.txt");
                let parsed = $type::parse_input(input);
                assert_eq!($test_result, $type::[< solve_ $index >](&parsed));
            }
        }
    };
    (test_impl part $type:ident $index:literal: $test_result:expr) => {
        aoc_lib::paste! {
            #[test]
            fn [<solve_part_ $index>]() {
                let input = include_str!("../input.txt");
                let parsed = $type::parse_input(input);
                assert_eq!($test_result, $type::[< solve_ $index >](&parsed));
            }
        }
    };
}

pub trait AdventOfCode {
    type Input;
    type Output;

    fn parse_input(s: &str) -> Self::Input;
    fn solve_1(input: &Self::Input) -> Self::Output;
    fn solve_2(input: &Self::Input) -> Self::Output;
}

/// Run and time just part 1 of a. AdventOfCode solution.
pub fn run_part_1<T: AdventOfCode<Output = impl Display>>(_: T) {
    let input = read_stdin();
    let (parsed, parsed_time) = time(|| T::parse_input(&input));
    let (solve_1, solve_1_time) = time(|| T::solve_1(&parsed));

    println!("Solution to part 1: {}", solve_1);

    println!("Parsing took: {:?}", parsed_time);
    println!("Solving part 1 took: {:?}", solve_1_time);
}

pub fn run<T: AdventOfCode<Output = impl Display>>(_: T) {
    let input = read_stdin();
    let (parsed, parsed_time) = time(|| T::parse_input(&input));
    let (solve_1, solve_1_time) = time(|| T::solve_1(&parsed));
    let (solve_2, solve_2_time) = time(|| T::solve_2(&parsed));

    println!("Solution to part 1: {}", solve_1);
    println!("Solution to part 2: {}", solve_2);

    println!("Parsing took: {:?}", parsed_time);
    println!("Solving part 1 took: {:?}", solve_1_time);
    println!("Solving part 2 took: {:?}", solve_2_time);
}

/// Executes some code and records the time it took to run
pub fn time<T, F>(fun: F) -> (T, std::time::Duration)
where
    F: FnOnce() -> T,
{
    let now = std::time::Instant::now();
    let ret = fun();
    let elapsed = now.elapsed();
    (ret, elapsed)
}

/// Reads stdin to a String
pub fn read_stdin() -> String {
    use std::io::Read;
    let mut string = String::new();
    std::io::stdin().lock().read_to_string(&mut string).unwrap();
    string
}

pub fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::Read;
    let mut file = File::open(filename).unwrap();
    let mut string = String::new();
    file.read_to_string(&mut string).unwrap();
    string
}
