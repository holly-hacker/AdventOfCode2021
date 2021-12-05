use aoc_lib::*;

aoc_setup!(Day5, sample 1: 5, sample 2: 12, part 1: 4826, part 2: 16793);

pub struct Day5;

impl AdventOfCode for Day5 {
    type Input = Vec<((u16, u16), (u16, u16))>;
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
        let max_x = input
            .iter()
            .map(|((x1, _), (x2, _))| x1.max(x2))
            .fold(0u16, |acc, &x| acc.max(x + 1));
        let max_y = input
            .iter()
            .map(|((_, y1), (_, y2))| y1.max(y2))
            .fold(0u16, |acc, &y| acc.max(y + 1));

        let mut grid = vec![0u8; max_x as usize * max_y as usize];

        for &((x1, y1), (x2, y2)) in input {
            // grid[(x as usize + y as usize * max_x as usize)] += 1;
            if x1 == x2 {
                for y in y1.min(y2)..=y1.max(y2) {
                    grid[(x1 as usize + y as usize * max_x as usize)] += 1;
                }
            } else if y1 == y2 {
                for x in x1.min(x2)..=x1.max(x2) {
                    grid[(x as usize + y1 as usize * max_x as usize)] += 1;
                }
            }
        }

        grid.iter().filter(|&&x| x >= 2).count()
    }

    fn solve_2(input: &Self::Input) -> Self::Output {
        let max_x = input
            .iter()
            .map(|((x1, _), (x2, _))| x1.max(x2))
            .fold(0u16, |acc, &x| acc.max(x + 1));
        let max_y = input
            .iter()
            .map(|((_, y1), (_, y2))| y1.max(y2))
            .fold(0u16, |acc, &y| acc.max(y + 1));

        let mut grid = vec![0u8; max_x as usize * max_y as usize];

        for &((x1, y1), (x2, y2)) in input {
            // grid[(x as usize + y as usize * max_x as usize)] += 1;
            if x1 == x2 {
                for y in y1.min(y2)..=y1.max(y2) {
                    grid[(x1 as usize + y as usize * max_x as usize)] += 1;
                }
            } else if y1 == y2 {
                for x in x1.min(x2)..=x1.max(x2) {
                    grid[(x as usize + y1 as usize * max_x as usize)] += 1;
                }
            } else if (x1 as i16 - x2 as i16).abs() == (y1 as i16 - y2 as i16).abs() {
                let mut x = x1;
                let mut y = y1;

                loop {
                    grid[(x as usize + y as usize * max_x as usize)] += 1;

                    if x == x2 && y == y2 {
                        break;
                    }

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
                }
            }
        }

        grid.iter().filter(|&&x| x >= 2).count()
    }
}
