use aoc_lib::*;

aoc_setup!(Day2, test 1: 150, test 2: 900);

#[derive(Clone, Copy)]
pub enum Direction {
    Forward,
    Down,
    Up,
}

pub struct Day2;

impl AdventOfCode for Day2 {
    type Input = Vec<(Direction, isize)>;
    type Output = isize;

    fn parse_input(s: &str) -> Self::Input {
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

    fn solve_1(input: &Self::Input) -> Self::Output {
        let sub = input
            .into_iter()
            .fold((0, 0), |sub, &(dir, dist)| match dir {
                Direction::Forward => (sub.0 + dist, sub.1),
                Direction::Down => (sub.0, sub.1 + dist),
                Direction::Up => (sub.0, sub.1 - dist),
            });
        sub.0 * sub.1
    }

    fn solve_2(input: &Self::Input) -> Self::Output {
        let sub = input
            .into_iter()
            .fold((0, 0, 0), |sub, &(dir, dist)| match dir {
                Direction::Forward => (sub.0 + dist, sub.1 + sub.2 * dist, sub.2),
                Direction::Down => (sub.0, sub.1, sub.2 + dist),
                Direction::Up => (sub.0, sub.1, sub.2 - dist),
            });
        sub.0 * sub.1
    }
}
