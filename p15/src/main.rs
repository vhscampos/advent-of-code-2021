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

    pub fn parse_input(handle: impl std::io::BufRead) -> Vec<Vec<u32>> {
        handle
            .lines()
            .map(|line_result| {
                let line = line_result.unwrap();
                line.chars().map(|c| c as u32 - '0' as u32).collect()
            })
            .collect()
    }

    pub fn solve(
        grid: &[Vec<u32>],
        rows: usize,
        cols: usize,
        get_grid_value: fn(&[Vec<u32>], usize, usize) -> u32,
    ) -> u32 {
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
                let local_cost = -(get_grid_value(grid, i - 1, j) as i32);
                q.push(Element::new(cost + local_cost, i - 1, j));
            }
            if i + 1 < rows {
                let local_cost = -(get_grid_value(grid, i + 1, j) as i32);
                q.push(Element::new(cost + local_cost, i + 1, j));
            }
            if j > 0 {
                let local_cost = -(get_grid_value(grid, i, j - 1) as i32);
                q.push(Element::new(cost + local_cost, i, j - 1));
            }
            if j + 1 < cols {
                let local_cost = -(get_grid_value(grid, i, j + 1) as i32);
                q.push(Element::new(cost + local_cost, i, j + 1));
            }
        }

        result
    }
}

mod part1 {
    fn get_grid_value(grid: &[Vec<u32>], i: usize, j: usize) -> u32 {
        grid[i][j]
    }

    pub fn solve(grid: &[Vec<u32>]) {
        use crate::common;

        let result = common::solve(grid, grid.len(), grid[0].len(), get_grid_value);
        println!("Part 1 - Result: {}", result);
    }
}

mod part2 {
    fn get_grid_value(grid: &[Vec<u32>], i: usize, j: usize) -> u32 {
        let norm_i = i % grid.len();
        let norm_j = j % grid[0].len();
        let tiles_i = i / grid.len();
        let tiles_j = j / grid[0].len();
        let mut value = grid[norm_i][norm_j] + tiles_i as u32;
        if value > 9 {
            value -= 9;
        }
        value += tiles_j as u32;
        if value > 9 {
            value -= 9;
        }
        value
    }

    pub fn solve(grid: &[Vec<u32>]) {
        use crate::common;

        let result = common::solve(grid, grid.len() * 5, grid[0].len() * 5, get_grid_value);
        println!("Part 2 - Result: {}", result);
    }
}

fn main() {
    let grid = common::parse_input(std::io::stdin().lock());
    part1::solve(&grid);
    part2::solve(&grid);
}
