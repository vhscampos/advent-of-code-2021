use itertools::Itertools;

fn parse_input(handle: impl std::io::BufRead) -> Vec<(Vec<String>, Vec<String>)> {
    handle
        .lines()
        .map(|line_result| line_result.unwrap())
        .map(|line| {
            let mut tokenizer = line.split_whitespace();
            let signals = tokenizer
                .by_ref()
                .take(10)
                .map(|x| x.to_owned())
                .collect::<Vec<String>>();

            tokenizer.next();

            let display = tokenizer
                .take(4)
                .map(|x| x.to_owned())
                .collect::<Vec<String>>();

            (signals, display)
        })
        .collect::<Vec<(Vec<String>, Vec<String>)>>()
}

fn part1(signals_display_list: &Vec<(Vec<String>, Vec<String>)>) {
    let mut num_1 = 0;
    let mut num_4 = 0;
    let mut num_7 = 0;
    let mut num_8 = 0;

    for signals_display in signals_display_list {
        let display = &signals_display.1;
        for digit in display {
            match digit.len() {
                2 => num_1 += 1,
                4 => num_4 += 1,
                3 => num_7 += 1,
                7 => num_8 += 1,
                _ => {}
            }
        }
    }

    println!("Part 1 - Result: {}", num_1 + num_4 + num_7 + num_8);
}

fn letters_to_numeric(letters: &str) -> Result<usize, ()> {
    match letters {
        "abcefg" => Ok(0),
        "cf" => Ok(1),
        "acdeg" => Ok(2),
        "acdfg" => Ok(3),
        "bcdf" => Ok(4),
        "abdfg" => Ok(5),
        "abdefg" => Ok(6),
        "acf" => Ok(7),
        "abcdefg" => Ok(8),
        "abcdfg" => Ok(9),
        _ => Err(()),
    }
}

fn part2(signals_display_list: &Vec<(Vec<String>, Vec<String>)>) {
    let alphabet = "abcdefg".chars().collect::<Vec<char>>();
    let alphabet_len = alphabet.len();

    let total = signals_display_list
        .iter()
        .map(|signals_display| {
            let signals = &signals_display.0;
            for candidate in alphabet.iter().permutations(alphabet_len) {
                let mut digits_found = vec![false; 10];
                for signal in signals {
                    let translated = signal
                        .chars()
                        .map(|c| candidate[c as usize - 'a' as usize])
                        .sorted()
                        .collect::<String>();
                    let decoded_result = letters_to_numeric(&translated);
                    let decoded = match decoded_result {
                        Ok(x) => x,
                        Err(_) => break,
                    };
                    if digits_found[decoded] {
                        break;
                    } else {
                        digits_found[decoded] = true;
                    }
                }

                if digits_found.into_iter().all(|x| x) {
                    // Solved
                    let display = &signals_display.1;
                    let result = display
                        .iter()
                        .map(|component| {
                            let translated = component
                                .chars()
                                .map(|c| candidate[c as usize - 'a' as usize])
                                .sorted()
                                .collect::<String>();
                            let decoded_result = letters_to_numeric(&translated);
                            let decoded = decoded_result.unwrap();
                            decoded.to_string()
                        })
                        .collect::<String>();

                    return result.parse::<usize>().unwrap();
                }
            }

            panic!("No suitable mapping found");
        })
        .sum::<usize>();

    println!("Part 2 - Result: {}", total);
}

fn main() {
    let signals_display_list = parse_input(std::io::stdin().lock());
    part1(&signals_display_list);
    part2(&signals_display_list);
}
