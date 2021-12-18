use std::{fmt::Display, ops::Add};

use aoc_lib::*;

aoc_setup!(Day18, sample 1: 4140, part 1: 4184, sample 2: 3993);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnailfishNumber(Vec<Token>); // TODO: try using tinyvec

impl SnailfishNumber {
    pub fn parse(line: &str) -> Self {
        SnailfishNumber(
            line.chars()
                .filter_map(|c| match c {
                    '[' => Some(Token::Open),
                    '0'..='9' => Some(Token::Number(c.to_digit(10).unwrap() as u8)),
                    _ => None,
                })
                .collect(),
        )
    }

    pub fn reduce(&mut self) {
        while self.reduce_once() {}
    }

    pub fn reduce_once(&mut self) -> bool {
        // each value in this stack indicates the index in the pair
        let mut stack = tinyvec::ArrayVec::<[u8; 16]>::new();

        #[derive(Debug)]
        enum Action {
            Explode,
            Split,
        }

        let mut action = None;

        'outer: for (i, &token) in self.0.iter().enumerate() {
            match token {
                Token::Open => {
                    stack.push(0);
                }
                Token::Number(n) => {
                    if stack.len() > 4 {
                        // explode!
                        action = Some((Action::Explode, i));
                        break;
                    }

                    if n > 9 {
                        if action.is_none() {
                            action = Some((Action::Split, i));
                        }
                    }

                    let stack_top = stack.len() - 1; // fucking borrowchecker
                    stack[stack_top] += 1;

                    // pop the pairs we're leaving
                    while stack[stack.len() - 1] >= 2 {
                        debug_assert!(stack[stack.len() - 1] == 2);
                        stack.pop();

                        // TODO: this condition doesnt make sense, it shouldnt be needed
                        if stack.len() != 0 {
                            let stack_top = stack.len() - 1; // fucking borrowchecker
                            stack[stack_top] += 1;
                        } else {
                            break 'outer;
                        }
                    }
                }
            }
        }

        match action {
            Some((Action::Explode, i)) => {
                self.explode_at(i);
                true
            }
            Some((Action::Split, i)) => {
                self.split_at(i);
                true
            }
            None => {
                debug_assert_eq!(stack.len(), 0);
                false
            }
        }
    }

    fn explode_at(&mut self, index: usize) {
        debug_assert!(self.0[index] != Token::Open);

        let (left, right) = (
            self.0[index].unwrap_number(),
            self.0[index + 1].unwrap_number(),
        );
        self.0.remove(index + 1);
        self.0[index] = Token::Number(0);

        for i in (0..index).rev() {
            if let Token::Number(n) = &self.0[i] {
                self.0[i] = Token::Number(n + left);
                break;
            }
        }

        for i in (index + 1)..self.0.len() {
            if let Token::Number(n) = &self.0[i] {
                self.0[i] = Token::Number(n + right);
                break;
            }
        }
        self.0.remove(index - 1);
    }

    fn split_at(&mut self, index: usize) {
        debug_assert!(self.0[index] != Token::Open);

        let value = self.0[index].unwrap_number();
        let (left, right) = if value % 2 == 0 {
            (value / 2, value / 2)
        } else {
            (value / 2 + 1, value / 2)
        };
        self.0.insert(index, Token::Open);
        self.0[index + 1] = Token::Number(left);
        self.0.insert(index + 1, Token::Number(right));
    }

    pub fn magnitude(&self) -> usize {
        let ret = Self::magnitude_internal(&self.0);
        debug_assert!(ret.0.is_empty());
        ret.1
    }

    fn magnitude_internal(tokens: &[Token]) -> (&[Token], usize) {
        match tokens[0] {
            Token::Open => {
                let (remainder, lhs) = Self::magnitude_internal(&tokens[1..]);
                let (remainder, rhs) = Self::magnitude_internal(remainder);
                (remainder, lhs * 3 + rhs * 2)
            }
            Token::Number(a) => (&tokens[1..], a as usize),
        }
    }
}

impl Add for SnailfishNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl Add for &SnailfishNumber {
    type Output = SnailfishNumber;

    fn add(self, rhs: Self) -> Self::Output {
        let mut new_num_data = vec![Token::Open];
        new_num_data.extend(self.0.clone());
        new_num_data.extend(rhs.0.clone());
        let mut num = SnailfishNumber(new_num_data);
        num.reduce();
        num
    }
}

impl Display for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // using stack-based method from `reduce` for ability to place ]
        let mut stack = tinyvec::ArrayVec::<[u8; 16]>::new();
        for token in &self.0 {
            match token {
                Token::Open => {
                    stack.push(0);
                    write!(f, "[")?;
                }
                Token::Number(n) => {
                    let stack_top = stack.len() - 1; // fucking borrowchecker
                    stack[stack_top] += 1;

                    write!(f, "{}", n)?;

                    // pop the pairs we're leaving
                    while stack[stack.len() - 1] >= 2 {
                        debug_assert!(stack[stack.len() - 1] == 2);
                        stack.pop();
                        write!(f, "]")?;

                        // TODO: this condition doesnt make sense, it shouldnt be needed
                        if stack.len() != 0 {
                            let stack_top = stack.len() - 1; // fucking borrowchecker
                            stack[stack_top] += 1;
                        } else {
                            break;
                        }
                    }
                }
            };
            if !stack.is_empty() && stack[stack.len() - 1] == 1 {
                write!(f, ",")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Open,
    Number(u8),
    // Closed is not needed
}

impl Token {
    pub fn unwrap_number(self) -> u8 {
        match self {
            Token::Number(n) => n,
            _ => panic!("unwrap_number called on non-number"),
        }
    }
}

pub struct Day18;

impl AdventOfCode for Day18 {
    type Input = Vec<SnailfishNumber>;
    type Output = usize;

    fn parse_input(s: &str) -> Self::Input {
        s.lines().map(SnailfishNumber::parse).collect()
    }

    fn solve_1(input: &Self::Input) -> Self::Output {
        input
            .iter()
            .cloned() // eww
            .reduce(|acc, i| acc + i)
            .unwrap()
            .magnitude()
    }

    fn solve_2(input: &Self::Input) -> Self::Output {
        (0..(input.len() * input.len()))
            .into_iter()
            .map(|i| {
                let a = i % input.len();
                let b = i / input.len();
                if a != b {
                    (&input[a] + &input[b]).magnitude()
                } else {
                    0
                }
            })
            .max()
            .unwrap()
    }
}

macro_rules! magnitude_test {
    ($name:ident, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let input = SnailfishNumber::parse($input);
            let expected = $expected;
            let actual = SnailfishNumber::magnitude(&input);
            assert_eq!(actual, expected);
        }
    };
}

macro_rules! reduce_test {
    ($name:ident, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let mut actual = SnailfishNumber::parse($input);
            let expected = SnailfishNumber::parse($expected);
            actual.reduce_once();
            assert_eq!(actual, expected);
        }
    };
}

magnitude_test!(magnitude_test_1, "[[1, 2], 3]", (3 * 1 + 2 * 2) * 3 + 2 * 3);
magnitude_test!(magnitude_test_2, "[1, [2, 3]]", 3 * 1 + 2 * (3 * 2 + 2 * 3));
magnitude_test!(magnitude_test_ex_1, "[[1,2],[[3,4],5]]", 143);
magnitude_test!(
    magnitude_test_ex_2,
    "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
    1384
);
magnitude_test!(magnitude_test_ex_3, "[[[[1,1],[2,2]],[3,3]],[4,4]]", 445);
magnitude_test!(magnitude_test_ex_4, "[[[[3,0],[5,3]],[4,4]],[5,5]]", 791);
magnitude_test!(magnitude_test_ex_5, "[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137);
magnitude_test!(
    magnitude_test_ex_6,
    "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
    3488
);

reduce_test!(reduce_test_none, "[1, [2, 3]]", "[1, [2, 3]]");
reduce_test!(
    reduce_test_explode_ex_1,
    "[[[[[9,8],1],2],3],4]",
    "[[[[0,9],2],3],4]"
);
reduce_test!(
    reduce_test_explode_ex_2,
    "[7,[6,[5,[4,[3,2]]]]]",
    "[7,[6,[5,[7,0]]]]"
);
reduce_test!(
    reduce_test_explode_ex_3,
    "[[6,[5,[4,[3,2]]]],1]",
    "[[6,[5,[7,0]]],3]"
);
reduce_test!(
    reduce_test_explode_ex_4,
    "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
    "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
);
reduce_test!(
    reduce_test_explode_ex_5,
    "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
    "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"
);

// NOTE: cannot test split because parser cannot handle numbers higher than 9

#[test]
fn test_explode_at() {
    let mut original = SnailfishNumber::parse("[1, [[2, 3], 4]");
    original.explode_at(4);
    assert_eq!(original, SnailfishNumber::parse("[3, [0, 7]]"));
}

#[test]
fn test_split_at_even() {
    let mut original = SnailfishNumber::parse("[1, [4, 4]]");
    original.split_at(3);
    assert_eq!(original, SnailfishNumber::parse("[1, [[2, 2], 4]"));
}

#[test]
fn test_split_at_odd() {
    let mut original = SnailfishNumber::parse("[1, [3, 4]]");
    original.split_at(3);
    assert_eq!(original, SnailfishNumber::parse("[1, [[1, 2], 4]"));
}

#[test]
fn test_sum_example_basic() {
    let a = SnailfishNumber::parse("[[[[4,3],4],4],[7,[[8,4],9]]]");
    let b = SnailfishNumber::parse("[1, 1]");
    let actual = a + b;
    let expected = SnailfishNumber::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");

    assert_eq!(actual, expected);
}

#[test]
fn test_sum_example_small() {
    let actual = ["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"]
        .into_iter()
        .map(|s| SnailfishNumber::parse(s))
        .reduce(|acc, i| acc + i)
        .unwrap();
    let expected = SnailfishNumber::parse("[[[[5,0],[7,4]],[5,5]],[6,6]]");

    assert_eq!(actual, expected);
}

#[test]
fn for_fucks_sake() {
    let actual = SnailfishNumber::parse("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]")
        + SnailfishNumber::parse(
            "
        + [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
        );
    let expected =
        SnailfishNumber::parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]");

    assert_eq!(actual, expected);
}
