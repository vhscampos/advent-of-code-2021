fn parse_input(handle: impl std::io::BufRead) -> Vec<String> {
    handle
        .lines()
        .collect::<std::io::Result<Vec<String>>>()
        .unwrap()
}

mod common {
    pub fn find_first_closing_delim(line: &str) -> Option<char> {
        let mut s = Vec::<char>::with_capacity(line.len());

        let lala = |c: char, s: &mut Vec<char>| {
            if let Some(top) = s.pop() {
                return match top {
                    '(' => c == ')',
                    '[' => c == ']',
                    '{' => c == '}',
                    '<' => c == '>',
                    _ => panic!(),
                };
            }
            false
        };

        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => s.push(c),
                ')' | ']' | '}' | '>' => {
                    if !lala(c, &mut s) {
                        return Some(c);
                    }
                }
                _ => panic!(),
            }
        }

        None
    }

    pub fn is_corrupted(s: &str) -> bool {
        find_first_closing_delim(s).is_some()
    }
}

mod part1 {
    use crate::common::*;

    fn score_table(c: char) -> u32 {
        match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!(),
        }
    }

    pub fn solve(line_list: &[String]) {
        let mut score = 0;
        for line in line_list {
            let result = find_first_closing_delim(line);
            if let Some(c) = result {
                score += score_table(c);
            }
        }
        println!("Part 1 - Result: {}", score);
    }
}

mod part2 {
    use crate::common::*;

    fn find_score(line: &str) -> u64 {
        let mut s = Vec::<char>::with_capacity(line.len());

        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => s.push(c),
                _ => {
                    s.pop().unwrap();
                }
            }
        }

        let mut score = 0;
        for c in s.into_iter().rev() {
            score *= 5;
            score += match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!(),
            };
        }

        score
    }

    pub fn solve(line_list: &[String]) {
        let list = line_list
            .iter()
            .filter(|s| !is_corrupted(s))
            .collect::<Vec<&String>>();

        let mut score_list = list
            .into_iter()
            .map(|s| find_score(s))
            .collect::<Vec<u64>>();
        score_list.sort_unstable();
        let middle_score = score_list[score_list.len() / 2];
        println!("Part 2 - Result: {}", middle_score);
    }
}

fn main() {
    let input = parse_input(std::io::stdin().lock());
    part1::solve(&input);
    part2::solve(&input);
}
