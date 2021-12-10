use aoc_lib::*;

aoc_setup!(Day10, sample 1: 26397, sample 2: 288957, part 1: 344193, part 2: 3241238967);

pub struct Day10;

impl AdventOfCode for Day10 {
    type Input = String;
    type Output = usize;

    fn parse_input(s: &str) -> Self::Input {
        String::from(s)
    }

    fn solve_1(input: &Self::Input) -> Self::Output {
        input
            .lines()
            .map(|l| match parse(l) {
                b')' => 3,
                b']' => 57,
                b'}' => 1197,
                b'>' => 25137,
                0 => 0,
                _ => unreachable!(),
            })
            .sum()
    }

    fn solve_2(input: &Self::Input) -> Self::Output {
        let mut scores = input
            .lines()
            .filter_map(|line| parse_recursive_counting_closing(line.as_bytes(), 0).map(|x| x.2))
            .collect::<Vec<usize>>();

        scores.sort_unstable();

        scores[scores.len() / 2]
    }
}

fn parse(line: &str) -> u8 {
    if line.is_empty() {
        return 0;
    }
    let bytes = line.as_bytes();

    match parse_recursive_until_closing(bytes) {
        Ok(_) => 0,
        Err(e) => e,
    }
}

fn parse_recursive_until_closing(bytes: &[u8]) -> Result<(u8, &[u8]), u8> {
    if bytes.is_empty() {
        return Ok((0, &[]));
    }
    let first = bytes[0];
    let rest = &bytes[1..];

    match first {
        b'(' => {
            let (x, rest) = parse_recursive_until_closing(rest)?;
            if x != b')' {
                Err(x)
            } else {
                parse_recursive_until_closing(rest)
            }
        }
        b'[' => {
            let (x, rest) = parse_recursive_until_closing(rest)?;
            if x != b']' {
                Err(x)
            } else {
                parse_recursive_until_closing(rest)
            }
        }
        b'{' => {
            let (x, rest) = parse_recursive_until_closing(rest)?;
            if x != b'}' {
                Err(x)
            } else {
                parse_recursive_until_closing(rest)
            }
        }
        b'<' => {
            let (x, rest) = parse_recursive_until_closing(rest)?;
            if x != b'>' {
                Err(x)
            } else {
                parse_recursive_until_closing(rest)
            }
        }
        _ => Ok((first, rest)),
    }
}

fn parse_recursive_counting_closing(bytes: &[u8], points: usize) -> Option<(u8, &[u8], usize)> {
    if bytes.is_empty() {
        return Some((0, &[], points));
    }
    let first = bytes[0];
    let rest = &bytes[1..];

    match first {
        b'(' => {
            let (x, rest, points) = parse_recursive_counting_closing(rest, points)?;
            if x != b')' {
                let points = points * 5 + 1;
                if x != 0 {
                    None
                } else if rest.is_empty() {
                    Some((0, &[], points))
                } else {
                    Some((rest[0], &rest[1..], points))
                }
            } else {
                parse_recursive_counting_closing(rest, points)
            }
        }
        b'[' => {
            let (x, rest, points) = parse_recursive_counting_closing(rest, points)?;
            if x != b']' {
                let points = points * 5 + 2;
                if x != 0 {
                    None
                } else if rest.is_empty() {
                    Some((0, &[], points))
                } else {
                    Some((rest[0], &rest[1..], points))
                }
            } else {
                parse_recursive_counting_closing(rest, points)
            }
        }
        b'{' => {
            let (x, rest, points) = parse_recursive_counting_closing(rest, points)?;
            if x != b'}' {
                let points = points * 5 + 3;
                if x != 0 {
                    None
                } else if rest.is_empty() {
                    Some((0, &[], points))
                } else {
                    Some((rest[0], &rest[1..], points))
                }
            } else {
                parse_recursive_counting_closing(rest, points)
            }
        }
        b'<' => {
            let (x, rest, points) = parse_recursive_counting_closing(rest, points)?;
            if x != b'>' {
                let points = points * 5 + 4;
                if x != 0 {
                    None
                } else if rest.is_empty() {
                    Some((0, &[], points))
                } else {
                    Some((rest[0], &rest[1..], points))
                }
            } else {
                parse_recursive_counting_closing(rest, points)
            }
        }
        _ => Some((first, rest, points)),
    }
}
