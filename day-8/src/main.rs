use aoc_lib::*;

aoc_setup!(Day8, sample 1: 26, sample 2: 61229, part 1: 530, part 2: 1051087);

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Digit(u8, u32);

impl std::fmt::Debug for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Digit({:#09b})", self.0)
    }
}

impl Digit {
    pub fn parse(input: &str) -> Self {
        debug_assert!(input.len() <= 7);

        let x = input.as_bytes().iter().fold(0u8, |ret, &c| {
            debug_assert!(c >= b'a' && c <= b'g');
            ret | 1 << (7 + c - b'g')
        });

        Digit(x, x.count_ones())
    }

    pub fn active_segments(&self) -> u32 {
        self.1
    }

    pub fn get_number_from_segments(&self) -> Option<usize> {
        match self.active_segments() {
            2 => Some(1),
            4 => Some(4),
            3 => Some(7),
            7 => Some(8),
            _ => None,
        }
    }
}

pub struct InputLine {
    input: [Digit; 10],
    output: [Digit; 4],
}

pub struct Day8;

impl AdventOfCode for Day8 {
    type Input = Vec<InputLine>;
    type Output = usize;

    fn parse_input(s: &str) -> Self::Input {
        s.lines()
            .map(|l| {
                let (input_str, output_str) = l.split_once(" | ").unwrap();

                let mut input = [Digit(0, 0); 10];
                let mut output = [Digit(0, 0); 4];

                input_str.split(' ').enumerate().for_each(|(i, s)| {
                    input[i] = Digit::parse(s);
                });
                output_str.split(' ').enumerate().for_each(|(i, s)| {
                    output[i] = Digit::parse(s);
                });

                InputLine { input, output }
            })
            .collect()
    }

    fn solve_1(input: &Self::Input) -> Self::Output {
        input
            .iter()
            .map(|line| line.output)
            .flatten()
            .filter(|digit| digit.get_number_from_segments().is_some())
            .count()
    }

    fn solve_2(input: &Self::Input) -> Self::Output {
        input.iter().map(solve_2_single_line).sum()
    }
}

pub fn solve_2_single_line(line: &InputLine) -> usize {
    let mut known_digits = [Digit(0, 0); 10];

    for digit in line.input {
        match digit.get_number_from_segments() {
            Some(1) => known_digits[1] = digit,
            Some(4) => known_digits[4] = digit,
            Some(7) => known_digits[7] = digit,
            Some(8) => known_digits[8] = digit,
            _ => (),
        };
    }

    // can know what is 0, 6 and 9 through the segment that differs with digit 8
    for digit in line.input {
        match digit.active_segments() {
            // 0, 6, 9
            6 => {
                // segment that differs from 8
                let differing_segment = digit.0 ^ known_digits[8].0;
                debug_assert!(differing_segment.count_ones() == 1);

                // check if this segment is in digit 1
                // if it is, it is 6
                if differing_segment & known_digits[1].0 != 0 {
                    known_digits[6] = digit;
                } else {
                    // check if the segment is in digit 4
                    // if it is, it is 0
                    if differing_segment & known_digits[4].0 != 0 {
                        known_digits[0] = digit;
                    } else {
                        // this should be digit 0
                        known_digits[9] = digit;
                    }
                }
            }
            // 2, 3, 5
            5 => {
                let matching_segments = digit.0 & known_digits[1].0;
                if matching_segments.count_ones() == 2 {
                    // the digit contains all segments of 1
                    known_digits[3] = digit;
                }
            }
            _ => (),
        }
    }

    // remaining is 2 and 5. just compare to one of the other numbers
    // in our case, use 6
    for digit in line.input {
        if digit.active_segments() == 5 && digit != known_digits[3] {
            let matching_segments = digit.0 & known_digits[6].0;

            match matching_segments.count_ones() {
                5 => known_digits[5] = digit,
                4 => known_digits[2] = digit,
                _ => unreachable!(),
            }
        }
    }

    // all digits are known!
    let thousands = known_digits
        .iter()
        .position(|&x| line.output[0] == x)
        .unwrap()
        * 1000;
    let hundreds = known_digits
        .iter()
        .position(|&x| line.output[1] == x)
        .unwrap()
        * 100;
    let tens = known_digits
        .iter()
        .position(|&x| line.output[2] == x)
        .unwrap()
        * 10;
    let the_rest_idk = known_digits
        .iter()
        .position(|&x| line.output[3] == x)
        .unwrap();

    thousands + hundreds + tens + the_rest_idk
}
