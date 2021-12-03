use aoc_lib::*;

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

fn parse_input(s: &str) -> Vec<usize> {
    s.lines().map(|l| l.parse().unwrap()).collect()
}

fn solve_1(input: &[usize]) -> usize {
    input.windows(2).filter(|w| w[0] < w[1]).count()
}

fn solve_2(input: &[usize]) -> usize {
    input.windows(4).filter(|w| w[0] < w[3]).count()
}

#[test]
fn test_solve_1() {
    let input = include_str!("../sample.txt");
    let parsed = parse_input(input);
    assert_eq!(7, solve_1(&parsed));
}

#[test]
fn test_solve_2() {
    let input = include_str!("../sample.txt");
    let parsed = parse_input(input);
    assert_eq!(5, solve_2(&parsed));
}
