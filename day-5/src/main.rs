use aoc_lib::*;

aoc_setup!(Day5, sample 1: 5, sample 2: 12, part 1: 4826, part 2: 16793);

pub struct Grid {
    grid: Vec<u8>,
    stride: usize,
}

impl Grid {
    pub fn create(input: &[((usize, usize), (usize, usize))]) -> Self {
        let width = input
            .iter()
            .map(|((x1, _), (x2, _))| x1.max(x2))
            .fold(0usize, |acc, &x| acc.max(x + 1));
        let height = input
            .iter()
            .map(|((_, y1), (_, y2))| y1.max(y2))
            .fold(0usize, |acc, &y| acc.max(y + 1));

        let grid = vec![0u8; width * height];

        Self {
            grid,
            stride: width,
        }
    }

    pub fn increment(&mut self, x: usize, y: usize) {
        self.grid[(x + y * self.stride)] += 1;
    }

    pub fn count_with_min_value(&self, min_value: u8) -> usize {
        self.grid.iter().filter(|&&v| v >= min_value).count()
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
        let mut grid = Grid::create(input);

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

        grid.count_with_min_value(2)
    }

    fn solve_2(input: &Self::Input) -> Self::Output {
        let mut grid = Grid::create(input);

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

        grid.count_with_min_value(2)
    }
}
