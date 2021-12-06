use aoc_lib::*;

aoc_setup!(Day6, sample 1: 5934, sample 2: 26984457539, part 1: 360268);

pub struct Day6;

impl AdventOfCode for Day6 {
    type Input = Vec<usize>; // can be u8
    type Output = u64;

    fn parse_input(s: &str) -> Self::Input {
        s.split(',').map(|l| l.parse().unwrap()).collect()
    }

    fn solve_1(input: &Self::Input) -> Self::Output {
        solve_smart::<80>(input)
    }

    fn solve_2(input: &Self::Input) -> Self::Output {
        solve_smart::<256>(input)
    }
}

pub fn solve_naive<const DAYS: usize>(input: &[usize]) -> u64 {
    let mut vec: Vec<usize> = input.iter().copied().collect();

    for _day in 0..DAYS {
        let start_len = vec.len();
        for i in 0..start_len {
            if vec[i] == 0 {
                vec[i] = 6;
                vec.push(8);
            } else {
                vec[i] -= 1;
            }
        }
    }

    vec.len() as u64
}

pub fn solve_smart<const DAYS: usize>(input: &[usize]) -> u64 {
    let map = generate_map(DAYS);
    // println!("map: {:?}", map);

    // for every starting fish, calculate the amount of offspring they generate
    let birthed_offspring: u64 = input
        .iter()
        .map(|&fish| (map[8 - fish + DAYS])) // lower number means born earlier, means more fish
        .sum();

    birthed_offspring // + (input.len() as u64)
}

fn generate_map(days: usize) -> Vec<u64> {
    // this map contains the number of offspring that are resultant from a fish born at time 0
    // TODO: use stack array instead of vector!
    // TODO: can build this map inside a const fn!!
    let mut map = vec![0u64; (days + 8) as usize]; // + 7? + 0?

    for day in 0..map.len() {
        let added_fish = get_fish_count_from_map_day(&map, day);
        map[day] = added_fish;
        // println!("day: {}, added_fish: {}", day, added_fish);
    }

    map
}

// rename me
// also const I think
fn get_fish_count_from_map_day(map: &[u64], day: usize) -> u64 {
    // always start with 1 fish
    let mut added_fish = 1;

    // look at all days that spawn a fish, see how many fish result from this day
    for offspring_day in (9..=day).step_by(7) {
        let remaining_days = day - offspring_day;
        added_fish += map[remaining_days];
    }

    added_fish
}

#[test]
pub fn test_map() {
    let map = generate_map(30 - 8);

    assert_eq!(
        map,
        &[
            1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 3, 3, 4, 4, 4, 4, 4, 5, 5, 7, 7, 8, 8,
            8
        ]
    );
}

#[test]
pub fn test_solve_naive() {
    assert_eq!(5, solve_naive::<1>(&[3, 4, 3, 1, 2]));
    assert_eq!(6, solve_naive::<2>(&[3, 4, 3, 1, 2]));
    assert_eq!(7, solve_naive::<3>(&[3, 4, 3, 1, 2]));
    assert_eq!(9, solve_naive::<4>(&[3, 4, 3, 1, 2]));
    assert_eq!(10, solve_naive::<5>(&[3, 4, 3, 1, 2]));
    assert_eq!(10, solve_naive::<6>(&[3, 4, 3, 1, 2]));
    assert_eq!(10, solve_naive::<7>(&[3, 4, 3, 1, 2]));
    assert_eq!(10, solve_naive::<8>(&[3, 4, 3, 1, 2]));
    assert_eq!(11, solve_naive::<9>(&[3, 4, 3, 1, 2]));
    assert_eq!(12, solve_naive::<10>(&[3, 4, 3, 1, 2]));
    assert_eq!(15, solve_naive::<11>(&[3, 4, 3, 1, 2]));
}

#[test]
pub fn test_solve_smart() {
    assert_eq!(5, solve_smart::<1>(&[3, 4, 3, 1, 2]));
    assert_eq!(6, solve_smart::<2>(&[3, 4, 3, 1, 2]));
    assert_eq!(7, solve_smart::<3>(&[3, 4, 3, 1, 2]));
    assert_eq!(9, solve_smart::<4>(&[3, 4, 3, 1, 2]));
    assert_eq!(10, solve_smart::<5>(&[3, 4, 3, 1, 2]));
    assert_eq!(10, solve_smart::<6>(&[3, 4, 3, 1, 2]));
    assert_eq!(10, solve_smart::<7>(&[3, 4, 3, 1, 2]));
    assert_eq!(10, solve_smart::<8>(&[3, 4, 3, 1, 2]));
    assert_eq!(11, solve_smart::<9>(&[3, 4, 3, 1, 2])); // fails
    assert_eq!(12, solve_smart::<10>(&[3, 4, 3, 1, 2]));
    assert_eq!(15, solve_smart::<11>(&[3, 4, 3, 1, 2]));
}
