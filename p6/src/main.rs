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

fn part1(start: &[i64]) {
    let mut dp = DynProg::new(80);
    let result = dp.calculate(start);
    println!("Part 1 - Result: {}", result);
}

fn part2(start: &[i64]) {
    let mut dp = DynProg::new(256);
    let result = dp.calculate(start);
    println!("Part 2 - Result: {}", result);
}

fn main() {
    let list = parse_input(std::io::stdin().lock());
    part1(&list);
    part2(&list);
}
