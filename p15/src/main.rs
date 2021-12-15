mod common {
    use std::collections::BinaryHeap;

    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    struct Element {
        cost: i32,
        i: usize,
        j: usize,
    }

    impl Element {
        fn new(cost: i32, i: usize, j: usize) -> Self {
            Element { cost, i, j }
        }
    }

    pub fn solve(grid: &[Vec<u32>]) -> u32 {
        let rows = grid.len();
        let cols = grid[0].len();

        let mut result = 0;
        let mut q = BinaryHeap::<Element>::new();
        let mut visited = vec![vec![false; cols]; rows];
        q.push(Element::new(0, 0, 0));

        while let Some(elem) = q.pop() {
            let Element { cost, i, j } = elem;

            if visited[i][j] {
                continue;
            }
            visited[i][j] = true;

            if i == rows - 1 && j == cols - 1 {
                result = (-cost) as u32;
                break;
            }

            if i > 0 {
                let local_cost = -(grid[i - 1][j] as i32);
                q.push(Element::new(cost + local_cost, i - 1, j));
            }
            if i + 1 < rows {
                let local_cost = -(grid[i + 1][j] as i32);
                q.push(Element::new(cost + local_cost, i + 1, j));
            }
            if j > 0 {
                let local_cost = -(grid[i][j - 1] as i32);
                q.push(Element::new(cost + local_cost, i, j - 1));
            }
            if j + 1 < cols {
                let local_cost = -(grid[i][j + 1] as i32);
                q.push(Element::new(cost + local_cost, i, j + 1));
            }
        }

        result
    }
}

mod part1 {
    pub fn parse_input(handle: impl std::io::BufRead) -> Vec<Vec<u32>> {
        handle
            .lines()
            .map(|line_result| {
                let line = line_result.unwrap();
                line.chars().map(|c| c as u32 - '0' as u32).collect()
            })
            .collect()
    }

    pub fn solve(grid: &[Vec<u32>]) {
        use crate::common;

        let result = common::solve(grid);
        println!("Part 1 - Result: {}", result);
    }
}

mod part2 {
    pub fn parse_input(handle: impl std::io::BufRead) -> Vec<Vec<u32>> {
        let initial_grid = handle
            .lines()
            .map(|line_result| {
                let line = line_result.unwrap();
                line.chars()
                    .map(|c| c as u32 - '0' as u32)
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<_>>();

        let initial_rows = initial_grid.len();
        let initial_cols = initial_grid[0].len();
        let rows = initial_rows * 5;
        let cols = initial_cols * 5;

        let mut grid = vec![vec![0; cols]; rows];

        for i in 0..rows {
            for j in 0..cols {
                let norm_i = i % initial_rows;
                let norm_j = j % initial_cols;
                let tiles_i = i / initial_rows;
                let tiles_j = j / initial_cols;
                let mut value = initial_grid[norm_i][norm_j] + tiles_i as u32;
                if value > 9 {
                    value -= 9;
                }
                value += tiles_j as u32;
                if value > 9 {
                    value -= 9;
                }
                grid[i][j] = value;
            }
        }

        grid
    }

    pub fn solve(grid: &[Vec<u32>]) {
        use crate::common;

        let result = common::solve(grid);
        println!("Part 2 - Result: {}", result);
    }
}

fn main() {
    // let grid = part1::parse_input(std::io::stdin().lock());
    // part1::solve(&grid);
    let grid = part2::parse_input(std::io::stdin().lock());
    part2::solve(&grid);
}
