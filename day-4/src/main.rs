use std::ops::IndexMut;

use aoc_lib::*;
use itertools::Itertools;

#[derive(Debug)]
pub struct Input(Vec<u8>, Vec<Board>);

#[derive(Debug)]
pub struct Board([u8; 5 * 5]);

impl Board {
    fn update_mask(&self, num: u8, mask: &mut u32) {
        (0usize..25)
            .filter(|&i| self.0[i] == num)
            .for_each(|i| *mask |= 1 << i);
    }

    fn get_unmarked_sum(&self, mask: u32) -> usize {
        (0usize..25)
            .filter(|&i| (mask & (1 << i)) == 0)
            .map(|i| self.0[i] as usize)
            .sum()
    }

    fn is_solved(mask: u32) -> bool {
        // TODO: bitmasks instead
        const INDICES: [[i32; 5]; 10] = [
            // horizontal
            [0, 1, 2, 3, 4],
            [5, 6, 7, 8, 9],
            [10, 11, 12, 13, 14],
            [15, 16, 17, 18, 19],
            [20, 21, 22, 23, 24],
            // vertical
            [0, 5, 10, 15, 20],
            [1, 6, 11, 16, 21],
            [2, 7, 12, 17, 22],
            [3, 8, 13, 18, 23],
            [4, 9, 14, 19, 24],
            // diagonal
            // [0, 6, 12, 18, 24],
            // [4, 8, 12, 16, 20],
        ];

        INDICES
            .into_iter()
            .any(|row| row.into_iter().all(|i| mask & (1 << i) != 0))
    }
}

fn main() {
    let input = read_stdin();
    let (parsed, parsed_time) = time(|| parse_input(&input));
    let (solve_1, solve_1_time) = time(|| solve_1(&parsed));
    let (solve_2, solve_2_time) = time(|| solve_2(&parsed));

    println!("Solution to part 1: {}", solve_1);
    println!("Solution to part 2: {}", solve_2);

    println!("Parsing took: {:?}", parsed_time);
    println!("Solving part 1 took: {:?}", solve_1_time);
    println!("Solving part 2 took: {:?}", solve_2_time);
}

fn parse_input(s: &str) -> Input {
    let mut iter = s.lines().filter(|line| line.len() > 0);

    let x = iter
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect();

    let boards = iter
        .chunks(5)
        .into_iter()
        .map(|chunk| {
            let mut numbers = [0; 5 * 5];
            for (i, n) in chunk.into_iter().enumerate() {
                n.split(' ')
                    .filter(|x| x.len() > 0)
                    .enumerate()
                    .for_each(|(j, s)| {
                        numbers[i * 5 + j] = s.parse::<u8>().unwrap();
                    });
            }
            Board(numbers)
        })
        .collect();

    Input(x, boards)
}

fn solve_1(input: &Input) -> usize {
    let mut solve_mask = vec![0u32; input.1.len()];
    let mut last_num = 0;

    let board_index = input
        .0
        .iter()
        .find_map(|&num| {
            input.1.iter().enumerate().find_map(|(i, board)| {
                board.update_mask(num, solve_mask.index_mut(i));
                if Board::is_solved(solve_mask[i]) {
                    last_num = num as usize;
                    Some(i)
                } else {
                    None
                }
            })
        })
        .unwrap();

    input.1[board_index].get_unmarked_sum(solve_mask[board_index]) * last_num
}

fn solve_2(input: &Input) -> usize {
    let mut solve_mask = vec![0u32; input.1.len()];
    let mut last_num = 0;
    let mut solved_list = vec![0usize; (input.1.len() / (usize::BITS as usize)) + 1];
    let mut last_solved = 0;
    let mut last_mask = 0;

    input.0.iter().for_each(|&num| {
        input.1.iter().enumerate().for_each(|(i, board)| {
            board.update_mask(num, solve_mask.index_mut(i));

            let solved_ref = solved_list.index_mut(i / (usize::BITS as usize));
            let solved_bit_idx = i % (usize::BITS as usize);

            if ((*solved_ref) & (1 << solved_bit_idx)) == 0 {
                if Board::is_solved(solve_mask[i]) {
                    last_num = num as usize;
                    last_solved = i;
                    last_mask = solve_mask[last_solved];
                    *solved_ref |= 1 << solved_bit_idx;

                    // TODO: early exit
                }
            }
        });
    });

    input.1[last_solved].get_unmarked_sum(last_mask) * last_num
}

#[test]
fn test_solve_1() {
    let input = include_str!("../sample.txt");
    let parsed = parse_input(input);
    assert_eq!(4512, solve_1(&parsed));
}

#[test]
fn test_solve_2() {
    let input = include_str!("../sample.txt");
    let parsed = parse_input(input);
    assert_eq!(1924, solve_2(&parsed));
}
