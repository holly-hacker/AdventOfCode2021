use aoc_lib::utils::*;
use aoc_lib::*;

aoc_setup!(Day9, sample 1: 15, sample 2: 1134, part 1: 458, part 2: 1391940);

pub struct Day9;

impl AdventOfCode for Day9 {
    type Input = Field2D<u8>;
    type Output = usize;

    fn parse_input(s: &str) -> Self::Input {
        Field2D::parse(s)
    }

    fn solve_1(input: &Self::Input) -> Self::Output {
        (0..input.data.len())
            .filter_map(|idx| {
                let center = input.data[idx];
                let higher_neighbours = input
                    .neighbour_indices(idx)
                    .iter()
                    .filter(|&&n| match n {
                        Some(n) => input.data[n as usize] > center,
                        None => true,
                    })
                    .count();
                if higher_neighbours == 4 {
                    Some((center + 1) as usize)
                } else {
                    None
                }
            })
            .sum::<usize>()
    }

    fn solve_2(input: &Self::Input) -> Self::Output {
        let mut basin_field = Field2D::<usize> {
            data: vec![0; input.data.len()],
            stride: input.stride,
        };
        let (a, b, c) = flood_fill(input, &mut basin_field);

        a * b * c
    }
}

/// executes a flood fill algorithm and returns the sizes of the largest 3 regions
pub fn flood_fill(field: &Field2D<u8>, target: &mut Field2D<usize>) -> (usize, usize, usize) {
    let mut idx = 0;
    let mut num = 1;
    let mut regions = (0, 0, 0);
    loop {
        let idx_data = target.data.get(idx);
        if idx_data.is_none() {
            break;
        }
        if *idx_data.unwrap() != 0 || field.data[idx] == 9 {
            idx += 1;
            continue;
        }

        let size = flood_fill_recursive(field, target, num, idx, 0);

        if size < regions.0 {
            regions = (regions.0, regions.1, regions.2);
        } else if size < regions.1 {
            regions = (size, regions.1, regions.2);
        } else if size < regions.2 {
            regions = (regions.1, size, regions.2);
        } else {
            regions = (regions.1, regions.2, size);
        }

        num += 1;
        idx += 1;
    }

    regions
}

/// flood fills a single region and returns its size
fn flood_fill_recursive(
    field: &Field2D<u8>,
    target: &mut Field2D<usize>,
    num: usize,
    idx: usize,
    mut size: usize,
) -> usize {
    if idx >= target.data.len() || target.data[idx] != 0 {
        return size;
    }

    if field.data[idx] == 9 {
        return size;
    }

    target.data[idx] = num;
    size += 1;

    field
        .neighbour_indices(idx)
        .into_iter()
        .flatten()
        .for_each(|idx| size = flood_fill_recursive(field, target, num, idx, size));

    size
}
