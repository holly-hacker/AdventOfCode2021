use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    fmt::Display,
    ops::{Index, IndexMut},
};

use aoc_lib::*;
use rustc_hash::FxHashMap;
use tinyvec::ArrayVec;

aoc_setup!(Day23, sample 1: 12521, sample 2: 44169, part 1: 11417, part 2: 49529);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Burrow<const N: usize> {
    side_rooms: [[Option<Amphipod>; N]; 4],
    // NOTE: can remove 4 of these indices, but probably not very important
    hallway: [Option<Amphipod>; 11],
}

const VALID_HALLWAY_INDICES: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];

impl<const N: usize> Burrow<N> {
    pub fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        lines.next().unwrap();
        lines.next().unwrap();
        let mut burrow = Burrow {
            side_rooms: [[None; N]; 4],
            hallway: [None; 11],
        };

        for i in 0..N {
            let line = lines.next().unwrap().as_bytes();
            burrow.side_rooms[0][i] = Amphipod::parse(line[3] as char);
            burrow.side_rooms[1][i] = Amphipod::parse(line[5] as char);
            burrow.side_rooms[2][i] = Amphipod::parse(line[7] as char);
            burrow.side_rooms[3][i] = Amphipod::parse(line[9] as char);
        }

        burrow
    }

    pub fn generate_moves(&self) -> ArrayVec<[Move; (11 - 4) * 4]> {
        let mut ret = ArrayVec::default();

        // 1. generate moves for all items in the hallway
        for i in VALID_HALLWAY_INDICES {
            let from = Location::Hallway(i);
            if self[from].is_none() {
                continue;
            }
            let j = self[from].unwrap().get_room_index();
            let side_room = self.side_rooms[j];

            let can_move_to_location = side_room
                .iter()
                .all(|&x| x.is_none() || x.unwrap().get_room_index() == j);

            if can_move_to_location {
                // reverse loop because we want to find the deepest free point in this room (highest index)
                for n in (0..N).rev() {
                    if side_room[n].is_none() {
                        let to = Location::Sideroom(j, n);
                        let new_move = Move { to, from };
                        new_move.is_valid(self).then(|| ret.push(new_move));
                        break;
                    }
                }
            }
        }

        // 2. generate moves for all top items in the side rooms
        for (i, side_room) in self.side_rooms.iter().enumerate() {
            // find the first filled slot
            if let Some((n, _)) = side_room.iter().enumerate().find(|(_, x)| x.is_some()) {
                VALID_HALLWAY_INDICES
                    .into_iter()
                    .filter(|&j| self.hallway[j].is_none())
                    .map(|j| {
                        let from = Location::Sideroom(i, n);
                        let to = Location::Hallway(j);
                        Move { to, from }
                    })
                    .filter(|x| x.is_valid(self))
                    .for_each(|new_move| ret.push(new_move));
            }
        }

        ret
    }

    #[must_use]
    pub fn after_move(&self, the_move: Move) -> Self {
        let mut new_burrow = self.clone();
        new_burrow.do_move(the_move);
        new_burrow
    }

    fn do_move(&mut self, the_move: Move) {
        debug_assert!(the_move.is_valid(self), "move should be valid");

        self[the_move.to] = self[the_move.from];
        self[the_move.from] = None;
    }

    #[must_use]
    pub fn before_move(&self, the_move: Move) -> Self {
        let mut new_burrow = self.clone();
        new_burrow.undo_move(the_move);
        new_burrow
    }

    fn undo_move(&mut self, the_move: Move) {
        self[the_move.from] = self[the_move.to];
        self[the_move.to] = None;
    }

    pub fn is_solved(&self) -> bool {
        self.side_rooms[0]
            .iter()
            .all(|&x| x == Some(Amphipod::Amber))
            && self.side_rooms[1]
                .iter()
                .all(|&x| x == Some(Amphipod::Bronze))
            && self.side_rooms[2]
                .iter()
                .all(|&x| x == Some(Amphipod::Copper))
            && self.side_rooms[3]
                .iter()
                .all(|&x| x == Some(Amphipod::Desert))
    }
}

impl Burrow<2> {
    pub fn extend(&self) -> Burrow<4> {
        let mut ret = Burrow::<4> {
            hallway: self.hallway,
            ..Default::default()
        };

        ret.side_rooms[0] = [
            self.side_rooms[0][0],
            Some(Amphipod::Desert),
            Some(Amphipod::Desert),
            self.side_rooms[0][1],
        ];
        ret.side_rooms[1] = [
            self.side_rooms[1][0],
            Some(Amphipod::Copper),
            Some(Amphipod::Bronze),
            self.side_rooms[1][1],
        ];
        ret.side_rooms[2] = [
            self.side_rooms[2][0],
            Some(Amphipod::Bronze),
            Some(Amphipod::Amber),
            self.side_rooms[2][1],
        ];
        ret.side_rooms[3] = [
            self.side_rooms[3][0],
            Some(Amphipod::Amber),
            Some(Amphipod::Copper),
            self.side_rooms[3][1],
        ];

        ret
    }
}

impl<const N: usize> Default for Burrow<N> {
    fn default() -> Self {
        Burrow {
            side_rooms: [[None; N]; 4],
            hallway: [None; 11],
        }
    }
}

impl<const N: usize> Display for Burrow<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "#############")?;

        write!(f, "#")?;
        for &hallway_spot in &self.hallway {
            write!(f, "{}", Amphipod::to_char(hallway_spot))?;
        }
        writeln!(f, "#")?;

        for y in 0..N {
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

impl<const N: usize> Index<Location> for Burrow<N> {
    type Output = Option<Amphipod>;

    fn index(&self, index: Location) -> &Self::Output {
        match index {
            Location::Hallway(idx) => &self.hallway[idx],
            Location::Sideroom(hall, idx) => &self.side_rooms[hall][idx],
        }
    }
}

impl<const N: usize> IndexMut<Location> for Burrow<N> {
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
    pub fn cost<const N: usize>(self, burrow: &Burrow<N>) -> usize {
        // println!("calculating cost for {:?}", self);
        debug_assert!(burrow[self.from].is_some());
        debug_assert_eq!(None, burrow[self.to]);

        let base_weight = burrow[self.from].unwrap().weight();
        let manhattan_distance = self.manhattan_distance();

        base_weight * manhattan_distance
    }

    fn manhattan_distance(self) -> usize {
        debug_assert!(
            (!(matches!(self.from, Location::Sideroom(_, _))
                && matches!(self.to, Location::Sideroom(_, _)))),
            "cannot calculate manhattan distance between 2 siderooms yet"
        );
        let ((x1, y1), (x2, y2)) = (self.from.coordinate(), self.to.coordinate());
        let (dx, dy) = (x1.max(x2) - x1.min(x2), y1.max(y2) - y1.min(y2));

        dx + dy
    }

    pub fn is_valid<const N: usize>(self, burrow: &Burrow<N>) -> bool {
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

        // ensure that we're not crossing amphipods in the hallway
        if let Location::Sideroom(x, depth) = self.from {
            // excludes self
            if (0..depth)
                .map(|i| burrow.side_rooms[x][i])
                .any(|it| it.is_some())
            {
                return false;
            }
        }
        if let Location::Sideroom(x, depth) = self.to {
            if (0..=depth)
                .map(|i| burrow.side_rooms[x][i])
                .any(|it| it.is_some())
            {
                return false;
            }
        }

        true
    }

    pub fn cmp_with_burrow<const N: usize>(self, other: Self, burrow: &Burrow<N>) -> Ordering {
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
        Location::Hallway(usize::MAX)
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

    pub const fn to_char(me: Option<Self>) -> char {
        match me {
            Some(Amphipod::Amber) => 'A',
            Some(Amphipod::Bronze) => 'B',
            Some(Amphipod::Copper) => 'C',
            Some(Amphipod::Desert) => 'D',
            None => '.',
        }
    }

    pub const fn weight(self) -> usize {
        match self {
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000,
        }
    }

    pub const fn get_room_index(self) -> usize {
        match self {
            Amphipod::Amber => 0,
            Amphipod::Bronze => 1,
            Amphipod::Copper => 2,
            Amphipod::Desert => 3,
        }
    }
}

pub struct Day23;

impl AdventOfCode for Day23 {
    type Input = Burrow<2>;
    type Output = usize;

    fn parse_input(s: &str) -> Self::Input {
        Burrow::parse(s)
    }

    fn solve_1(input: &Self::Input) -> Self::Output {
        dijkstra(input)
    }

    fn solve_2(input: &Self::Input) -> Self::Output {
        dijkstra(&input.extend())
    }
}

#[derive(PartialEq, Eq)]
struct State<const N: usize> {
    burrow: Burrow<N>,
    cost: usize,
}

impl<const N: usize> PartialOrd for State<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl<const N: usize> Ord for State<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

// TODO: could try a*
fn dijkstra<const N: usize>(input: &Burrow<N>) -> usize {
    // let mut dist = vec![(usize::MAX, None); input.data.len()];
    let mut dist: FxHashMap<Burrow<N>, (usize, Option<Move>)> = FxHashMap::default();
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
            // println!("{}", editable_burrow);
            while let Some(prev) = last_move {
                path.push(prev);
                editable_burrow.undo_move(prev);
                // println!("{}", editable_burrow);
                last_move = dist[&editable_burrow].1;
            }
            path.reverse();
            // dbg!(path);
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
