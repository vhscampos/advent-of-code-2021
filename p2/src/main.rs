use std::io::BufRead;

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
    aim: i32,
}

fn part1(steps: &[impl AsRef<str>]) {
    let mut cursor = Position { x: 0, y: 0, aim: 0 };

    for step in steps {
        let mut token_iterator = step.as_ref().split_whitespace();
        let direction = token_iterator.next().unwrap();
        let amount = token_iterator.next().unwrap().parse::<i32>().unwrap();
        match direction {
            "forward" => {
                cursor.x += amount;
            }
            "down" => {
                cursor.y += amount;
            }
            "up" => {
                cursor.y -= amount;
            }
            _ => panic!(),
        }
    }

    println!("{:?}", cursor);
    println!("{}", cursor.x * cursor.y);
}

fn part2(steps: &[impl AsRef<str>]) {
    let mut cursor = Position { x: 0, y: 0, aim: 0 };

    for step in steps {
        let mut token_iterator = step.as_ref().split_whitespace();
        let direction = token_iterator.next().unwrap();
        let amount = token_iterator.next().unwrap().parse::<i32>().unwrap();
        match direction {
            "forward" => {
                cursor.x += amount;
                cursor.y += cursor.aim * amount;
            }
            "down" => {
                cursor.aim += amount;
            }
            "up" => {
                cursor.aim -= amount;
            }
            _ => panic!(),
        }
    }

    println!("{:?}", cursor);
    println!("{}", cursor.x * cursor.y);
}

fn main() {
    let steps = std::io::stdin()
        .lock()
        .lines()
        .map(|res| res.unwrap())
        .collect::<Vec<String>>();

    part1(&steps);
    part2(&steps);
}
