use std::ops::Add;

use aoc_lib::*;
use tinyvec::ArrayVec;

aoc_setup!(Day14, sample 1: 1588, sample 2: 2188189693529, part 1: 2967, part 2: 3692219987038);

// TODO: could be stack-based hashmap. may need to make size u32
#[derive(Default, Clone, Debug)]
struct NumSet([u64; 26]);

impl NumSet {
    pub fn increment(&mut self, i: u8) {
        self.0[(i - b'A') as usize] += 1;
    }
}

impl Add for NumSet {
    type Output = NumSet;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl Add for &NumSet {
    type Output = NumSet;

    fn add(self, rhs: Self) -> Self::Output {
        let mut new_set: NumSet = Default::default();
        for (i, s) in self
            .0
            .iter()
            .zip(rhs.0.iter())
            .map(|(a, b)| a + b)
            .enumerate()
        {
            new_set.0[i] = s;
        }
        new_set
    }
}

type InputData = (ArrayVec<[u8; 20]>, Vec<((u8, u8), u8)>);

pub struct Day14;

impl AdventOfCode for Day14 {
    type Input = InputData;
    type Output = usize;

    fn parse_input(s: &str) -> Self::Input {
        let mut lines = s.lines();
        let first_line = lines.next().unwrap();
        let mut template = ArrayVec::new();
        template.extend_from_slice(first_line.as_bytes());
        lines.next();

        let pairs = lines
            .map(|l| {
                let (first, second) = l.split_once(" -> ").unwrap();
                (
                    (first.as_bytes()[0], first.as_bytes()[1]),
                    second.as_bytes()[0],
                )
            })
            .collect();

        (template, pairs)
    }

    fn solve_1(input: &Self::Input) -> Self::Output {
        // let start_pairs = input.0.as_slice().windows(2).map(|w| (w[0], w[1]));

        run(input, 10)
    }

    fn solve_2(input: &Self::Input) -> Self::Output {
        run(input, 40)
    }
}

fn run(input: &InputData, count: usize) -> usize {
    type Iteration = rustc_hash::FxHashMap<(u8, u8), NumSet>; // TODO: also stack-based maybe? would be pretty large

    fn gen_iteration(pairs: &[((u8, u8), u8)], old: &Iteration, iteration: &mut Iteration) {
        for &(from, to) in pairs {
            let (target1, target2) = ((from.0, to), (to, from.1));

            let mut new_set = match (old.get(&target1), old.get(&target2)) {
                (Some(s1), Some(s2)) => s1 + s2,
                (Some(s), None) => s.clone(),
                (None, Some(s)) => s.clone(),
                (None, None) => Default::default(),
            };
            new_set.increment(to);
            iteration.insert(from, new_set);
        }
    }

    let (mut old_iteration, mut new_iteration) = (Iteration::default(), Iteration::default());

    for _ in 0..count {
        std::mem::swap(&mut old_iteration, &mut new_iteration);
        gen_iteration(&input.1, &old_iteration, &mut new_iteration);
    }

    let final_map = new_iteration;

    let mut final_set: NumSet = input
        .0
        .as_slice()
        .windows(2)
        .map(|w| (w[0], w[1]))
        .fold(Default::default(), |acc, t| &acc + &final_map[&t]);

    input.0.iter().for_each(|&c| final_set.increment(c));

    let (min, max) = final_set
        .0
        .iter()
        .filter(|&&x| x != 0)
        .fold((u64::MAX, u64::MIN), |(min, max), &x| {
            (min.min(x), max.max(x))
        });

    (max - min) as usize
}
