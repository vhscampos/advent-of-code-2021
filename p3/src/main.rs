use std::io::BufRead;

fn bin_to_dec(s: &str) -> i32 {
    s.chars().rev().enumerate().fold(0, |acc, (pos, i)| {
        acc + (1 << pos) * (i as i32 - '0' as i32)
    })
}

fn part1(l: &[String]) {
    let line_len = l[0].len();
    let mut gamma_rate = String::with_capacity(line_len);

    for pos in 0..line_len {
        let mut counter = 0;
        for line in l.iter() {
            let bit = line.chars().nth(pos).unwrap();
            if bit == '1' {
                counter += 1;
            } else {
                counter -= 1;
            }
        }
        if counter > 0 {
            gamma_rate.push('1');
        } else {
            gamma_rate.push('0');
        }
    }

    let echo_rate = gamma_rate
        .chars()
        .map(|c| if c == '1' { '0' } else { '1' })
        .collect::<String>();

    println!("Gamma rate: {}", gamma_rate);
    println!("Echo rate: {}", echo_rate);

    let gamma_rate_decimal = bin_to_dec(&gamma_rate);
    let echo_rate_decimal = bin_to_dec(&echo_rate);

    println!("Gamma rate decimal: {}", gamma_rate_decimal);
    println!("Echo rate decimal: {}", echo_rate_decimal);
    println!("Product: {}", gamma_rate_decimal * echo_rate_decimal);
}

fn part2(l: &[String]) {
    let line_len = l[0].len();

    let mut candidates = (0..l.len()).collect::<Vec<usize>>();

    for pos in 0..line_len {
        let mut counter = 0;
        for &cand in candidates.iter() {
            let bit = l[cand].chars().nth(pos).unwrap();
            if bit == '1' {
                counter += 1;
            } else {
                counter -= 1;
            }
        }
        let most_common = if counter >= 0 { '1' } else { '0' };

        candidates = candidates
            .into_iter()
            .filter(|&x| l[x].chars().nth(pos).unwrap() == most_common)
            .collect();

        if candidates.len() == 1 {
            break;
        }
    }

    assert_eq!(candidates.len(), 1);
    let oxygen_rate = &l[candidates[0]];

    candidates = (0..l.len()).collect::<Vec<usize>>();

    for pos in 0..line_len {
        let mut counter = 0;
        for &cand in candidates.iter() {
            let bit = l[cand].chars().nth(pos).unwrap();
            if bit == '1' {
                counter += 1;
            } else {
                counter -= 1;
            }
        }
        let least_common = if counter >= 0 { '0' } else { '1' };

        candidates = candidates
            .into_iter()
            .filter(|&x| l[x].chars().nth(pos).unwrap() == least_common)
            .collect();

        if candidates.len() == 1 {
            break;
        }
    }

    assert_eq!(candidates.len(), 1);
    let co2_rate = &l[candidates[0]];

    let oxygen_rate_decimal = bin_to_dec(oxygen_rate);
    let co2_rate_decimal = bin_to_dec(co2_rate);

    println!("Oxygen rate: {}", oxygen_rate);
    println!("CO2 rate: {}", co2_rate);
    println!("Oxygen rate decimal: {}", oxygen_rate_decimal);
    println!("CO2 rate decimal: {}", co2_rate_decimal);
    println!("Product: {}", oxygen_rate_decimal * co2_rate_decimal);
}

fn main() {
    let l = std::io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>();

    part1(&l);
    part2(&l);
}
