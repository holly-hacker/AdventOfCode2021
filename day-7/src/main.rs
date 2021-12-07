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
        // TODO: may need 2, similar to part 2?
        let median = {
            // TODO: there has to be a faster way to get the median!
            let mut sorted = input.clone();
            sorted.sort();
            sorted[sorted.len() / 2]
        };

        input
            .iter()
            .map(|&x| ((x as isize - median as isize).abs() as usize))
            .sum()
    }

    fn solve_2(input: &Self::Input) -> Self::Output {
        // take the floor and ceil of the average, and try both
        let avg1 = input.iter().sum::<usize>() / input.len();
        let avg2 = avg1 + 1;

        let solutions: (usize, usize) = input.iter().fold((0usize, 0usize), |acc, &x| {
            (
                acc.0 + (calculate_fuel((x as isize - avg1 as isize).abs() as usize)),
                acc.1 + (calculate_fuel((x as isize - avg2 as isize).abs() as usize)),
            )
        });

        usize::min(solutions.0, solutions.1)
    }
}

// calculates the triangular number
const fn calculate_fuel(input: usize) -> usize {
    input * (input + 1) / 2
}
