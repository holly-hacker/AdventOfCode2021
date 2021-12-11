use aoc_lib::*;

aoc_setup!(Day11, sample 1: 1656, sample 2: 195, part 1: 1785, part 2: 354);

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 10;

#[derive(Clone)]
pub struct Input {
    // NOTE: very minor improvement (noise?) to part 1 and 2 if data type is u32
    pub data: [u8; WIDTH * HEIGHT],
}

impl Input {
    pub fn parse(input: &str) -> Self {
        Input {
            data: input
                .lines()
                .enumerate()
                .fold([0; WIDTH * HEIGHT], |mut acc, (y, line)| {
                    for (x, c) in line.chars().enumerate() {
                        acc[x + y * WIDTH] = (c as u8) - b'0';
                    }
                    acc
                }),
        }
    }

    pub fn step(&mut self) -> usize {
        // NOTE: could look into trying to do this in a single pass
        (0..self.data.len()).for_each(|idx| self.data[idx] += 1);
        (0..self.data.len()).for_each(|idx| self.flash_recursive(idx));

        self.data.iter().filter(|&&x| x == 0).count()
    }

    fn flash_recursive(&mut self, idx: usize) {
        if self.data[idx] > 9 {
            self.data[idx] = 0;

            self.neighbour_indices(idx)
                .into_iter()
                .flatten()
                .for_each(|n| {
                    if self.data[n] != 0 {
                        self.data[n] += 1;
                        self.flash_recursive(n);
                    }
                })
        }
    }

    fn neighbour_indices(&self, idx: usize) -> [Option<usize>; 8] {
        let x = idx % WIDTH;
        let space_above = idx >= WIDTH;
        let space_below = idx + WIDTH < self.data.len();
        let space_left = x > 0;
        let space_right = x < WIDTH - 1;
        [
            (space_above && space_left).then(|| idx - WIDTH - 1),
            space_above.then(|| idx - WIDTH),
            (space_above && space_right).then(|| idx - WIDTH + 1),
            space_left.then(|| idx - 1),
            space_right.then(|| idx + 1),
            (space_below && space_left).then(|| idx + WIDTH - 1),
            space_below.then(|| idx + WIDTH),
            (space_below && space_right).then(|| idx + WIDTH + 1),
        ]
    }
}

pub struct Day11;

impl AdventOfCode for Day11 {
    type Input = Input;
    type Output = usize;

    fn parse_input(s: &str) -> Self::Input {
        Self::Input::parse(s)
    }

    fn solve_1(input: &Self::Input) -> Self::Output {
        let mut input = input.clone();
        (0..100).fold(0, |mut acc, _| {
            acc += input.step();
            acc
        })
    }

    fn solve_2(input: &Self::Input) -> Self::Output {
        let mut input = input.clone();
        (0..usize::MAX)
            .filter_map(|i| match input.step() {
                100 => Some(i + 1),
                _ => None,
            })
            .next()
            .unwrap()
    }
}
