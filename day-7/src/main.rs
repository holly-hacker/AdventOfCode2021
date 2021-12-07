use aoc_lib::*;

aoc_setup!(Day7, sample 1: 37, sample 2: 168, part 1: 355150, part 2: 98368490);

pub struct Day7;

impl AdventOfCode for Day7 {
    type Input = Vec<usize>;
    type Output = usize;

    fn parse_input(s: &str) -> Self::Input {
        s.split(',').map(|l| l.parse().unwrap()).collect()
    }

    fn solve_1(input: &Self::Input) -> Self::Output {
        // could join into 1 loop I suppose
        let min = *input.iter().min().unwrap();
        let max = *input.iter().max().unwrap();

        let mut lowest_cost = usize::MAX;
        for i in min..=max {
            let cost = input
                .iter()
                .map(|&x| (x as isize - i as isize).abs() as usize)
                .sum();
            lowest_cost = lowest_cost.min(cost);
        }

        lowest_cost
    }

    fn solve_2(input: &Self::Input) -> Self::Output {
        // could join into 1 loop I suppose
        let min = *input.iter().min().unwrap();
        let max = *input.iter().max().unwrap();

        let mut lowest_cost = usize::MAX;
        for i in min..=max {
            let cost = input
                .iter()
                .map(|&x| calculate_fuel((x as isize - i as isize).abs() as usize))
                .sum();
            lowest_cost = lowest_cost.min(cost);
        }

        lowest_cost
    }
}

fn calculate_fuel(input: usize) -> usize {
    (1..=input).sum()
}
