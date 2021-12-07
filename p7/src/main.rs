fn parse_input(handle: impl std::io::BufRead) -> Vec<u32> {
    handle
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

fn part1(list: &[u32]) {
    let mut positions = list.to_owned();
    positions.sort_unstable();
    let size = positions.len();
    let median = if size % 2 == 0 {
        (positions[size / 2 - 1] + positions[size / 2]) / 2
    } else {
        positions[size / 2]
    };

    let result = positions
        .into_iter()
        .map(|x| (median as i32 - x as i32).abs() as u32)
        .sum::<u32>();
    println!("Part 1 - Result: {}", result);
}

#[inline]
fn calculate_fuel(a: u32, b: u32) -> u32 {
    let n = (a as i32 - b as i32).abs() as u32;
    ((1 + n) * n) / 2
}

fn part2(list: &[u32]) {
    let max_position = list.iter().max().unwrap();

    let result = (0..=*max_position)
        .map(|position| {
            list.iter()
                .map(|&x| calculate_fuel(position, x))
                .sum::<u32>()
        })
        .min()
        .unwrap();

    println!("Part 2 - Result: {}", result);
}

fn main() {
    let list = parse_input(std::io::stdin().lock());
    part1(&list);
    part2(&list);
}
