use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Neg, Sub},
};

use aoc_lib::*;

aoc_setup!(Day19, sample 1: 79, sample 2: 3621, part 1: 398, part 2: 10965);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Vector3(pub i16, pub i16, pub i16);

impl Vector3 {
    pub fn parse(line: &str) -> Self {
        let mut parts = line.split(',');
        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();
        let z = parts.next().unwrap().parse().unwrap();
        Vector3(x, y, z)
    }

    // transform from normal space into the given space
    pub fn transform_to(mut self, direction: FacingDirection) -> Self {
        self = Vector3(
            match direction.axes.0 {
                Axis::X => self.0,
                Axis::Y => self.1,
                Axis::Z => self.2,
            },
            match direction.axes.1 {
                Axis::X => self.0,
                Axis::Y => self.1,
                Axis::Z => self.2,
            },
            match direction.axes.2 {
                Axis::X => self.0,
                Axis::Y => self.1,
                Axis::Z => self.2,
            },
        );

        direction.invert_x.then(|| self.0 = -self.0);
        direction.invert_y.then(|| self.1 = -self.1);
        direction.invert_z.then(|| self.2 = -self.2);

        self
    }

    // transform from the given space into normal space
    pub fn transform_from(mut self, direction: FacingDirection) -> Self {
        direction.invert_x.then(|| self.0 = -self.0);
        direction.invert_y.then(|| self.1 = -self.1);
        direction.invert_z.then(|| self.2 = -self.2);

        let clone = self;

        match direction.axes.0 {
            Axis::X => self.0 = clone.0,
            Axis::Y => self.1 = clone.0,
            Axis::Z => self.2 = clone.0,
        }

        match direction.axes.1 {
            Axis::X => self.0 = clone.1,
            Axis::Y => self.1 = clone.1,
            Axis::Z => self.2 = clone.1,
        }

        match direction.axes.2 {
            Axis::X => self.0 = clone.2,
            Axis::Y => self.1 = clone.2,
            Axis::Z => self.2 = clone.2,
        }

        self
    }

    pub fn is_within_range(&self, other: &Self, range: i16) -> bool {
        (self.0 - other.0).abs() <= range
            && (self.1 - other.1).abs() <= range
            && (self.2 - other.2).abs() <= range
    }

    pub fn manhattan_distance(&self, other: &Self) -> i16 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs() + (self.2 - other.2).abs()
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            0: self.0 + rhs.0,
            1: self.1 + rhs.1,
            2: self.2 + rhs.2,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            0: self.0 - rhs.0,
            1: self.1 - rhs.1,
            2: self.2 - rhs.2,
        }
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector3(-self.0, -self.1, -self.2)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, Copy)]
pub struct FacingDirection {
    pub invert_x: bool,
    pub invert_y: bool,
    pub invert_z: bool,
    pub axes: (Axis, Axis, Axis),
}

impl Default for FacingDirection {
    fn default() -> Self {
        Self {
            invert_x: false,
            invert_y: false,
            invert_z: false,
            axes: (Axis::X, Axis::Y, Axis::Z),
        }
    }
}

const AXIS_PERMUTATIONS: [(Axis, Axis, Axis); 6] = [
    (Axis::X, Axis::Y, Axis::Z),
    (Axis::X, Axis::Z, Axis::Y),
    (Axis::Y, Axis::X, Axis::Z),
    (Axis::Y, Axis::Z, Axis::X),
    (Axis::Z, Axis::X, Axis::Y),
    (Axis::Z, Axis::Y, Axis::X),
];

impl FacingDirection {
    pub fn iter_all() -> impl Iterator<Item = FacingDirection> {
        struct FacingDirectionIter(u8);

        impl Iterator for FacingDirectionIter {
            type Item = FacingDirection;

            fn next(&mut self) -> Option<Self::Item> {
                // TODO: should be 24 instead of 48?
                if self.0 == 8 * 6 {
                    return None;
                }

                let axes_i = (self.0 % 6) as usize;
                let other = self.0 / 6;
                let (nx, ny, nz) = (other & 0b001 != 0, other & 0b010 != 0, other & 0b100 != 0);
                self.0 += 1;
                Some(FacingDirection {
                    invert_x: nx,
                    invert_y: ny,
                    invert_z: nz,
                    axes: AXIS_PERMUTATIONS[axes_i],
                })
            }
        }

        FacingDirectionIter(0)
    }

    pub fn iter_axis() -> impl Iterator<Item = FacingDirection> {
        AXIS_PERMUTATIONS.into_iter().map(|axes| Self {
            invert_x: false,
            invert_y: false,
            invert_z: false,
            axes,
        })
    }
}

pub struct Day19;

impl AdventOfCode for Day19 {
    type Input = Vec<Vec<Vector3>>; // TODO: vec<vec3> prob has fixed lenght, can optimize for that
    type Output = usize;

    fn parse_input(s: &str) -> Self::Input {
        let mut input = vec![];
        let mut iter = s.lines();
        for i in 0.. {
            let line = match iter.next() {
                Some(line) => line,
                None => break,
            };
            debug_assert_eq!(line, format!("--- scanner {} ---", i));

            let mut vector_list = vec![];

            for line in iter.by_ref() {
                if line.is_empty() {
                    break;
                }

                vector_list.push(Vector3::parse(line));
            }

            input.push(vector_list);
        }

        input
    }

    fn solve_1(input: &Self::Input) -> Self::Output {
        calculate_positions(input).1.len()
    }

    fn solve_2(input: &Self::Input) -> Self::Output {
        let scanners = calculate_positions(input).0;
        let positions = scanners
            .into_iter()
            .map(|(_, (pos, _))| pos)
            .collect::<Vec<_>>();
        (0..positions.len())
            .flat_map(|a| (a + 1..positions.len()).map(move |b| (a, b)))
            .map(|(a, b)| positions[a].manhattan_distance(&positions[b]))
            .max()
            .unwrap() as usize
    }
}

fn calculate_positions(
    input: &[Vec<Vector3>],
) -> (HashMap<usize, (Vector3, FacingDirection)>, HashSet<Vector3>) {
    // TODO: can just be [Option<FacingDirection>; 32] or tiny_vec::ArrayVec<[Option<FacingDirection>; 32]>
    let mut directions = HashMap::new();
    directions.insert(0, (Vector3::default(), FacingDirection::default()));

    // create a collection with all known locations
    let mut known_locations = HashSet::new();
    for &probe in &input[0] {
        known_locations.insert(probe);
    }

    let mut iteration_count = 0;
    while directions.len() != input.len() {
        for (i, list) in input.iter().enumerate() {
            if directions.contains_key(&i) {
                continue;
            }

            // check if the list we're given matches at least 12 nodes in the known locations
            let valid_directions = FacingDirection::iter_all()
                .filter_map(|direction| {
                    compare_sets(&known_locations, list, direction).map(|x| (x, direction))
                })
                .collect::<Vec<_>>();
            // println!("found for index {}: {}", i, valid_directions.len());

            if valid_directions.len() > 1 {
                todo!("more than 1 valid direction");
            }

            if valid_directions.len() == 1 {
                let (found_root, direction) = valid_directions[0];

                // store the position of this probe
                let previous_value = directions.insert(i, (found_root, direction));

                if let Some(previous_value) = previous_value {
                    panic!(
                        "Found duplicate probe! {:?} vs old {:?}",
                        (found_root, direction),
                        previous_value
                    );
                }

                // store all newly found points
                for new_point in list {
                    // TODO: wtf?? this should be add, but it only works with sub
                    known_locations.insert(found_root - new_point.transform_from(direction));
                }
            }
        }

        iteration_count += 1;
        if iteration_count > input.len() {
            panic!("Too many iterations");
        }
    }

    (directions, known_locations)
}

fn compare_sets(
    known_set: &HashSet<Vector3>,
    other: &[Vector3],
    other_direction: FacingDirection,
) -> Option<Vector3> {
    // TODO: can try heapless indexmap here
    let mut found_offsets = HashMap::new();

    for &v in other {
        // first item is inversely transformed so it turns to the true direction
        let relative_location = v.transform_from(other_direction);

        for &real_probe_location in known_set {
            let expected_root_location = relative_location + real_probe_location;

            let entry = found_offsets.entry(expected_root_location).or_insert(0);
            *entry += 1;
        }
    }

    if false {
        let found_count = found_offsets.iter().map(|(_, &c)| c).max().unwrap();
        if found_count > 1 {
            println!("Found matches: {}", found_count);
        }
        if found_count >= 12 {
            for &v in other {
                println!("{:?} ({:?})", v, v.transform_from(other_direction));
            }
        }
    }

    // return first diff where 12 items match
    // TODO: may need to also ensure that something is in range (i forgot what)
    let mut iter = found_offsets
        .iter()
        .filter(|(_, &count)| count >= 12)
        .map(|(&diff, _)| diff);

    // assert there is only 1 match in debug builds
    debug_assert!(iter.clone().count() <= 1);

    iter.next()
}

#[test]
pub fn test_transform() {
    let point = Vector3(1, 2, 3);

    assert_eq!(point.transform_from(FacingDirection::default()), point);

    assert_eq!(
        point.transform_from(FacingDirection {
            invert_x: true,
            ..Default::default()
        }),
        Vector3(-1, 2, 3)
    );
    assert_eq!(
        point.transform_from(FacingDirection {
            invert_y: true,
            ..Default::default()
        }),
        Vector3(1, -2, 3)
    );
    assert_eq!(
        point.transform_from(FacingDirection {
            invert_z: true,
            ..Default::default()
        }),
        Vector3(1, 2, -3)
    );
}

#[test]
pub fn test_inverse_transform() {
    for direction in FacingDirection::iter_all() {
        let v1 = Vector3(1, 2, 3);
        let v2 = v1.transform_to(direction).transform_from(direction);
        assert_eq!(v1, v2);
    }
}
