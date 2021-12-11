fn parse_input(handle: impl std::io::BufRead) -> Vec<Vec<u32>> {
    handle
        .lines()
        .map(|l| l.unwrap().chars().map(|c| c as u32 - '0' as u32).collect())
        .collect()
}

fn try_flash(grid: &mut Vec<Vec<u32>>, flashes: &mut Vec<Vec<bool>>, i: usize, j: usize) -> u32 {
    if flashes[i][j] {
        return 0;
    }
    let mut counter = 1;
    flashes[i][j] = true;

    for k in i.saturating_sub(1)..=std::cmp::min(i + 1, grid.len() - 1) {
        for l in j.saturating_sub(1)..=std::cmp::min(j + 1, grid[0].len() - 1) {
            if k == i && l == j {
                continue;
            }
            grid[k][l] += 1;

            if grid[k][l] > 9 {
                counter += try_flash(grid, flashes, k, l);
            }
        }
    }

    counter
}

fn solve(mut grid: Vec<Vec<u32>>, is_part2: bool) {
    let row_len = grid[0].len();
    let col_len = grid.len();
    let mut counter = 0;
    let mut steps = 0;

    loop {
        let mut flashes = vec![vec![false; row_len]; col_len];

        grid.iter_mut().flatten().for_each(|x| *x += 1);

        for i in 0..col_len {
            for j in 0..row_len {
                if grid[i][j] > 9 {
                    counter += try_flash(&mut grid, &mut flashes, i, j);
                }
            }
        }

        grid.iter_mut().flatten().for_each(|x| {
            if *x > 9 {
                *x = 0;
            }
        });

        steps += 1;
        if !is_part2 && steps == 100 {
            println!("Part 1 - Result: {}", counter);
            break;
        }
        if is_part2 && grid.iter().all(|row| row.iter().all(|&x| x == 0)) {
            println!("Part 2 - Result: {}", steps);
            break;
        }
    }
}

fn main() {
    let grid = parse_input(std::io::stdin().lock());
    solve(grid.clone(), false);
    solve(grid, true);
}
