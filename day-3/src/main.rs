use aoc_lib::*;

aoc_setup!(Day3, test 1: 198, test 2: 230);

pub struct Day3;

impl AdventOfCode for Day3 {
    type Input = (usize, Vec<usize>);
    type Output = usize;

    fn parse_input(s: &str) -> Self::Input {
        (
            s.lines().next().unwrap().len(),
            s.lines()
                .map(|l| usize::from_str_radix(l, 2).unwrap())
                .collect(),
        )
    }

    fn solve_1(input: &Self::Input) -> Self::Output {
        let mask = (1 << input.0) - 1;
        let half_len = input.1.len() / 2;

        let gamma = (0..input.0)
            .map(|i| (i, get_one_count_at_location(input.1.iter(), i) > half_len))
            .fold(0, |acc, (i, set)| if set { acc | (1 << i) } else { acc });

        let epsilon = (!gamma) & mask;

        gamma * epsilon
    }

    fn solve_2(input: &Self::Input) -> Self::Output {
        let part1 = solve_2_sub(input, false);
        let part2 = solve_2_sub(input, true);

        part1 * part2
    }
}

fn solve_2_sub(input: &(usize, Vec<usize>), reverse: bool) -> usize {
    let full_mask = (1 << input.0) - 1;
    let mut pattern = 0;
    let mut pattern_len = 0;
    let mut pattern_bitmask = 0;

    (0..input.0)
        .rev()
        .find_map(|i| {
            let iterator = input
                .1
                .iter()
                .filter(|&&num| (num & pattern_bitmask) == pattern);

            let (total_count, one_count) = iterator.fold((0, 0), |(total, ones), &n| {
                (total + 1, if (n & (1 << i)) > 0 { ones + 1 } else { ones })
            });
            if (!reverse && (one_count * 2) >= total_count)
                || (reverse && (one_count * 2) < total_count)
            {
                // position has more ones than zeroes
                pattern |= 1 << i;
            }
            pattern_len += 1;

            // see if we find only a single match with this pattern
            // NOTE: can probably use `1 << i` instead of tracking pattern_len
            pattern_bitmask = full_mask & !((1 << (input.0 - pattern_len)) - 1);

            // NOTE: ideally, I would only iterate over `input` once per `i`,
            let mut iterator = input
                .1
                .iter()
                .filter(|&&num| (num & pattern_bitmask) == pattern);

            match (iterator.next(), iterator.next()) {
                (Some(x), None) => Some(*x),
                _ => None,
            }
        })
        .unwrap()
}

fn get_one_count_at_location<'a>(items: impl Iterator<Item = &'a usize>, i: usize) -> usize {
    items.filter(|&&n| (n & (1 << i)) > 0).count()
}
