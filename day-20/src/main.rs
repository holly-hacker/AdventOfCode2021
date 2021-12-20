use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
};

use aoc_lib::*;

aoc_setup!(Day20, sample 1: 35, sample 2: 3351, part 1: 5489, part 2: 19066);

// TODO: consider other datatypes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector2 {
    x: i16,
    y: i16,
}

impl From<(i16, i16)> for Vector2 {
    fn from(data: (i16, i16)) -> Self {
        Self {
            x: data.0,
            y: data.1,
        }
    }
}

impl Vector2 {
    pub fn move_x(self, x: i16) -> Self {
        Self {
            x: self.x + x as i16,
            y: self.y,
        }
    }

    pub fn move_y(self, y: i16) -> Self {
        Self {
            x: self.x,
            y: self.y + y as i16,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct InfiniteField2D {
    data: HashSet<Vector2>,
    min_x: i16,
    min_y: i16,
    max_x: i16,
    max_y: i16,
    padded_with_ones: bool,
}

impl InfiniteField2D {
    pub fn from_lines<'a>(lines: impl Iterator<Item = &'a str>) -> Self {
        lines
            .enumerate()
            .flat_map(|(y, line)| {
                line.bytes().enumerate().map(move |(x, c)| {
                    (
                        Vector2 {
                            x: x as i16,
                            y: y as i16,
                        },
                        c == b'#',
                    )
                })
            })
            .fold(Self::default(), |mut acc, (pos, bit)| {
                acc.insert(pos, bit);
                acc
            })
    }

    pub fn insert(&mut self, pos: Vector2, bit: bool) -> bool {
        if bit {
            self.min_x = self.min_x.min(pos.x);
            self.min_y = self.min_y.min(pos.y);
            self.max_x = self.max_x.max(pos.x);
            self.max_y = self.max_y.max(pos.y);
            self.data.insert(pos)
        } else {
            self.data.remove(&pos)
        }
    }

    pub fn read_square_at(&self, pos: Vector2) -> i16 {
        self.bit(pos.move_y(-1).move_x(-1)) << 8
            | self.bit(pos.move_y(-1).move_x(0)) << 7
            | self.bit(pos.move_y(-1).move_x(1)) << 6
            | self.bit(pos.move_y(0).move_x(-1)) << 5
            | self.bit(pos.move_y(0).move_x(0)) << 4
            | self.bit(pos.move_y(0).move_x(1)) << 3
            | self.bit(pos.move_y(1).move_x(-1)) << 2
            | self.bit(pos.move_y(1).move_x(0)) << 1
            | self.bit(pos.move_y(1).move_x(1)) << 0
    }

    fn bit(&self, pos: Vector2) -> i16 {
        if (self.is_out_of_range(pos)) && self.padded_with_ones {
            1
        } else {
            self.data.contains(&pos) as i16
        }
    }

    fn is_out_of_range(&self, pos: Vector2) -> bool {
        !(self.min_x..=self.max_x).contains(&pos.x) || !(self.min_y..=self.max_y).contains(&pos.y)
    }
}

impl Display for InfiniteField2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                write!(
                    f,
                    "{}",
                    if self.data.contains(&Vector2 { x, y }) {
                        '#'
                    } else {
                        '.'
                    }
                )?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

pub struct Day20;

impl AdventOfCode for Day20 {
    type Input = (Vec<bool>, InfiniteField2D); // TODO: first arg is exactly 512 characters long
    type Output = usize;

    fn parse_input(s: &str) -> Self::Input {
        let mut lines = s.lines();
        let v = lines
            .next()
            .unwrap()
            .as_bytes()
            .iter()
            .map(|&b| b == b'#')
            .collect();
        lines.next();

        (v, InfiniteField2D::from_lines(lines))
    }

    fn solve_1(input: &Self::Input) -> Self::Output {
        do_stuff(input, 2)
    }

    fn solve_2(input: &Self::Input) -> Self::Output {
        do_stuff(input, 50)
    }
}

fn do_stuff(input: &(Vec<bool>, InfiniteField2D), count: usize) -> usize {
    debug_assert_eq!(input.0.len(), 512);
    let should_pad_with_ones = input.0[0];
    if should_pad_with_ones {
        debug_assert!(!input.0[511]);
    }

    let mut field = input.1.clone();

    println!("{}", field);

    const EXTEND: i16 = 1;
    for _ in 0..count {
        let mut new_field = field.clone();
        for x in (field.min_x - EXTEND)..=(field.max_x + EXTEND) {
            for y in (field.min_y - EXTEND)..=(field.max_y + EXTEND) {
                let index = field.read_square_at((x, y).into());
                let new_bit = input.0[index as usize];
                new_field.insert((x, y).into(), new_bit);
            }
        }

        if should_pad_with_ones {
            new_field.padded_with_ones = !field.padded_with_ones;
        }

        field = new_field;
    }

    debug_assert!(!field.padded_with_ones);
    field.data.len()
}

#[test]
pub fn test_read_index() {
    const INPUT: &str = "#..#.\n#....\n##..#\n..#..\n..###";
    let field = InfiniteField2D::from_lines(INPUT.lines());
    println!("{:?}", field);
    assert_eq!(field.read_square_at(Vector2 { x: 2, y: 2 }), 0b000100010);
}
