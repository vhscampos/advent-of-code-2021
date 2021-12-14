use std::collections::HashMap;

fn parse_input(handle: impl std::io::BufRead) -> (String, HashMap<String, String>) {
    let mut line_iterator = handle.lines();

    let template = line_iterator.by_ref().next().unwrap().unwrap();

    line_iterator.next();

    let rules = line_iterator
        .map(|line_result| {
            let line = line_result.unwrap();
            let mut tokenizer = line.split_whitespace();
            let from = tokenizer.next().unwrap().to_string();
            let to = tokenizer.nth(1).unwrap().to_string();
            (from, to)
        })
        .collect::<HashMap<String, String>>();

    (template, rules)
}

fn part1(template: &str, rules: &HashMap<String, String>) {
    let mut current = template.to_string();
    const NUM_STEPS: usize = 10;

    for _ in 0..NUM_STEPS {
        let mut iterator = current.chars();
        let mut next_state = String::with_capacity(current.len());
        next_state.push(iterator.clone().next().unwrap());

        for _ in 0..current.len() - 1 {
            let segment = iterator.clone().take(2).collect::<String>();
            let rule = &rules[&segment];
            next_state.push_str(rule);
            next_state.push(segment.chars().nth(1).unwrap());
            iterator.next();
        }

        current = next_state;
    }

    let mut counter_list = [None; 'Z' as usize - 'A' as usize];
    current.chars().for_each(|c| {
        *counter_list[c as usize - 'A' as usize].get_or_insert(0) += 1;
    });

    let max = counter_list.iter().filter_map(|&c| c).max().unwrap();
    let min = counter_list.iter().filter_map(|&c| c).min().unwrap();

    println!("Part 1 - Result: {}", max - min);
}

fn part2(template: &str, rules: &HashMap<String, String>) {
    let mut status = HashMap::<String, usize>::new();
    let mut iterator = template.chars();

    for _ in 0..template.len() - 1 {
        let segment = iterator.clone().take(2).collect::<String>();

        status.entry(segment).and_modify(|x| *x += 1).or_insert(1);

        iterator.next();
    }

    const NUM_STEPS: usize = 40;

    for _ in 0..NUM_STEPS {
        let mut new_status = HashMap::<String, usize>::with_capacity(status.len());
        for (key, value) in status.drain() {
            let rule = &rules[&key];
            let mut c_iterator = key.chars();
            let c1 = c_iterator.by_ref().next().unwrap();
            let c2 = c_iterator.next().unwrap();

            let mut segment1 = String::from(c1);
            segment1.push_str(rule);
            let mut segment2 = rule.clone();
            segment2.push(c2);

            new_status
                .entry(segment1)
                .and_modify(|x| *x += value)
                .or_insert(value);
            new_status
                .entry(segment2)
                .and_modify(|x| *x += value)
                .or_insert(value);
        }
        status = new_status;
    }

    let mut counter_list = [None; 'Z' as usize - 'A' as usize];

    for (key, value) in status.drain() {
        let c = key.chars().next().unwrap();
        *counter_list[c as usize - 'A' as usize].get_or_insert(0) += value;
    }
    *counter_list[template.chars().rev().next().unwrap() as usize - 'A' as usize]
        .get_or_insert(0) += 1;

    let max = counter_list.iter().filter_map(|&c| c).max().unwrap();
    let min = counter_list.iter().filter_map(|&c| c).min().unwrap();

    println!("Part 2 - Result: {}", max - min);
}

fn main() {
    let (template, rules) = parse_input(std::io::stdin().lock());
    part1(&template, &rules);
    part2(&template, &rules);
}
