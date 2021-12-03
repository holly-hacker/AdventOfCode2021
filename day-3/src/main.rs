use aoc_lib::*;

struct Input(usize, Vec<usize>);

fn main() {
    let input = read_stdin();
    let (parsed, parsed_time) = time(|| parse_input(&input));
    let (solve_1, solve_1_time) = time(|| solve_1(&parsed));
    let (solve_2, solve_2_time) = time(|| solve_2(&parsed));

    println!("Solution to part 1: {}", solve_1);
    println!("Solution to part 2: {}", solve_2);

    println!("Parsing took: {:?}", parsed_time);
    println!("Solving part 1 took: {:?}", solve_1_time);
    println!("Solving part 2 took: {:?}", solve_2_time);
}

fn parse_input(s: &str) -> Input {
    Input(
        s.lines().next().unwrap().len(),
        s.lines()
            .map(|l| usize::from_str_radix(l, 2).unwrap())
            .collect(),
    )
}

fn solve_1(input: &Input) -> usize {
    let mask = (1 << input.0) - 1;
    let half_len = input.1.len() / 2;

    let gamma = (0..input.0)
        .map(|i| (i, get_one_count_at_location(input.1.iter(), i) > half_len))
        .fold(0, |acc, (i, set)| if set { acc | (1 << i) } else { acc });

    let epsilon = (!gamma) & mask;

    gamma * epsilon
}

fn solve_2(input: &Input) -> usize {
    let part1 = solve_2_sub(input, false);
    let part2 = solve_2_sub(input, true);

    part1 * part2
}

fn solve_2_sub(input: &Input, reverse: bool) -> usize {
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
            let one_count = get_one_count_at_location(iterator, i);
            let total_count = input
                .1
                .iter()
                .filter(|&&num| (num & pattern_bitmask) == pattern)
                .count();
            if (!reverse && (one_count * 2) >= total_count)
                || (reverse && (one_count * 2) < total_count)
            {
                // position has more ones than zeroes
                pattern |= 1 << i;
            }
            pattern_len += 1;

            // see if we find only a single match with this pattern
            pattern_bitmask = full_mask & !((1 << (input.0 - pattern_len)) - 1);
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

// TODO: generic over iterator?
fn get_one_count_at_location<'a>(items: impl Iterator<Item = &'a usize>, i: usize) -> usize {
    items.filter(|&&n| (n & (1 << i)) > 0).count()
}

#[test]
fn test_solve_1() {
    let input = include_str!("../sample.txt");
    let parsed = parse_input(input);
    assert_eq!(198, solve_1(&parsed));
}

#[test]
fn test_solve_2() {
    let input = include_str!("../sample.txt");
    let parsed = parse_input(input);
    assert_eq!(230, solve_2(&parsed));
}
