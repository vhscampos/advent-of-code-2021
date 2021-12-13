#![allow(clippy::needless_range_loop)]

#[derive(Clone, Copy, PartialEq, Debug)]
enum FoldAlong {
    X,
    Y,
}

fn parse_input(handle: impl std::io::BufRead) -> (Vec<Vec<bool>>, Vec<(FoldAlong, usize)>) {
    let mut it = handle.lines();
    let coord_list = it
        .by_ref()
        .take_while(|line_result| {
            let line = line_result.as_ref().unwrap();
            !line.is_empty()
        })
        .map(|line_result| {
            let line = line_result.unwrap();
            let mut tokenizer = line.split(',');
            (
                tokenizer.next().unwrap().parse::<usize>().unwrap(),
                tokenizer.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<(usize, usize)>>();

    let largest_x = coord_list.iter().map(|(x, _)| x).max().unwrap();
    let largest_y = coord_list.iter().map(|(_, y)| y).max().unwrap();
    let mut result = vec![vec![false; largest_x + 1]; largest_y + 1];

    coord_list
        .into_iter()
        .for_each(|(x, y)| result[y][x] = true);

    let fold_list = it
        .map(|line_result| {
            let line = line_result.unwrap();
            let mut tokenizer1 = line.split_whitespace();
            let description = tokenizer1.nth(2).unwrap();
            let mut tokenizer2 = description.split('=');
            let axis = tokenizer2.next().unwrap();
            let amount = tokenizer2.next().unwrap().parse::<usize>().unwrap();
            (
                if axis == "x" {
                    FoldAlong::X
                } else {
                    FoldAlong::Y
                },
                amount,
            )
        })
        .collect::<Vec<(FoldAlong, usize)>>();

    (result, fold_list)
}

fn print_paper(paper: &[Vec<bool>]) {
    for row in paper {
        for col in row {
            if !*col {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
    }
    println!("End");
}

fn solve<'a>(
    paper: &mut Vec<Vec<bool>>,
    fold_iter: impl Iterator<Item = &'a (FoldAlong, usize)>,
) -> usize {
    for fold in fold_iter {
        let height = paper.len();
        let width = paper[0].len();

        let base = fold.1;
        if fold.0 == FoldAlong::X {
            for y in 0..height {
                (1..=std::cmp::min(base, width - base - 1)).for_each(|i| {
                    paper[y][base - i] |= paper[y][base + i];
                })
            }
            paper
                .iter_mut()
                .for_each(|row| row.resize(base, bool::default()));
        } else {
            for i in 1..=std::cmp::min(base, height - base - 1) {
                (0..width).for_each(|x| paper[base - i][x] |= paper[base + i][x]);
            }
            paper.resize(base, Vec::<bool>::default());
        }
    }
    let counter = paper.iter().flatten().filter(|&&x| x).count();
    counter
}

fn part1(paper: &mut Vec<Vec<bool>>, fold_list: &[(FoldAlong, usize)]) {
    let fold_iter = fold_list.iter().take(1);
    let result = solve(paper, fold_iter);
    println!("Part 1 - Result: {}", result);
}

fn part2(paper: &mut Vec<Vec<bool>>, fold_list: &[(FoldAlong, usize)]) {
    let fold_iter = fold_list.iter();
    solve(paper, fold_iter);
    println!("Part 2 - Result:");
    print_paper(paper);
}

fn main() {
    let (mut paper, fold_list) = parse_input(std::io::stdin().lock());
    part1(&mut paper.clone(), &fold_list);
    part2(&mut paper, &fold_list);
}
