use aoc_lib::*;
use itertools::Itertools;
use std::ops::IndexMut;

aoc_setup!(Day4, sample 1: 4512, sample 2: 1924, part 1: 16674, part 2: 7075);

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
        const INDICES: [u32; 10] = [
            // horizontal
            0b00000_00000_00000_00000_11111,
            0b00000_00000_00000_11111_00000,
            0b00000_00000_11111_00000_00000,
            0b00000_11111_00000_00000_00000,
            0b11111_00000_00000_00000_00000,
            // vertical
            0b00001_00001_00001_00001_00001,
            0b00010_00010_00010_00010_00010,
            0b00100_00100_00100_00100_00100,
            0b01000_01000_01000_01000_01000,
            0b10000_10000_10000_10000_10000,
        ];

        INDICES.into_iter().any(|row| row & mask == row)
    }
}

pub struct Day4;

impl AdventOfCode for Day4 {
    type Input = Input;
    type Output = usize;

    fn parse_input(s: &str) -> Self::Input {
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

    fn solve_1(input: &Self::Input) -> Self::Output {
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

    fn solve_2(input: &Self::Input) -> Self::Output {
        let mut solve_mask = vec![0u32; input.1.len()];
        let mut last_num = 0;
        let mut solved_list = vec![0usize; (input.1.len() / (usize::BITS as usize)) + 1];
        let mut last_solved = 0;
        let mut last_mask = 0;
        let mut solved_count = 0; // i'm too lazy to write a check against solved_list

        input
            .0
            .iter()
            .filter_map(|&num| {
                input
                    .1
                    .iter()
                    .enumerate()
                    .filter_map(|(i, board)| {
                        board.update_mask(num, solve_mask.index_mut(i));

                        let solved_ref = solved_list.index_mut(i / (usize::BITS as usize));
                        let solved_bit_idx = i % (usize::BITS as usize);

                        if ((*solved_ref) & (1 << solved_bit_idx)) == 0 {
                            if Board::is_solved(solve_mask[i]) {
                                last_num = num as usize;
                                last_solved = i;
                                last_mask = solve_mask[last_solved];
                                *solved_ref |= 1 << solved_bit_idx;
                                solved_count += 1;

                                // check if everything is solved
                                if solved_count == input.1.len() {
                                    return Some(());
                                }
                            }
                        }

                        None
                    })
                    .next()
            })
            .next()
            .unwrap();

        input.1[last_solved].get_unmarked_sum(last_mask) * last_num
    }
}
