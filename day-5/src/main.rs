#![allow(clippy::type_complexity)] // TODO: fix

use aoc_lib::*;

aoc_setup!(Day5, sample 1: 5, sample 2: 12, part 1: 4826, part 2: 16793);

pub struct Grid<const MIN_VALUE: u8> {
    grid: Vec<u8>,
    stride: usize,
    min_count: usize,
}

impl<const MIN_VALUE: u8> Grid<{ MIN_VALUE }> {
    pub fn create(input: &[((usize, usize), (usize, usize))]) -> Self {
        let width = input
            .iter()
            .map(|((x1, _), (x2, _))| x1.max(x2))
            .fold(0usize, |acc, &x| acc.max(x + 1));
        let height = input
            .iter()
            .map(|((_, y1), (_, y2))| y1.max(y2))
            .fold(0usize, |acc, &y| acc.max(y + 1));

        Self {
            grid: vec![0u8; width * height],
            stride: width,
            min_count: 0,
        }
    }

    pub fn increment(&mut self, x: usize, y: usize) {
        let ptr = self.grid.get_mut(x + y * self.stride).unwrap();

        *ptr += 1;

        if *ptr == MIN_VALUE {
            self.min_count += 1;
        }
    }
}

pub struct Day5;

impl AdventOfCode for Day5 {
    type Input = Vec<((usize, usize), (usize, usize))>;
    type Output = usize;

    fn parse_input(s: &str) -> Self::Input {
        s.lines()
            .map(|l| {
                let (s1, s2) = l.split_once(" -> ").unwrap();
                let ((s11, s12), (s21, s22)) =
                    (s1.split_once(',').unwrap(), s2.split_once(',').unwrap());
                (
                    (s11.parse().unwrap(), s12.parse().unwrap()),
                    (s21.parse().unwrap(), s22.parse().unwrap()),
                )
            })
            .collect()
    }

    fn solve_1(input: &Self::Input) -> Self::Output {
        let mut grid = Grid::<2>::create(input);

        for &((x1, y1), (x2, y2)) in input {
            if x1 == x2 {
                for y in y1.min(y2)..=y1.max(y2) {
                    grid.increment(x1, y);
                }
            } else if y1 == y2 {
                for x in x1.min(x2)..=x1.max(x2) {
                    grid.increment(x, y1);
                }
            }
        }

        grid.min_count
    }

    fn solve_2(input: &Self::Input) -> Self::Output {
        let mut grid = Grid::<2>::create(input);

        for &((x1, y1), (x2, y2)) in input {
            if x1 == x2 {
                for y in y1.min(y2)..=y1.max(y2) {
                    grid.increment(x1, y);
                }
            } else if y1 == y2 {
                for x in x1.min(x2)..=x1.max(x2) {
                    grid.increment(x, y1);
                }
            } else if (x1 as isize - x2 as isize).abs() == (y1 as isize - y2 as isize).abs() {
                let mut x = x1;
                let mut y = y1;

                grid.increment(x, y);

                while x != x2 || y != y2 {
                    if x1 < x2 {
                        x += 1;
                    } else {
                        x -= 1;
                    }
                    if y1 < y2 {
                        y += 1;
                    } else {
                        y -= 1;
                    }

                    grid.increment(x, y);
                }
            }
        }

        // println!("Grid created in {:?}", grid_time);

        grid.min_count
    }
}
