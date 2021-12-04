use aoc_lib::*;

aoc_setup!(Day1, test 1: 7, test 2: 5);

pub struct Day1;

impl AdventOfCode for Day1 {
    type Input = Vec<usize>;
    type Output = usize;

    fn parse_input(s: &str) -> Self::Input {
        s.lines().map(|l| l.parse().unwrap()).collect()
    }

    fn solve_1(input: &Self::Input) -> Self::Output {
        input.windows(2).filter(|w| w[0] < w[1]).count()
    }

    fn solve_2(input: &Self::Input) -> Self::Output {
        input.windows(4).filter(|w| w[0] < w[3]).count()
    }
}
