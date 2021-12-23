use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    fmt::Display,
    ops::{Index, IndexMut},
};

use aoc_lib::*;
use rustc_hash::FxHashMap;
use tinyvec::ArrayVec;

aoc_setup!(Day23, sample 1: 12521, part 1: 11417);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Burrow {
    side_rooms: [[Option<Amphipod>; 2]; 4],
    // TODO: can remove 4 of these indices
    hallway: [Option<Amphipod>; 11],
}

const VALID_HALLWAY_INDICES: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];

impl Burrow {
    pub fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        lines.next().unwrap();
        lines.next().unwrap();
        let mut burrow = Burrow {
            side_rooms: [[None; 2]; 4],
            hallway: [None; 11],
        };

        for i in 0..2 {
            let line = lines.next().unwrap().as_bytes();
            burrow.side_rooms[0][i] = Amphipod::parse(line[3] as char);
            burrow.side_rooms[1][i] = Amphipod::parse(line[5] as char);
            burrow.side_rooms[2][i] = Amphipod::parse(line[7] as char);
            burrow.side_rooms[3][i] = Amphipod::parse(line[9] as char);
        }

        burrow
    }

    // TODO: figure out correct max length, or change to TinyVec
    pub fn generate_moves(&self) -> ArrayVec<[Move; 7 * 4]> {
        let mut ret = ArrayVec::default();

        // 1. generate moves for all items in the hallway
        for (i, _) in self
            .hallway
            .iter()
            .enumerate()
            .filter(|(_, it)| it.is_some())
        {
            let from = Location::Hallway(i);
            for (j, side_room) in self.side_rooms.iter().enumerate() {
                if side_room[1].is_none() {
                    let to = Location::Sideroom(j, 1);
                    let new_move = Move { to, from };
                    new_move.is_valid(self).then(|| ret.push(new_move));
                } else if side_room[0].is_none() {
                    let to = Location::Sideroom(j, 0);
                    let new_move = Move { to, from };
                    new_move.is_valid(self).then(|| ret.push(new_move));
                }
            }

            // TODO: also check all other hallway locations?
        }

        // 2. generate moves for all top items in the side rooms
        for (i, side_room) in self.side_rooms.iter().enumerate() {
            if side_room[0].is_some() {
                VALID_HALLWAY_INDICES
                    .into_iter()
                    .filter(|j| self.hallway[*j].is_none())
                    .for_each(|j| {
                        let from = Location::Sideroom(i, 0);
                        let to = Location::Hallway(j);
                        let new_move = Move { to, from };
                        new_move.is_valid(self).then(|| ret.push(new_move));
                    });
            } else if side_room[1].is_some() {
                VALID_HALLWAY_INDICES
                    .into_iter()
                    .filter(|j| self.hallway[*j].is_none())
                    .for_each(|j| {
                        let from = Location::Sideroom(i, 1);
                        let to = Location::Hallway(j);
                        let new_move = Move { to, from };
                        new_move.is_valid(self).then(|| ret.push(new_move));
                    });
            }
        }

        // 3. sort by weight
        // TODO: not sure if needed
        // ret.sort_unstable_by(|a: &Move, b| a.cmp_with_burrow(*b, self));

        ret
    }

    pub fn after_move(&self, the_move: Move) -> Self {
        let mut new_burrow = self.clone();
        new_burrow.do_move(the_move);
        new_burrow
    }

    fn do_move(&mut self, the_move: Move) {
        debug_assert!(the_move.is_valid(self), "move should be valid");

        self[the_move.to] = self[the_move.from].clone();
        self[the_move.from] = None;
    }

    pub fn before_move(&self, the_move: Move) -> Self {
        let mut new_burrow = self.clone();
        new_burrow.undo_move(the_move);
        new_burrow
    }

    fn undo_move(&mut self, the_move: Move) {
        self[the_move.from] = self[the_move.to].clone();
        self[the_move.to] = None;
    }

    pub fn is_solved(&self) -> bool {
        self.side_rooms
            == [
                [Some(Amphipod::Amber), Some(Amphipod::Amber)],
                [Some(Amphipod::Bronze), Some(Amphipod::Bronze)],
                [Some(Amphipod::Copper), Some(Amphipod::Copper)],
                [Some(Amphipod::Desert), Some(Amphipod::Desert)],
            ]
    }
}

impl Display for Burrow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "#############")?;

        write!(f, "#")?;
        for &hallway_spot in &self.hallway {
            write!(f, "{}", Amphipod::to_char(hallway_spot))?;
        }
        writeln!(f, "#")?;

        for y in 0..2 {
            write!(f, "{}", if y == 0 { "##" } else { "  " })?;
            for x in 0..4 {
                write!(f, "#")?;
                write!(f, "{}", Amphipod::to_char(self.side_rooms[x][y]))?;
            }
            writeln!(f, "{}", if y == 0 { "###" } else { "#" })?;
        }

        writeln!(f, "  #########")?;

        Ok(())
    }
}

impl Index<Location> for Burrow {
    type Output = Option<Amphipod>;

    fn index(&self, index: Location) -> &Self::Output {
        match index {
            Location::Hallway(idx) => &self.hallway[idx],
            Location::Sideroom(hall, idx) => &self.side_rooms[hall][idx],
        }
    }
}

impl IndexMut<Location> for Burrow {
    fn index_mut(&mut self, index: Location) -> &mut Self::Output {
        match index {
            Location::Hallway(idx) => &mut self.hallway[idx],
            Location::Sideroom(hall, idx) => &mut self.side_rooms[hall][idx],
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Move {
    from: Location,
    to: Location,
}

impl Move {
    pub fn cost(self, burrow: &Burrow) -> usize {
        // println!("calculating cost for {:?}", self);
        debug_assert!(burrow[self.from].is_some());
        debug_assert_eq!(None, burrow[self.to]);

        let base_weight = burrow[self.from].unwrap().weight();
        let manhattan_distance = self.manhattan_distance();

        base_weight * manhattan_distance
    }

    fn manhattan_distance(self) -> usize {
        debug_assert!(
            (matches!(self.from, Location::Sideroom(_, _)) as u8
                + matches!(self.to, Location::Sideroom(_, _)) as u8)
                != 2,
            "cannot calculate manhattan distance between 2 siderooms yet"
        );
        let ((x1, y1), (x2, y2)) = (self.from.coordinate(), self.to.coordinate());
        let (dx, dy) = (x1.max(x2) - x1.min(x2), y1.max(y2) - y1.min(y2));

        dx + dy
    }

    pub fn is_valid(self, burrow: &Burrow) -> bool {
        let ((x1, y1), (x2, y2)) = (self.from.coordinate(), self.to.coordinate());

        // if move through hallway, check that we're not crossing existing amphipods
        if y1.min(y2) == 0 {
            let (mut x_min, mut x_max) = (x1.min(x2), x1.max(x2));

            // if one of the bounds is a start/end location, skip it
            if (y1 == 0 && x_min == x1) || (y2 == 0 && x_min == x2) {
                x_min += 1;
            }
            if (y1 == 0 && x_max == x1) || (y2 == 0 && x_max == x2) {
                x_max -= 1;
            }

            if burrow.hallway[x_min..=x_max].iter().any(|it| it.is_some()) {
                // println!("hallway blocked: {:?}", burrow.hallway);
                return false;
            }
        }

        if let Location::Sideroom(x, y) = self.from {
            if y > 0 && burrow.side_rooms[x][0].is_some() {
                // println!("sideroom from blocked: {:?}", burrow.side_rooms[x]);
                return false;
            }
        }

        if let Location::Sideroom(x, y) = self.to {
            if y > 0 && burrow.side_rooms[x][0].is_some() {
                // println!("sideroom to blocked: {:?}", burrow.side_rooms[x]);
                return false;
            }
        }

        true
    }

    pub fn cmp_with_burrow(self, other: Self, burrow: &Burrow) -> Ordering {
        Ord::cmp(&self.cost(burrow), &other.cost(burrow))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Location {
    Hallway(usize),
    Sideroom(usize, usize),
}

impl Location {
    pub fn coordinate(self) -> (usize, usize) {
        match self {
            Location::Hallway(idx) => (idx, 0),
            Location::Sideroom(hall, idx) => (2 + hall * 2, idx + 1),
        }
    }
}

impl Default for Location {
    fn default() -> Self {
        Location::Hallway(99)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    pub fn parse(input: char) -> Option<Amphipod> {
        match input {
            'A' => Some(Amphipod::Amber),
            'B' => Some(Amphipod::Bronze),
            'C' => Some(Amphipod::Copper),
            'D' => Some(Amphipod::Desert),
            '.' => None,
            _ => panic!("Unknown Amphipod: {}", input),
        }
    }

    pub fn to_char(me: Option<Self>) -> char {
        match me {
            Some(Amphipod::Amber) => 'A',
            Some(Amphipod::Bronze) => 'B',
            Some(Amphipod::Copper) => 'C',
            Some(Amphipod::Desert) => 'D',
            None => '.',
        }
    }

    pub fn weight(self) -> usize {
        match self {
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000,
        }
    }
}

pub struct Day23;

impl AdventOfCode for Day23 {
    type Input = Burrow;
    type Output = usize;

    fn parse_input(s: &str) -> Self::Input {
        Burrow::parse(s)
    }

    fn solve_1(input: &Self::Input) -> Self::Output {
        dijkstra(input)
    }

    fn solve_2(_input: &Self::Input) -> Self::Output {
        0
    }
}

#[derive(PartialEq, Eq)]
struct State {
    burrow: Burrow,
    cost: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

// TODO: could try a*
fn dijkstra(input: &Burrow) -> usize {
    // let mut dist = vec![(usize::MAX, None); input.data.len()];
    let mut dist: FxHashMap<Burrow, (usize, Option<Move>)> = FxHashMap::default();
    let mut heap = BinaryHeap::new();

    dist.insert(input.clone(), (0, None));

    heap.push(State {
        burrow: input.clone(),
        cost: 0,
    });

    while let Some(State { burrow, cost }) = heap.pop() {
        // exit out if we're done
        if burrow.is_solved() {
            let mut editable_burrow = burrow.clone();
            let mut path = Vec::new();
            let mut last_move = dist[&burrow].1;
            path.push(last_move.unwrap());
            println!("{}", editable_burrow);
            while let Some(prev) = last_move {
                path.push(prev);
                editable_burrow.undo_move(prev);
                println!("{}", editable_burrow);
                last_move = dist[&editable_burrow].1;
            }
            path.reverse();
            dbg!(path);
            return cost;
        }

        // if we've been here before in a faster way, skip
        if !dist.contains_key(&burrow) || cost > dist[&burrow].0 {
            // println!("skipping {:?}", burrow);
            continue;
        }

        // look at every candidate move
        for new_move in burrow.generate_moves() {
            // calculate next state
            let next = State {
                burrow: burrow.after_move(new_move),
                cost: cost + new_move.cost(&burrow),
            };

            // println!("new path with cost {}", next.cost);

            // if this is a cheaper way than the previous way to get to this state, store it
            if !dist.contains_key(&next.burrow) || next.cost < dist[&next.burrow].0 {
                dist.insert(next.burrow.clone(), (next.cost, Some(new_move)));
                heap.push(next);
            }
        }
    }

    // println!("{:?}", dist);

    panic!("failed to find a path")
}
