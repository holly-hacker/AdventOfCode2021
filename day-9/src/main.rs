use aoc_lib::*;

aoc_setup!(Day9, sample 1: 15, sample 2: 1134, part 1: 458, part 2: 1391940);

// TODO: could move to AocLib
#[derive(Debug)]
pub struct Field2D<T> {
    data: Vec<T>,
    stride: usize,
}

impl<T> Field2D<T>
where
    T: Copy,
{
    pub fn item_at(&self, x: isize, y: isize) -> Option<T> {
        if x < 0 || y < 0 {
            return None;
        }

        let x = x as usize;
        let y = y as usize;
        let x_max = self.stride - 1;
        let y_max = self.data.len() / self.stride - 1;
        if x > x_max || y > y_max {
            return None;
        }
        Some(self.data[y * self.stride + x])
    }

    pub fn item_at_unsafe(&self, x: usize, y: usize) -> T {
        self.data[y * self.stride + x]
    }

    pub fn neighbours(&self, x: usize, y: usize) -> [Option<T>; 4] {
        let x = x as isize;
        let y = y as isize;
        [
            self.item_at(x - 1, y),
            self.item_at(x + 1, y),
            self.item_at(x, y - 1),
            self.item_at(x, y + 1),
        ]
    }
}

impl Field2D<u8> {
    pub fn flood_fill(&self, target: &mut Field2D<usize>) -> usize {
        let mut idx = 0;
        let mut num = 1;
        loop {
            let idx_data = target.data.get(idx);
            if idx_data.is_none() {
                break;
            }
            if *idx_data.unwrap() != 0 || self.data[idx] == 9 {
                idx += 1;
                continue;
            }

            self.flood_fill_recursive(target, num, idx);
            num += 1;
            idx += 1;
        }

        num
    }

    fn flood_fill_recursive(&self, target: &mut Field2D<usize>, num: usize, idx: usize) -> () {
        if idx >= target.data.len() || target.data[idx] != 0 {
            return;
        }

        if self.data[idx] == 9 {
            return;
        }

        target.data[idx] = num;

        let (x, y) = ((idx % self.stride) as isize, (idx / self.stride) as isize);
        [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
            .into_iter()
            .filter(|&(x, y)| {
                (x < self.stride as isize)
                    && (y < (self.data.len() / self.stride) as isize)
                    && (x >= 0)
                    && (y >= 0)
            })
            .for_each(|(x, y)| {
                self.flood_fill_recursive(target, num, y as usize * self.stride + x as usize);
            });
    }
}

pub struct Day9;

impl AdventOfCode for Day9 {
    type Input = Field2D<u8>;
    type Output = usize;

    fn parse_input(s: &str) -> Self::Input {
        Field2D {
            data: s
                .lines()
                .map(|l| l.chars().map(|c| (c as u8) - b'0'))
                .flatten()
                .collect(),
            stride: s.lines().next().unwrap().len(),
        }
    }

    fn solve_1(input: &Self::Input) -> Self::Output {
        (0..(input.data.len() / input.stride))
            .map(|y| (0..input.stride).map(move |x| (x, y)))
            .flatten()
            .filter_map(|(x, y)| {
                let center = input.item_at_unsafe(x, y);
                let neighbours = input.neighbours(x, y);
                let higher_neighbours = neighbours
                    .iter()
                    .filter(|&&n| match n {
                        Some(n) => n > center,
                        None => true,
                    })
                    .count();
                if higher_neighbours == 4 {
                    Some((center + 1) as usize)
                } else {
                    None
                }
            })
            .sum::<usize>()
    }

    fn solve_2(input: &Self::Input) -> Self::Output {
        let mut basin_field = Field2D::<usize> {
            data: vec![0; input.data.len()],
            stride: input.stride,
        };
        let count = input.flood_fill(&mut basin_field);

        let (a, b, c) = (1..count)
            .map(|i| basin_field.data.iter().filter(|&&x| x == i).count())
            .fold((0, 0, 0), |acc, n| {
                if n < acc.0 {
                    (acc.0, acc.1, acc.2)
                } else {
                    if n < acc.1 {
                        (n, acc.1, acc.2)
                    } else {
                        if n < acc.2 {
                            (acc.1, n, acc.2)
                        } else {
                            (acc.1, acc.2, n)
                        }
                    }
                }
            });

        a * b * c
    }
}
