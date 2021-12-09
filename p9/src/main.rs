fn parse_input(handle: impl std::io::BufRead) -> Vec<Vec<u32>> {
    let mut grid = Vec::<Vec<u32>>::new();
    for line_result in handle.lines() {
        let mut v = Vec::<u32>::new();
        let line = line_result.unwrap();
        for element in line.chars() {
            let height = element as u32 - '0' as u32;
            v.push(height);
        }
        grid.push(v);
    }

    grid
}

mod common {
    pub fn is_low_point(grid: &Vec<Vec<u32>>, i: usize, j: usize) -> bool {
        let element = grid[i][j];
        (if i > 0 {
            element < grid[i - 1][j]
        } else {
            true
        }) && (if i < grid.len() - 1 {
            element < grid[i + 1][j]
        } else {
            true
        }) && (if j > 0 {
            element < grid[i][j - 1]
        } else {
            true
        }) && (if j < grid[0].len() - 1 {
            element < grid[i][j + 1]
        } else {
            true
        })
    }

    pub fn find_low_points(grid: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
        (0..grid.len())
            .flat_map(|i| {
                (0..grid[0].len()).filter_map(move |j| {
                    if is_low_point(grid, i, j) {
                        Some((i, j))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }
}

mod part1 {
    use crate::common::find_low_points;

    pub fn solve(grid: &Vec<Vec<u32>>) {
        let low_points = find_low_points(grid);
        let result = low_points
            .into_iter()
            .map(|(i, j)| grid[i][j] + 1)
            .sum::<u32>();

        println!("Part 1 - Result: {}", result);
    }
}

mod part2 {
    use crate::common::find_low_points;
    use bitvec::prelude::*;

    fn search(grid: &Vec<Vec<u32>>, i: usize, j: usize) -> usize {
        let row_len = grid[0].len();
        let col_len = grid.len();
        let index = |i, j| i * row_len + j;
        let mut visited = bitvec![0; grid.len() * grid[0].len()];
        let mut q = vec![(i, j)];
        let mut basin_size = 0;

        while let Some((i, j)) = q.pop() {
            let height = grid[i][j];
            if height == 9 || visited[index(i, j)] {
                continue;
            }

            basin_size += 1;
            visited.set(index(i, j), true);

            if i > 0 {
                q.push((i - 1, j));
            }
            if i < col_len - 1 {
                q.push((i + 1, j));
            }
            if j > 0 {
                q.push((i, j - 1));
            }
            if j < row_len - 1 {
                q.push((i, j + 1));
            }
        }

        basin_size
    }

    pub fn solve(grid: &Vec<Vec<u32>>) {
        let low_points = find_low_points(grid);

        let mut result_vec = low_points
            .into_iter()
            .map(|point| search(grid, point.0, point.1))
            .collect::<Vec<usize>>();

        result_vec.sort_unstable();

        let result = result_vec.into_iter().rev().take(3).product::<usize>();

        println!("Part 2 - Result: {}", result);
    }
}

fn main() {
    let grid = parse_input(std::io::stdin().lock());
    part1::solve(&grid);
    part2::solve(&grid);
}
