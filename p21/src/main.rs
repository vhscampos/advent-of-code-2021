fn parse_input(handle: impl std::io::BufRead) -> (usize, usize) {
    let mut lines_iterator = handle.lines().map(|lr| lr.unwrap());
    let p1 = lines_iterator
        .next()
        .unwrap()
        .split_whitespace()
        .nth(4)
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let p2 = lines_iterator
        .next()
        .unwrap()
        .split_whitespace()
        .nth(4)
        .unwrap()
        .parse::<usize>()
        .unwrap();

    (p1, p2)
}

mod part1 {
    use std::iter::Cycle;
    use std::ops::RangeInclusive;
    struct DeterministicDie(Cycle<RangeInclusive<usize>>);

    const VICTORY_SCORE: u64 = 1000;

    impl DeterministicDie {
        fn new() -> Self {
            DeterministicDie((1..=100).cycle())
        }

        fn roll(&mut self) -> usize {
            self.0.next().unwrap()
        }
    }

    pub fn solve(mut p1: usize, mut p2: usize) {
        let mut die = DeterministicDie::new();
        let mut score_p1 = 0;
        let mut score_p2 = 0;
        let mut num_rolls = 0;
        p1 -= 1;
        p2 -= 1;

        loop {
            let mut steps = die.roll() + die.roll() + die.roll();
            num_rolls += 3;
            p1 += steps;
            p1 %= 10;
            score_p1 += p1 as u64 + 1;

            if score_p1 >= VICTORY_SCORE {
                break;
            }

            steps = die.roll() + die.roll() + die.roll();
            num_rolls += 3;
            p2 += steps;
            p2 %= 10;
            score_p2 += p2 as u64 + 1;

            if score_p2 >= VICTORY_SCORE {
                break;
            }
        }

        let result = num_rolls
            * if score_p1 > score_p2 {
                score_p2
            } else {
                score_p1
            };
        println!("Part 1 - Result: {}", result);
    }
}

mod part2 {
    use itertools::Itertools;
    use std::collections::HashMap;

    type DynProg = HashMap<(usize, usize, u64, u64), (u64, u64)>;

    const VICTORY_SCORE: u64 = 21;

    fn calc(p1: usize, p2: usize, score_p1: u64, score_p2: u64, d: &mut DynProg) -> (u64, u64) {
        let tuple = (p1, p2, score_p1, score_p2);
        if let Some(&value) = d.get(&tuple) {
            return value;
        }

        let mut subproblems_sum = (0, 0);

        let possible_steps = (1..=3)
            .cartesian_product(1..=3)
            .cartesian_product(1..=3)
            .map(|((x, y), z)| x + y + z)
            .collect::<Vec<usize>>();

        for step_p1 in &possible_steps {
            let new_p1 = (p1 + step_p1) % 10;
            let new_score_p1 = score_p1 + new_p1 as u64 + 1;

            if new_score_p1 >= VICTORY_SCORE {
                subproblems_sum.0 += 1;
                continue;
            }

            for step_p2 in &possible_steps {
                let new_p2 = (p2 + step_p2) % 10;
                let new_score_p2 = score_p2 + new_p2 as u64 + 1;

                if new_score_p2 >= VICTORY_SCORE {
                    subproblems_sum.1 += 1;
                } else {
                    let subproblems_result = calc(new_p1, new_p2, new_score_p1, new_score_p2, d);
                    subproblems_sum.0 += subproblems_result.0;
                    subproblems_sum.1 += subproblems_result.1;
                }
            }
        }

        d.insert(tuple, subproblems_sum);

        subproblems_sum
    }

    pub fn solve(p1: usize, p2: usize) {
        let mut d = DynProg::new();
        let (p1_wins, p2_wins) = calc(p1 - 1, p2 - 1, 0, 0, &mut d);
        println!("Part 2 - Result: {}", std::cmp::max(p1_wins, p2_wins));
    }
}

fn main() {
    let (p1, p2) = parse_input(std::io::stdin().lock());
    part1::solve(p1, p2);
    part2::solve(p1, p2);
}
