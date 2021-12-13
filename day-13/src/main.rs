use aoc_lib::utils::*;
use aoc_lib::*;

// TODO: part 2: FJAHJGAH
aoc_setup!(Day13, sample 1: 17, sample 2: 16, part 1: 785, part 2: 98);

pub enum Fold {
    AlongX(usize),
    AlongY(usize),
}

pub struct FoldableField2D<T> {
    pub field: Field2D<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> FoldableField2D<T>
where
    T: Copy,
{
    pub fn new(width: usize, height: usize, value: T) -> Self {
        let field = Field2D::new(width, height, value);
        Self {
            field,
            width,
            height,
        }
    }
}

impl FoldableField2D<bool> {
    pub fn fold_horizontal(&mut self, fold_idx: usize) {
        ((fold_idx + 1)..(self.height.min(fold_idx * 2 + 1))).for_each(|y| {
            let target = fold_idx * 2 - y;
            (0..self.width).for_each(|x| {
                if self.field[(x, y)] {
                    self.field[(x, target)] = true;
                }
            });
        });
        self.height = fold_idx;
    }

    pub fn fold_vertical(&mut self, fold_idx: usize) {
        ((fold_idx + 1)..(self.width.min(fold_idx * 2 + 1))).for_each(|x| {
            let target = fold_idx * 2 - x;
            (0..self.height).for_each(|y| {
                if self.field[(x, y)] {
                    // this seems to panic
                    self.field[(target, y)] = true;
                }
            });
        });
        self.width = fold_idx;
    }

    pub fn count(&self) -> usize {
        (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| (x, y)))
            .filter(|&(x, y)| self.field.get(x, y))
            .count()
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", if self.field.get(x, y) { '#' } else { '.' });
            }
            println!();
        }
    }
}

pub struct Day13;

impl AdventOfCode for Day13 {
    type Input = (Vec<(usize, usize)>, Vec<Fold>);
    type Output = usize;

    fn parse_input(s: &str) -> Self::Input {
        let mut points = vec![];
        let mut folds = vec![];
        s.lines().fold(false, |acc, line| {
            if line.len() == 0 {
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
        // TODO: for day 1, I don't need to do any mutations. I could make a better algorithm!

        // 600ns
        let (width, height) = input
            .0
            .iter()
            .fold((0, 0), |(w, h), &(x, y)| (w.max(x + 1), h.max(y + 1)));

        // 1.5us
        let mut field = FoldableField2D::new(width, height, false);

        // 200us
        input
            .0
            .iter()
            .for_each(|&(x, y)| field.field[(x, y)] = true);

        // 583us
        match input.1[0] {
            Fold::AlongX(x) => field.fold_vertical(x),
            Fold::AlongY(y) => field.fold_horizontal(y),
        };

        // 262us
        field.count()
    }

    fn solve_2(input: &Self::Input) -> Self::Output {
        let (width, height) = input
            .0
            .iter()
            .fold((0, 0), |(w, h), &(x, y)| (w.max(x + 1), h.max(y + 1)));
        let mut field = FoldableField2D::new(width, height, false);

        input
            .0
            .iter()
            .for_each(|&(x, y)| field.field[(x, y)] = true);

        input.1.iter().for_each(|fold| match fold {
            Fold::AlongX(x) => field.fold_vertical(*x),
            Fold::AlongY(y) => field.fold_horizontal(*y),
        });

        // field.print();

        field.count()
    }
}
