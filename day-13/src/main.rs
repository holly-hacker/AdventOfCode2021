use aoc_lib::utils::*;
use aoc_lib::*;
use tinyvec::{array_vec, ArrayVec};

// TODO: part 2: FJAHJGAH
aoc_setup!(Day13, sample 1: 17, sample 2: 16, part 1: 785, part 2: 98);

#[derive(Clone, Copy)]
pub enum Fold {
    AlongX(usize),
    AlongY(usize),
}

impl Default for Fold {
    fn default() -> Self {
        Fold::AlongX(0)
    }
}

pub struct FoldableField2D<T> {
    pub field: Field2D<T>,
    pub width: usize,
    pub height: usize,
}

pub struct Day13;

impl AdventOfCode for Day13 {
    type Input = (Vec<(usize, usize)>, ArrayVec<[Fold; 12]>);
    type Output = usize;

    fn parse_input(s: &str) -> Self::Input {
        let mut points = vec![];
        let mut folds = array_vec![];
        s.lines().fold(false, |acc, line| {
            if line.is_empty() {
                true
            } else if !acc {
                // part 1
                let split = line.split_once(',').unwrap();
                points.push((split.0.parse().unwrap(), split.1.parse().unwrap()));
                false
            } else {
                // part 2
                let important_part = line.rsplit_once(' ').unwrap().1;
                match important_part.split_once('=').unwrap() {
                    ("x", x) => folds.push(Fold::AlongX(x.parse().unwrap())),
                    ("y", y) => folds.push(Fold::AlongY(y.parse().unwrap())),
                    x => panic!("unexpected fold {}", x.0),
                }
                true
            }
        });

        (points, folds)
    }

    fn solve_1(input: &Self::Input) -> Self::Output {
        let fold = input.1[0];
        let mut hashmap = rustc_hash::FxHashSet::default();
        hashmap.reserve(input.0.len());
        match fold {
            Fold::AlongX(fold_idx) => input.0.iter().for_each(|&(x, y)| {
                hashmap.insert((if x > fold_idx { fold_idx * 2 - x } else { x }, y));
            }),
            Fold::AlongY(fold_idx) => input.0.iter().for_each(|&(x, y)| {
                hashmap.insert((x, if y > fold_idx { fold_idx * 2 - y } else { y }));
            }),
        };

        hashmap.len()
    }

    fn solve_2(input: &Self::Input) -> Self::Output {
        let (width, height) = input
            .0
            .iter()
            .fold((0, 0), |(w, h), &(x, y)| (w.max(x + 1), h.max(y + 1)));

        let (width, height) = input
            .1
            .iter()
            .fold((width, height), |(w, h), &fold| match fold {
                Fold::AlongX(fold_idx) => (fold_idx, h),
                Fold::AlongY(fold_idx) => (w, fold_idx),
            });

        let mut field = Field2D::new(width, height, false);

        input
            .0
            .iter()
            .map(|&(x, y)| {
                input.1.iter().fold((x, y), |(x, y), &fold| match fold {
                    Fold::AlongX(fold_idx) => (if x > fold_idx { fold_idx * 2 - x } else { x }, y),
                    Fold::AlongY(fold_idx) => (x, if y > fold_idx { fold_idx * 2 - y } else { y }),
                })
            })
            .for_each(|(x, y)| field[(x, y)] = true);

        for y in 0..height {
            for x in 0..width {
                print!("{}", if field.get(x, y) { '#' } else { '.' });
            }
            println!();
        }

        field.data.iter().filter(|&&aa| aa).count()
    }
}
