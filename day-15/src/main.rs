use std::{cmp::Ordering, collections::BinaryHeap};

use aoc_lib::{utils::Field2D, *};

aoc_setup!(Day15, sample 1: 40, sample 2: 315, part 1: 562, part 2: 2874);

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    node: usize,
    cost: usize,
}

// Manually implement Ord so we get a min-heap instead of a max-heap
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Day15;

impl AdventOfCode for Day15 {
    type Input = Field2D<u8>;
    type Output = usize;

    fn parse_input(s: &str) -> Self::Input {
        Field2D::from_str(s)
    }

    fn solve_1(input: &Self::Input) -> Self::Output {
        // I guess we're doing dijkstra today bois
        dijkstra(input)
    }

    fn solve_2(input: &Self::Input) -> Self::Output {
        let mut new_field = Field2D::new(input.width() * 5, input.height() * 5, 0u8);
        input.data.iter().enumerate().for_each(|(i, &data)| {
            (0..5)
                .flat_map(|y| (0..5).map(move |x| (x, y)))
                .for_each(|(x, y)| {
                    let new_pos = (i / input.width()) + ((i % input.width()) * new_field.width());
                    let offset_x = input.stride * x;
                    let offset_y = (5 * input.data.len()) * y;
                    let new_index = new_pos + offset_x + offset_y;
                    let new_data = (((data as usize + x + y - 1) % 9) + 1) as u8;
                    new_field.data[new_index] = new_data;
                });
        });

        dijkstra(&new_field)
    }
}

// ripped off from rosettacode. I'm not spending my entire evening figuring out why dijkstra is slow without a binary heap
fn dijkstra(input: &Field2D<u8>) -> usize {
    let mut dist = vec![(usize::MAX, None); input.data.len()];
    let mut heap = BinaryHeap::new();
    let end = input.data.len() - 1;

    dist[0] = (0, None);

    heap.push(State { node: 0, cost: 0 });

    while let Some(State { node, cost }) = heap.pop() {
        if node == end {
            let mut path = Vec::with_capacity(dist.len() / 2);
            let mut current_dist = dist[end];
            path.push(end);
            while let Some(prev) = current_dist.1 {
                path.push(prev);
                current_dist = dist[prev];
            }
            path.reverse();
            return cost;
        }

        if cost > dist[node].0 {
            continue;
        }
        for edge in input.neighbour_indices(node).into_iter().flatten() {
            let new_cost = input.data[edge] as usize;
            let next = State {
                node: edge,
                cost: cost + new_cost,
            };
            if next.cost < dist[next.node].0 {
                dist[next.node] = (next.cost, Some(node));
                heap.push(next);
            }
        }
    }

    unreachable!()
}
