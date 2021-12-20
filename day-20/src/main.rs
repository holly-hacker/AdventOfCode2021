use std::{
    fmt::{Display, Formatter},
    ops::{Add, Sub},
};

use aoc_lib::{utils::Field2D, *};

aoc_setup!(Day20, sample 1: 35, sample 2: 3351, part 1: 5489, part 2: 19066);

// TODO: consider other datatypes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector2 {
    x: isize,
    y: isize,
}

impl From<(isize, isize)> for Vector2 {
    fn from(data: (isize, isize)) -> Self {
        Self {
            x: data.0,
            y: data.1,
        }
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Vector2 {
    pub fn move_x(self, x: isize) -> Self {
        Self {
            x: self.x + x as isize,
            y: self.y,
        }
    }

    pub fn move_y(self, y: isize) -> Self {
        Self {
            x: self.x,
            y: self.y + y as isize,
        }
    }
}

pub struct Field2DWithMovableOrigin<T> {
    data: Field2D<T>,
    origin: Vector2,
    min_x: isize,
    min_y: isize,
    max_x: isize,
    max_y: isize,
    padded_with_ones: bool,
}

impl Field2DWithMovableOrigin<bool> {
    pub fn with_capacity(width: usize, height: usize) -> Self {
        Self {
            data: Field2D::new(width, height, false),
            origin: Vector2::from((0, 0)),
            min_x: 0,
            min_y: 0,
            max_x: 0,
            max_y: 0,
            padded_with_ones: false,
        }
    }

    pub fn init_with(&mut self, source: &Field2D<bool>) {
        self.max_x = self.max_x.max((source.width() - 1) as isize);
        self.max_y = self.max_y.max((source.height() - 1) as isize);

        debug_assert!(self.data.width() >= source.width());
        debug_assert!(self.data.height() >= source.height());

        // copy over data
        for y in 0..source.height() {
            let source_stride = source.stride_at(y as usize);
            let stride = &mut self.data.stride_at_mut(self.origin.y as usize + y)
                [(self.origin.x as usize)..(self.origin.x as usize + source.width())];
            stride.copy_from_slice(source_stride);
        }
    }
}

impl Field2DWithMovableOrigin<bool> {
    pub fn get(&self, pos: Vector2) -> bool {
        let real_position = pos + self.origin;
        self.data[(real_position.x as usize, real_position.y as usize)]
    }

    pub fn insert(&mut self, pos: Vector2, bit: bool) {
        if bit {
            self.min_x = self.min_x.min(pos.x);
            self.min_y = self.min_y.min(pos.y);
            self.max_x = self.max_x.max(pos.x);
            self.max_y = self.max_y.max(pos.y);
        }

        let real_position = pos + self.origin;
        self.data[(real_position.x as usize, real_position.y as usize)] = bit;
    }

    pub fn read_square_at(&self, pos: Vector2) -> isize {
        self.bit(pos.move_y(-1).move_x(-1)) << 8
            | self.bit(pos.move_y(-1).move_x(0)) << 7
            | self.bit(pos.move_y(-1).move_x(1)) << 6
            | self.bit(pos.move_y(0).move_x(-1)) << 5
            | self.bit(pos.move_y(0).move_x(0)) << 4
            | self.bit(pos.move_y(0).move_x(1)) << 3
            | self.bit(pos.move_y(1).move_x(-1)) << 2
            | self.bit(pos.move_y(1).move_x(0)) << 1
            | self.bit(pos.move_y(1).move_x(1))
    }

    fn bit(&self, pos: Vector2) -> isize {
        if self.is_out_of_range(pos) {
            self.padded_with_ones as isize
        } else {
            self.get(pos) as isize
        }
    }

    fn is_out_of_range(&self, pos: Vector2) -> bool {
        !(self.min_x..=self.max_x).contains(&pos.x) || !(self.min_y..=self.max_y).contains(&pos.y)
    }

    pub fn count_ones(&self) -> usize {
        self.data.data.iter().cloned().filter(|&x| x).count()
    }
}

impl Display for Field2DWithMovableOrigin<bool> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                write!(f, "{}", if self.get(Vector2 { x, y }) { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub struct Day20;

impl AdventOfCode for Day20 {
    type Input = (Vec<bool>, Field2D<bool>); // TODO: first arg is exactly 512 characters long
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

        let field = Field2D {
            stride: lines.clone().next().unwrap().len(),
            data: lines
                .map(|l| l.bytes().map(|c| c == b'#'))
                .flatten()
                .collect(),
        };

        (v, field)
    }

    fn solve_1(input: &Self::Input) -> Self::Output {
        do_stuff(input, 2)
    }

    fn solve_2(input: &Self::Input) -> Self::Output {
        do_stuff(input, 50)
    }
}

fn do_stuff(input: &(Vec<bool>, Field2D<bool>), count: usize) -> usize {
    debug_assert_eq!(input.0.len(), 512);
    let should_pad_with_ones = input.0[0];
    if should_pad_with_ones {
        debug_assert!(!input.0[511]);
    }

    const EXTEND: isize = 1;
    let extra_space_needed = (count * EXTEND as usize * 2) as usize;

    let (mut front_buffer, mut back_buffer) = (
        Field2DWithMovableOrigin::with_capacity(
            input.1.width() + extra_space_needed,
            input.1.height() + extra_space_needed,
        ),
        Field2DWithMovableOrigin::with_capacity(
            input.1.width() + extra_space_needed,
            input.1.height() + extra_space_needed,
        ),
    );

    front_buffer.origin = Vector2 {
        x: (count as isize) * EXTEND as isize,
        y: (count as isize) * EXTEND as isize,
    };
    back_buffer.origin = front_buffer.origin;

    front_buffer.init_with(&input.1);

    for _ in 0..count {
        for x in (front_buffer.min_x - EXTEND)..=(front_buffer.max_x + EXTEND) {
            for y in (front_buffer.min_y - EXTEND)..=(front_buffer.max_y + EXTEND) {
                let index = front_buffer.read_square_at((x, y).into());
                let new_bit = input.0[index as usize];
                back_buffer.insert((x, y).into(), new_bit);
            }
        }

        if should_pad_with_ones {
            back_buffer.padded_with_ones = !front_buffer.padded_with_ones;
        }

        // swap
        std::mem::swap(&mut front_buffer, &mut back_buffer);
    }

    debug_assert!(!front_buffer.padded_with_ones);
    front_buffer.count_ones()
}

#[test]
pub fn test_read_index() {
    const INPUT: &str = "#..#.\n#....\n##..#\n..#..\n..###";
    let lines = INPUT.lines();
    let field = Field2D {
        stride: lines.clone().next().unwrap().len(),
        data: lines
            .map(|l| l.bytes().map(|c| c == b'#'))
            .flatten()
            .collect(),
    };
    let mut field2 = Field2DWithMovableOrigin::with_capacity(field.width(), field.height());
    field2.init_with(&field);
    println!("{}", field2);
    assert_eq!(field2.read_square_at(Vector2 { x: 2, y: 2 }), 0b000100010);
}
