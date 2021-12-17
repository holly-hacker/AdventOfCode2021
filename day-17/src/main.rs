use std::ops::Range;

use aoc_lib::*;

aoc_setup!(Day17, sample 1: 45, sample 2: 112, part 1: 5565, part 2: 2118);

pub struct Vec2 {
    x: isize,
    y: isize,
}

impl Vec2 {
    pub fn new(x: isize, y: isize) -> Vec2 {
        Vec2 { x, y }
    }
}

pub struct Input {
    pub start: Vec2,
    pub end: Vec2,
}

impl Input {
    pub fn x(&self) -> isize {
        self.start.x
    }

    pub fn y(&self) -> isize {
        self.start.y
    }

    pub fn x2(&self) -> isize {
        self.end.x
    }

    pub fn y2(&self) -> isize {
        self.end.y
    }

    pub fn height(&self) -> usize {
        (self.end.y - self.start.y).abs() as usize
    }
    pub fn width(&self) -> usize {
        (self.end.y - self.start.y).abs() as usize
    }

    pub fn x_range(&self) -> Range<isize> {
        self.start.x..(self.end.x + 1)
    }
    pub fn y_range(&self) -> Range<isize> {
        self.start.y..(self.end.y + 1)
    }
}
pub struct Day17;

impl AdventOfCode for Day17 {
    type Input = Input;
    type Output = usize;

    fn parse_input(s: &str) -> Self::Input {
        let (x_range, y_range) = s[15..].split_once(", y=").unwrap();
        let ((x1, x2), (y1, y2)) = (
            x_range.split_once("..").unwrap(),
            y_range.split_once("..").unwrap(),
        );
        let (x1, x2, y1, y2): (isize, isize, isize, isize) = (
            x1.parse().unwrap(),
            x2.parse().unwrap(),
            y1.parse().unwrap(),
            y2.parse().unwrap(),
        );

        Input {
            start: Vec2::new(x1.min(x2), y1.max(y2)),
            end: Vec2::new(x1.max(x2), y1.min(y2)),
        }
    }

    fn solve_1(input: &Self::Input) -> Self::Output {
        debug_assert!(
            !(input.x() < 0 && input.x2() > 0),
            "target area should not cross x axis, solution would be infinity"
        );

        // 1. if you have a X velocity of N, the final X coordinate will be N(N+1)/2 (the triangle number) and it will be reached after about N steps
        // we can probably that each valid y velocity has a valid x velocity
        // 2. for any velocity N where N>=0, you will always pass at Y=0 after 2N+1 steps.
        // the Y velocity should thus be roughly equal to the lowest Y coordinate of the target area?
        // the example has Y=-10 and results in velocity 9
        debug_assert!(input.y2() < 0, "lowest y >= 0 is not implemented");

        let max_velocity_y = -input.y2() - 1;
        debug_assert!(max_velocity_y >= 0);
        triangle_number(max_velocity_y as usize)
    }

    fn solve_2(input: &Self::Input) -> Self::Output {
        debug_assert!(input.y() < 0, "assuming y < 0");
        debug_assert!(input.x() > 0, "assuming x > 0");
        // we need to find all shots that land in the target area. best start with bounds

        // start by finding all possible X and Y velocities
        let min_y_velocity = input.y2();
        let max_y_velocity = -input.y2() - 1;

        let min_x_velocity = inverse_triangle_number(
            INVERSE_TRIANGLE_LOOKUP
                .into_iter()
                .find(|&x| x >= input.x() as usize)
                .unwrap(),
        ) as usize;
        let max_x_velocity = input.x2() as usize;

        (min_y_velocity..=max_y_velocity)
            .map(|y_velocity| {
                let y_step_range = calculate_y_steps(input, y_velocity);

                (min_x_velocity..=max_x_velocity)
                    .filter(|&x_velocity| {
                        y_step_range.clone().any(|step| {
                            // try to find valid velocity for this step
                            input
                                .x_range()
                                .contains(&(get_x_position_at_step(x_velocity, step) as isize))
                        })
                    })
                    .count()
            })
            .sum()
    }
}

fn get_x_position_at_step(x_velocity: usize, step: usize) -> usize {
    let final_x_position = triangle_number(x_velocity);
    if step >= x_velocity {
        final_x_position
    } else {
        final_x_position - triangle_number(x_velocity - step)
    }
}

/// Calculate the step range in which the given y start velocity is in the target zone
fn calculate_y_steps(input: &Input, n: isize) -> Range<usize> {
    let steps_at_y0 = if n <= 0 { 0 } else { (n as usize) * 2 + 1 };
    let velocity_at_y0 = if n <= 0 { n } else { -n - 1 };

    let mut velocity = velocity_at_y0;
    let mut pos = 0isize;
    let mut inside_before = false;
    let mut found_start_step = Default::default();
    let mut current_step = steps_at_y0;

    loop {
        let inside_now = pos <= input.y() && pos >= input.y2();

        if inside_now && !inside_before {
            // entered the target area
            found_start_step = current_step;
            inside_before = true;
        } else if !inside_now && inside_before {
            // left the target area
            return found_start_step..current_step;
        } else if pos < input.y2() {
            return 0..0;
        }

        pos += velocity;
        velocity -= 1;
        current_step += 1;
    }
}

const fn triangle_number(n: usize) -> usize {
    (n * (n + 1)) / 2
}

const INVERSE_TRIANGLE_LOOKUP: [usize; 20] = [
    triangle_number(0),
    triangle_number(1),
    triangle_number(2),
    triangle_number(3),
    triangle_number(4),
    triangle_number(5),
    triangle_number(6),
    triangle_number(7),
    triangle_number(8),
    triangle_number(9),
    triangle_number(10),
    triangle_number(11),
    triangle_number(12),
    triangle_number(13),
    triangle_number(14),
    triangle_number(15),
    triangle_number(16),
    triangle_number(17),
    triangle_number(18),
    triangle_number(19),
];

fn inverse_triangle_number(n: usize) -> usize {
    // TODO: what if not in table?
    INVERSE_TRIANGLE_LOOKUP
        .into_iter()
        .position(|x| x == n)
        .unwrap()
}
