fn parse_input(handle: impl std::io::BufRead) -> Vec<i64> {
    handle
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

// First solution: dynamic programming
// It is clearly not the best solution, but it was my original working solution
// and took quite some time to nail, thus I'm keeping it here.
// Complexity is O(n+d^2) where n is the input list size and d is the number of
// days simulated. Space complexity is O(d).
struct DynProg {
    table: Vec<i64>,
    n: i32,
}

impl DynProg {
    fn new(n: usize) -> DynProg {
        DynProg {
            table: vec![-1; n],
            n: n as i32,
        }
    }

    fn calculate(&mut self, list: &[i64]) -> i64 {
        list.iter()
            .map(|&entry| self.calc_internal(self.n - entry as i32))
            .sum()
    }

    fn calc_internal(&mut self, days: i32) -> i64 {
        if days <= 0 {
            return 1;
        }
        if self.table[days as usize] != -1 {
            return self.table[days as usize];
        }

        let mut result = 1;

        result += (0..)
            .step_by(7)
            .take_while(|i| days - i > 0)
            .map(|i| self.calc_internal(days - i - 9))
            .sum::<i64>();

        self.table[days as usize] = result;
        result
    }
}

// The actual best solution using offspring cycle counters.
// Complexity is O(n+d) where n is the input list size and
// d is the number of days simulated.
fn counting_solution(start: &[i64], days: i32) -> i64 {
    let mut counter = [0i64; 9];

    start.iter().for_each(|&x| counter[x as usize] += 1);

    for _ in 0..days {
        let number_offspring = counter[0];
        (0..8).for_each(|i| counter[i] = counter[i + 1]);
        counter[6] += number_offspring;
        counter[8] = number_offspring;
    }

    counter.iter().sum::<i64>()
}

fn part1(start: &[i64]) {
    let result = counting_solution(start, 80);
    println!("Part 1 - Result: {}", result);
}

fn part2(start: &[i64]) {
    let result = counting_solution(start, 256);
    println!("Part 2 - Result: {}", result);
}

fn main() {
    let list = parse_input(std::io::stdin().lock());
    part1(&list);
    part2(&list);
}
