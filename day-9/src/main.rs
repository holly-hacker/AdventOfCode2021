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
    pub fn neighbour_indices(&self, idx: usize) -> [Option<usize>; 4] {
        let x = idx % self.stride;
        [
            if idx >= self.stride {
                Some(idx - self.stride)
            } else {
                None
            },
            if idx + self.stride < self.data.len() {
                Some(idx + self.stride)
            } else {
                None
            },
            if x > 0 { Some(idx - 1) } else { None },
            if x < self.stride - 1 {
                Some(idx + 1)
            } else {
                None
            },
        ]
    }
}

impl Field2D<u8> {
    /// executes a flood fill algorithm and returns the sizes of the largest 3 regions
    pub fn flood_fill(&self, target: &mut Field2D<usize>) -> (usize, usize, usize) {
        let mut idx = 0;
        let mut num = 1;
        let mut regions = (0, 0, 0);
        loop {
            let idx_data = target.data.get(idx);
            if idx_data.is_none() {
                break;
            }
            if *idx_data.unwrap() != 0 || self.data[idx] == 9 {
                idx += 1;
                continue;
            }

            let size = self.flood_fill_recursive(target, num, idx, 0);

            if size < regions.0 {
                regions = (regions.0, regions.1, regions.2);
            } else if size < regions.1 {
                regions = (size, regions.1, regions.2);
            } else if size < regions.2 {
                regions = (regions.1, size, regions.2);
            } else {
                regions = (regions.1, regions.2, size);
            }

            num += 1;
            idx += 1;
        }

        regions
    }

    /// flood fills a single region and returns its size
    fn flood_fill_recursive(
        &self,
        target: &mut Field2D<usize>,
        num: usize,
        idx: usize,
        mut size: usize,
    ) -> usize {
        if idx >= target.data.len() || target.data[idx] != 0 {
            return size;
        }

        if self.data[idx] == 9 {
            return size;
        }

        target.data[idx] = num;
        size += 1;

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
                size = self.flood_fill_recursive(
                    target,
                    num,
                    y as usize * self.stride + x as usize,
                    size,
                )
            });
        size
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
        (0..input.data.len())
            .filter_map(|idx| {
                let center = input.data[idx];
                let higher_neighbours = input
                    .neighbour_indices(idx)
                    .iter()
                    .filter(|&&n| match n {
                        Some(n) => input.data[n as usize] > center,
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
        let (a, b, c) = input.flood_fill(&mut basin_field);

        a * b * c
    }
}
