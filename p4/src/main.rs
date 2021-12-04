type Board = Vec<Vec<i32>>;

fn parse_input<T: std::io::BufRead>(input: T) -> (Vec<i32>, Vec<Board>) {
    let mut input_iterator = input.lines().fuse().peekable();
    let rand_numbers = input_iterator
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    input_iterator.next();

    let mut boards = Vec::<Board>::new();

    while input_iterator.peek().is_some() {
        boards.push(Vec::<Vec<i32>>::with_capacity(5));
        let idx = boards.len() - 1;

        for _ in 0..5 {
            let line = input_iterator
                .next()
                .unwrap()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            boards[idx].push(line);
        }

        input_iterator.next();
    }

    (rand_numbers, boards)
}

fn part1(rand_numbers: &[i32], boards: &[Board]) {
    let mut marks = vec![[[false; 5]; 5]; boards.len()];
    let mut winning_board_opt = None;
    let mut winning_number_opt = None;

    for &rand_number in rand_numbers {
        for (board_id, board) in boards.iter().enumerate() {
            for (row, board) in board.iter().enumerate() {
                board
                    .iter()
                    .enumerate()
                    .filter(|(_, &x)| x == rand_number)
                    .for_each(|(col, _)| marks[board_id][row][col] = true);
            }

            let any_row_complete = (0..5).any(|row| (0..5).all(|col| marks[board_id][row][col]));
            let any_col_complete = (0..5).any(|col| (0..5).all(|row| marks[board_id][row][col]));
            if any_row_complete || any_col_complete {
                winning_board_opt = Some(board_id);
                break;
            }
        }
        if winning_board_opt.is_some() {
            winning_number_opt = Some(rand_number);
            break;
        }
    }

    let winning_board = winning_board_opt.unwrap();
    let winning_number = winning_number_opt.unwrap();

    let result = winning_number
        * (0usize..5).fold(0, |acc, row| {
            acc + (0usize..5)
                .filter(|&col| !marks[winning_board][row][col])
                .fold(0, |acc_row, col| acc_row + boards[winning_board][row][col])
        });

    println!("{}", result);
}

fn part2(rand_numbers: &[i32], boards: &[Board]) {
    let mut marks = vec![[[false; 5]; 5]; boards.len()];
    let mut boards_won = vec![false; boards.len()];
    let mut last_winning_board_opt = None;
    let mut last_winning_number_opt = None;

    for &rand_number in rand_numbers {
        for (board_id, board) in boards.iter().enumerate() {
            if boards_won[board_id] {
                continue;
            }
            for (row, board) in board.iter().enumerate() {
                board
                    .iter()
                    .enumerate()
                    .filter(|(_, &x)| x == rand_number)
                    .for_each(|(col, _)| marks[board_id][row][col] = true);
            }

            let any_row_complete = (0..5).any(|row| (0..5).all(|col| marks[board_id][row][col]));
            let any_col_complete = (0..5).any(|col| (0..5).all(|row| marks[board_id][row][col]));
            if any_row_complete || any_col_complete {
                boards_won[board_id] = true;
                last_winning_board_opt = Some(board_id);
            }
        }
        if boards_won.iter().all(|&x| x) {
            last_winning_number_opt = Some(rand_number);
            break;
        }
    }

    let last_winning_board = last_winning_board_opt.unwrap();
    let last_winning_number = last_winning_number_opt.unwrap();

    let result = last_winning_number
        * (0usize..5).fold(0, |acc, row| {
            acc + (0usize..5)
                .filter(|&col| !marks[last_winning_board][row][col])
                .fold(0, |acc_row, col| {
                    acc_row + boards[last_winning_board][row][col]
                })
        });

    println!("{}", result);
}

fn main() {
    let (rand_numbers, boards) = parse_input(std::io::stdin().lock());
    part1(&rand_numbers, &boards);
    part2(&rand_numbers, &boards);
}
