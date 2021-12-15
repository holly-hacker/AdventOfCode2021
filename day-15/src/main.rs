use aoc_lib::{utils::Field2D, *};

aoc_setup!(Day1, sample 1: 40, sample 2: 315, part 1: 562);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DijkstraPair {
    distance: usize,
    last_index: usize,
    solved: bool,
}

impl Default for DijkstraPair {
    fn default() -> Self {
        Self {
            distance: usize::MAX,
            last_index: 0,
            solved: false,
        }
    }
}

impl PartialOrd for DijkstraPair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl Ord for DijkstraPair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

pub struct Day1;

impl AdventOfCode for Day1 {
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

fn dijkstra(input: &Field2D<u8>) -> usize {
    let mut distances = vec![DijkstraPair::default(); input.data.len()];

    distances[0] = DijkstraPair {
        distance: 0,
        last_index: 0,
        solved: true,
    };

    // fill in original neighbours
    input
        .neighbour_indices(0)
        .into_iter()
        .flatten()
        .for_each(|neighbour_idx| {
            distances[neighbour_idx] = DijkstraPair {
                distance: input.data[neighbour_idx] as usize,
                last_index: 0,
                solved: false,
            }
        });

    loop {
        // find lowest cost
        let (cheapest_idx, cheapest) =
            distances
                .iter()
                .enumerate()
                .fold((0, DijkstraPair::default()), |acc, i| {
                    if !i.1.solved && i.1.distance < acc.1.distance {
                        (i.0, *i.1)
                    } else {
                        acc
                    }
                });

        // if lowest_cost is end, we're done
        if cheapest_idx == input.data.len() - 1 {
            return cheapest.distance;
        }

        // mark as solved, ie. we found the shortest path to it
        distances[cheapest_idx].solved = true;

        // loop through unsolved neighbours where last node is lowest_cost_idx, and update their cost if lower
        input
            .neighbour_indices(cheapest_idx)
            .into_iter()
            .flatten()
            .for_each(|idx| {
                // cannot be inside filter block because of borrow checker, boo!
                if distances[idx].solved {
                    return;
                }

                let cur = distances[idx];
                let cur_cost = input.data[idx] as usize;

                // check if new path (through `cheapest`) is cheaper
                let proposed_distance = cheapest.distance + cur_cost;
                if proposed_distance < cur.distance {
                    distances[idx] = DijkstraPair {
                        distance: proposed_distance,
                        last_index: cheapest_idx,
                        solved: false,
                    };
                }
            });
    }
}
