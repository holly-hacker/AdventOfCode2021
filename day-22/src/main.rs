use aoc_lib::*;
use tinyvec::{array_vec, ArrayVec, TinyVec};

aoc_setup!(Day22, sample 1: 474140, sample 2: 2758514936282235, part 1: 568000, part 2: 1177411289280259);

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    state: bool,
    region: Region3D,
}

impl Instruction {
    pub fn parse(line: &str) -> Self {
        let (state, line) = line.split_once(' ').unwrap();

        Self {
            state: state.len() == 2,
            region: Region3D::parse(line),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Region3D {
    range_x: Region1D,
    range_y: Region1D,
    range_z: Region1D,
}

impl Region3D {
    pub fn parse(text: &str) -> Self {
        let mut parts = text.split(',');
        Self {
            range_x: Region1D::parse(&(parts.next().unwrap()[2..])),
            range_y: Region1D::parse(&(parts.next().unwrap()[2..])),
            range_z: Region1D::parse(&(parts.next().unwrap()[2..])),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.range_x.is_empty() || self.range_y.is_empty() || self.range_z.is_empty()
    }

    #[must_use]
    pub fn clamp_to_init(self) -> Self {
        Self {
            range_x: self.range_x.clamp_to_init(),
            range_y: self.range_y.clamp_to_init(),
            range_z: self.range_z.clamp_to_init(),
        }
    }

    // can return up to 3^3 values, if split in the center
    // for performance reasons, only store up to 9 items on the stack (handles common cases)
    // this can be increased to 27 but that seems worse for performance
    pub fn split_excluding_self(self, other: &Self) -> TinyVec<[Self; 9]> {
        self.range_x
            .split(other.range_x) // split x range
            .into_iter()
            .map(|new_region_x| Self {
                range_x: new_region_x,
                ..self
            }) // turn split x ranges into
            .flat_map(|x_chunk| {
                self.range_y
                    .split(other.range_y)
                    .into_iter()
                    .map(move |new_region_y| Self {
                        range_y: new_region_y,
                        ..x_chunk
                    })
                    .flat_map(|y_chunk| {
                        self.range_z
                            .split(other.range_z)
                            .into_iter()
                            .map(move |new_region_z| Self {
                                range_z: new_region_z,
                                ..y_chunk
                            })
                    })
            })
            .filter(|region| !region.fits_inside(other))
            .collect()
    }

    pub fn fits_inside(&self, other: &Self) -> bool {
        self.range_x.fits_inside(other.range_x)
            && self.range_y.fits_inside(other.range_y)
            && self.range_z.fits_inside(other.range_z)
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.range_x.intersects(other.range_x)
            && self.range_y.intersects(other.range_y)
            && self.range_z.intersects(other.range_z)
    }

    pub fn size(&self) -> usize {
        self.range_x.size() * self.range_y.size() * self.range_z.size()
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Region1D(i32, i32);

impl Region1D {
    pub fn parse(text: &str) -> Self {
        let (a, b) = text.split_once("..").unwrap();
        let pair: (i32, i32) = (a.parse().unwrap(), b.parse().unwrap());
        Self(pair.0.min(pair.1), pair.0.max(pair.1))
    }

    pub fn is_empty(&self) -> bool {
        self.0 > self.1
    }

    #[must_use]
    pub fn clamp_to_init(self) -> Self {
        Self(self.0.max(-50), self.1.min(50))
    }

    /// Splits the Region1D into pieces. One of the regions will perfectly cover the other region.
    pub fn split(self, other: Self) -> ArrayVec<[Self; 3]> {
        if other.0 <= self.0 && other.1 >= self.1 {
            // other completely covers us (ie. we are contained in them)
            return array_vec![[Self; 3] => self];
        }

        match (self.contains(other.0), self.contains(other.1)) {
            (false, false) => {
                // no overlap
                array_vec![[Self; 3] => self]
            }
            (true, true) => {
                // other is definitely inside
                array_vec![[Self; 3] => Self(self.0, other.0 - 1), other, Self(other.1 + 1, self.1)]
            }
            (false, true) => {
                // other covers left edge
                array_vec![[Self; 3] => Self(self.0, other.1), Self(other.1 + 1, self.1)]
            }
            (true, false) => {
                // other covers right edge
                array_vec![[Self; 3] => Self(self.0, other.0 - 1), Self(other.0, self.1)]
            }
        }
    }

    pub fn fits_inside(&self, other: Self) -> bool {
        self.0 >= other.0 && self.1 <= other.1
    }

    pub fn intersects(&self, other: Self) -> bool {
        self.0.max(other.0) <= self.1.min(other.1)
    }

    fn contains(&self, other: i32) -> bool {
        (self.0..=self.1).contains(&other)
    }

    pub fn size(&self) -> usize {
        (self.1 - self.0 + 1) as usize
    }
}

impl std::fmt::Debug for Region1D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}..{})", self.0, self.1)
    }
}

#[derive(Default)]
pub struct RegionCollection {
    regions: Vec<Region3D>,
}

impl RegionCollection {
    pub fn add_region(&mut self, to_add: Region3D) {
        self.segment_regions_to_fit(to_add, true)
    }

    pub fn remove_region(&mut self, to_remove: Region3D) {
        self.segment_regions_to_fit(to_remove, false)
    }

    fn segment_regions_to_fit(&mut self, to_fit: Region3D, add_afterwards: bool) {
        // for each region that intersects with the region to be fit: take it out, split it up, put the pieces back in
        for i in (0..self.regions.len()).rev() {
            if self.regions[i].intersects(&to_fit) {
                let new_regions = self.regions[i].split_excluding_self(&to_fit);

                if new_regions.is_empty() {
                    self.regions.remove(i);
                } else {
                    // for performance reasons, insert the first new region in the old region's position to prevent having to shift all items after it
                    self.regions[i] = new_regions[0];
                    self.regions.extend_from_slice(&new_regions.as_slice()[1..]);
                }
            }
        }

        if add_afterwards {
            self.regions.push(to_fit);
        }
    }

    pub fn total_size(&self) -> usize {
        self.regions.iter().map(|r| r.size()).sum()
    }
}

pub struct Day22;

impl AdventOfCode for Day22 {
    type Input = Vec<Instruction>;
    type Output = usize;

    fn parse_input(s: &str) -> Self::Input {
        s.lines().map(Instruction::parse).collect()
    }

    fn solve_1(input: &Self::Input) -> Self::Output {
        solve(input, false)
    }

    fn solve_2(input: &Self::Input) -> Self::Output {
        solve(input, true)
    }
}

fn solve(input: &[Instruction], solve_outside_init: bool) -> usize {
    let mut regions = RegionCollection::default();

    for &instruction in input {
        let region = if !solve_outside_init {
            instruction.region.clamp_to_init()
        } else {
            instruction.region
        };

        if !region.is_empty() {
            if instruction.state {
                regions.add_region(region);
            } else {
                regions.remove_region(region);
            }
        }
    }

    regions.total_size()
}

#[test]
fn test_region_size() {
    assert_eq!(9, Region3D::parse("x=0..2,y=3..1,z=1..1").size());
    assert_eq!(198, Region3D::parse("x=10..20,y=-1..1,z=0..5").size());
}

#[test]
fn test_split_region1d() {
    assert_eq!(
        [Region1D(5, 10)],
        Region1D(5, 10).split(Region1D(20, 30)).as_slice(),
        "outside"
    );

    assert_eq!(
        [Region1D(5, 10)],
        Region1D(5, 10).split(Region1D(5, 10)).as_slice(),
        "covers completely"
    );

    assert_eq!(
        [Region1D(5, 7), Region1D(8, 10)],
        Region1D(5, 10).split(Region1D(0, 7)).as_slice(),
        "covers left"
    );

    assert_eq!(
        [Region1D(5, 7), Region1D(8, 10)],
        Region1D(5, 10).split(Region1D(8, 20)).as_slice(),
        "covers right"
    );

    assert_eq!(
        [Region1D(5, 6), Region1D(7, 8), Region1D(9, 10)],
        Region1D(5, 10).split(Region1D(7, 8)).as_slice(),
        "covers center"
    );
}

#[test]
fn test_split_region3d() {
    assert_eq!(
        TinyVec::default(),
        Region3D::parse("x=2..7,y=2..7,z=2..7")
            .split_excluding_self(&Region3D::parse("x=2..7,y=2..7,z=2..7")),
        "covers completely",
    );

    assert_eq!(
        [Region3D::parse("x=6..7,y=2..7,z=2..7")],
        Region3D::parse("x=2..7,y=2..7,z=2..7")
            .split_excluding_self(&Region3D::parse("x=0..5,y=0..9,z=0..9"))
            .as_slice(),
        "covers left side",
    );

    assert_eq!(
        [
            Region3D::parse("x=2..7,y=2..7,z=2..3"),
            Region3D::parse("x=2..7,y=2..7,z=6..7")
        ],
        Region3D::parse("x=2..7,y=2..7,z=2..7")
            .split_excluding_self(&Region3D::parse("x=0..9,y=0..9,z=4..5"))
            .as_slice(),
        "covers middle",
    );

    assert_eq!(
        26,
        Region3D::parse("x=2..7,y=2..7,z=2..7")
            .split_excluding_self(&Region3D::parse("x=4..5,y=4..5,z=4..5"))
            .len(),
        "covers center",
    );
}

#[test]
fn part_1_small_sample() {
    const TEST:&str = "on x=10..12,y=10..12,z=10..12\non x=11..13,y=11..13,z=11..13\noff x=9..11,y=9..11,z=9..11\non x=10..10,y=10..10,z=10..10";
    let input = Day22::parse_input(TEST);
    let solution = Day22::solve_1(&input);
    assert_eq!(39, solution);
}
