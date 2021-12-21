use aoc_lib::*;

aoc_setup!(Day21, sample 1: 739785, sample 2: 444356092776315, part 1: 675024, part 2: 570239341223618);

pub struct Day21;

impl AdventOfCode for Day21 {
    type Input = (usize, usize);
    type Output = usize;

    fn parse_input(s: &str) -> Self::Input {
        // input always has same length :3c
        let b = s.as_bytes();
        ((b[28] - b'0') as usize, (b[b.len() - 1] - b'0') as usize)
    }

    // NOTE: I could precalculate a table for this since there are only 100 options
    fn solve_1(input: &Self::Input) -> Self::Output {
        let (mut die, mut die_roll_count) = (0, 0);
        let (mut score1, mut score2) = (0, 0);
        let (mut pos1, mut pos2) = input;
        pos1 -= 1;
        pos2 -= 1;

        fn roll_die(idx: usize) -> (usize, usize) {
            match idx {
                98 => (99 + 100 + 1, (idx + 3) % 100),
                99 => (100 + 1 + 2, (idx + 3) % 100),
                _ => ((idx + 2) * 3, (idx + 3) % 100),
            }
        }

        loop {
            let (roll1, new_die) = roll_die(die);
            die = new_die;
            die_roll_count += 1;
            pos1 = (pos1 + roll1) % 10;
            score1 += pos1 + 1;

            if score1 >= 1000 {
                return score2 * die_roll_count * 3; // TODO: try to just increase by 1 and mul by 3 near the end
            }

            let (roll2, new_die) = roll_die(die);
            die = new_die;
            die_roll_count += 1;
            pos2 = (pos2 + roll2) % 10;
            score2 += pos2 + 1;

            if score2 >= 1000 {
                return score1 * die_roll_count * 3;
            }
        }
    }

    fn solve_2(input: &Self::Input) -> Self::Output {
        #[derive(Clone, Copy)]
        struct State {
            score1: usize,
            score2: usize,
            pos1: usize,
            pos2: usize,
        }

        fn run(state: State) -> (usize, usize) {
            let possibility_map = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

            let mut total = (0, 0);

            for (roll, count1) in possibility_map {
                let mut new_state = state;
                new_state.pos1 = (state.pos1 + roll - 1) % 10 + 1;
                new_state.score1 = state.score1 + new_state.pos1;

                if new_state.score1 >= 21 {
                    // player 1 wins, count the number of universes where this happens
                    total.0 += count1;
                } else {
                    // roll for player 2
                    for (roll, count2) in possibility_map {
                        new_state.pos2 = (state.pos2 + roll - 1) % 10 + 1;
                        new_state.score2 = state.score2 + new_state.pos2;

                        if new_state.score2 >= 21 {
                            // player 2 wins, count the number of universes where this happens
                            total.1 += count1 * count2;
                        } else {
                            // neither player won this round, so calculate the possibilities from this point
                            let (p1, p2) = run(new_state);

                            let total_count = count1 * count2;
                            total.0 += p1 * total_count;
                            total.1 += p2 * total_count;
                        }
                    }
                }
            }

            total
        }

        let results = run(State {
            score1: 0,
            score2: 0,
            pos1: input.0,
            pos2: input.1,
        });

        results.0.max(results.1)
    }
}
