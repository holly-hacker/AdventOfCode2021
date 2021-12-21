use aoc_lib::*;

aoc_setup!(Day21, sample 1: 739785, sample 2: 444356092776315, part 1: 675024);

struct DeterministicDie {
    index: usize,
    roll_count: usize,
}

impl Default for DeterministicDie {
    fn default() -> Self {
        Self {
            index: 0,
            roll_count: 0,
        }
    }
}

impl Iterator for DeterministicDie {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.index = self.index % 100 + 1;
        self.roll_count += 1;
        Some(self.index)
    }
}

pub struct Day21;

impl AdventOfCode for Day21 {
    type Input = (usize, usize);
    type Output = usize;

    fn parse_input(s: &str) -> Self::Input {
        let mut lines = s.lines();
        (
            (lines.next().unwrap().bytes().last().unwrap() - b'0') as usize,
            (lines.next().unwrap().bytes().last().unwrap() - b'0') as usize,
        )
    }

    fn solve_1(input: &Self::Input) -> Self::Output {
        let die = &mut DeterministicDie::default();
        let (mut score1, mut score2) = (0, 0);
        let (mut pos1, mut pos2) = input;

        loop {
            let roll1 = die.take(3).sum::<usize>();
            pos1 = (pos1 + roll1 - 1) % 10 + 1;
            score1 += pos1;

            if score1 >= 1000 {
                return score2 * die.roll_count;
            }

            let roll2 = die.take(3).sum::<usize>();
            pos2 = (pos2 + roll2 - 1) % 10 + 1;
            score2 += pos2;

            if score2 >= 1000 {
                return score1 * die.roll_count;
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

                            total.0 += p1 * count1 * count2;
                            total.1 += p2 * count1 * count2;
                        }
                    }
                }
            }

            return total;
        }

        let results = run(State {
            score1: 0,
            score2: 0,
            pos1: input.0,
            pos2: input.1,
        });

        dbg!(results);

        results.0.max(results.1)
    }
}
