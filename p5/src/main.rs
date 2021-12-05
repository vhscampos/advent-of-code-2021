fn parse_input(handle: impl std::io::BufRead) -> Vec<Vec<Vec<i32>>> {
    handle
        .lines()
        .map(|l| {
            l.unwrap()
                .split(" -> ")
                .map(|x| {
                    x.split(',')
                        .map(|y| y.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>()
        })
        .collect::<Vec<Vec<Vec<i32>>>>()
}

fn solution(coords: &Vec<Vec<Vec<i32>>>, allow_diagonal_lines: bool) {
    let grid_size = (
        coords
            .iter()
            .map(|coord| std::cmp::max(coord[0][0], coord[1][0]))
            .max()
            .unwrap()
            + 1,
        coords
            .iter()
            .map(|coord| std::cmp::max(coord[0][1], coord[1][1]))
            .max()
            .unwrap()
            + 1,
    );

    let mut grid = vec![vec![0; grid_size.1 as usize]; grid_size.0 as usize];

    for coord in coords {
        let point1 = &coord[0];
        let point2 = &coord[1];

        if point1[0] == point2[0] {
            // Vertical
            let x = point1[0];
            for y in std::cmp::min(point1[1], point2[1])..=std::cmp::max(point1[1], point2[1]) {
                grid[x as usize][y as usize] += 1;
            }
        } else if point1[1] == point2[1] {
            // Horizontal
            let y = point1[1];
            for x in std::cmp::min(point1[0], point2[0])..=std::cmp::max(point1[0], point2[0]) {
                grid[x as usize][y as usize] += 1;
            }
        } else if allow_diagonal_lines {
            // Diagonal
            let step: (i32, i32) = (
                if point1[0] < point2[0] { 1 } else { -1 },
                if point1[1] < point2[1] { 1 } else { -1 },
            );

            let (mut x, mut y) = (point1[0], point1[1]);

            while x != point2[0] + step.0 && y != point2[1] + step.1 {
                grid[x as usize][y as usize] += 1;
                x += step.0;
                y += step.1;
            }
        }
    }

    let result = grid.into_iter().flatten().filter(|&x| x >= 2).count();
    println!("{}", result);
}

fn main() {
    let coords = parse_input(std::io::stdin().lock());
    solution(&coords, false);
    solution(&coords, true);
}
