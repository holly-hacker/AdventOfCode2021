use aoc_lib::*;

#[derive(Clone, Copy)]
pub enum Direction {
    Forward,
    Down,
    Up,
}

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

pub fn parse_input(s: &str) -> Vec<(Direction, isize)> {
    s.lines()
        .map(|line| {
            let mut split = line.split(' ');
            (
                match split.next().unwrap() {
                    "forward" => Direction::Forward,
                    "down" => Direction::Down,
                    "up" => Direction::Up,
                    _ => unreachable!(),
                },
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

pub fn solve_1(input: &[(Direction, isize)]) -> isize {
    let sub = input
        .into_iter()
        .fold((0, 0), |sub, &(dir, dist)| match dir {
            Direction::Forward => (sub.0 + dist, sub.1),
            Direction::Down => (sub.0, sub.1 + dist),
            Direction::Up => (sub.0, sub.1 - dist),
        });
    sub.0 * sub.1
}

pub fn solve_2(input: &[(Direction, isize)]) -> isize {
    let sub = input
        .into_iter()
        .fold((0, 0, 0), |sub, &(dir, dist)| match dir {
            Direction::Forward => (sub.0 + dist, sub.1 + sub.2 * dist, sub.2),
            Direction::Down => (sub.0, sub.1, sub.2 + dist),
            Direction::Up => (sub.0, sub.1, sub.2 - dist),
        });
    sub.0 * sub.1
}

#[test]
fn test_solve_1() {
    let input = include_str!("../sample.txt");
    let parsed = parse_input(input);
    assert_eq!(150, solve_1(&parsed));
}

#[test]
fn test_solve_2() {
    let input = include_str!("../sample.txt");
    let parsed = parse_input(input);
    assert_eq!(900, solve_2(&parsed));
}
