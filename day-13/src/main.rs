use aoc_lib::{utils::Field2D, *};
use either::Either;
use tinyvec::{array_vec, ArrayVec};

// sample 2: either::Right("O".into())?
aoc_setup!(Day13, sample 1: either::Left(17), part 1: either::Left(785), part 2: either::Right("FJAHJGAH".into()));

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

pub struct Day13;

impl AdventOfCode for Day13 {
    type Input = (Vec<(usize, usize)>, ArrayVec<[Fold; 12]>);
    type Output = Either<usize, String>;

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

        Either::Left(hashmap.len())
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

        Either::Right(
            (0..(field.stride / 5))
                .map(|i| (b'A' + ocr(&field, i)) as char)
                .collect::<String>(),
        )
    }
}

fn ocr(field: &Field2D<bool>, index: usize) -> u8 {
    let start_x = 5 * index;

    LETTER_MAP
        .iter()
        .enumerate()
        .filter_map(|(i, &l)| l.map(|letter| (i, letter)))
        .find(|&(_, letter)| {
            (0..6).all(|y| {
                let idx = start_x + y * field.stride;
                let slice: [bool; 4] = field.data[idx..idx + 4].try_into().unwrap();

                slice == letter[y]
            })
        })
        .unwrap()
        .0 as u8
}

const fn parse_letter(data: [u8; 6]) -> Option<Letter> {
    Some([
        to_bool_map(data[0]),
        to_bool_map(data[1]),
        to_bool_map(data[2]),
        to_bool_map(data[3]),
        to_bool_map(data[4]),
        to_bool_map(data[5]),
    ])
}

const fn to_bool_map(num: u8) -> [bool; 4] {
    [
        (num & 0b1000) != 0,
        (num & 0b0100) != 0,
        (num & 0b0010) != 0,
        (num & 0b0001) != 0,
    ]
}

type Letter = [[bool; 4]; 6];

const LETTER_MAP: [Option<Letter>; 26] = [
    parse_letter([0b0110, 0b1001, 0b1001, 0b1111, 0b1001, 0b1001]),
    parse_letter([0b1110, 0b1001, 0b1110, 0b1001, 0b1001, 0b1110]),
    parse_letter([0b0110, 0b1001, 0b1000, 0b1000, 0b1001, 0b0110]),
    None,
    parse_letter([0b1111, 0b1000, 0b1110, 0b1000, 0b1000, 0b1111]),
    parse_letter([0b1111, 0b1000, 0b1110, 0b1000, 0b1000, 0b1000]),
    parse_letter([0b0110, 0b1001, 0b1000, 0b1011, 0b1001, 0b0111]),
    parse_letter([0b1001, 0b1001, 0b1111, 0b1001, 0b1001, 0b1001]),
    None,
    parse_letter([0b0011, 0b0001, 0b0001, 0b0001, 0b1001, 0b0110]),
    parse_letter([0b1001, 0b1010, 0b1100, 0b1010, 0b1010, 0b1001]),
    None,
    None,
    None,
    None,
    parse_letter([0b1110, 0b1001, 0b1001, 0b1110, 0b1000, 0b1000]),
    None,
    parse_letter([0b1110, 0b1001, 0b1001, 0b1110, 0b1010, 0b1001]),
    None,
    None,
    parse_letter([0b1001, 0b1001, 0b1001, 0b1001, 0b1001, 0b0110]),
    None,
    None,
    None,
    None,
    parse_letter([0b1111, 0b0001, 0b0010, 0b0100, 0b1000, 0b1111]),
];
