fn parse_input(handle: impl std::io::BufRead) -> (Vec<char>, Vec<Vec<char>>) {
    let mut line_iter = handle.lines();

    let algorithm = line_iter
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .collect::<Vec<char>>();

    line_iter.next();

    let image = line_iter
        .map(|line_result| {
            let line = line_result.unwrap();
            let v = line.chars().collect::<Vec<char>>();
            v
        })
        .collect();

    (algorithm, image)
}

fn print_image(image: &Vec<Vec<char>>) {
    for row in image {
        for column in row {
            print!("{}", column);
        }
        println!();
    }
}

fn solve(algorithm: &Vec<char>, mut input_image: Vec<Vec<char>>, steps: usize) {
    // Add steps+1 elements for padding in each of the 4 edges of the image
    for row in input_image.iter_mut() {
        for _ in 0..steps + 2 {
            row.insert(0, '.');
            row.push('.');
        }
    }
    for _ in 0..steps + 2 {
        input_image.insert(0, vec!['.'; input_image[0].len()]);
        input_image.push(vec!['.'; input_image[0].len()]);
    }

    let rows = input_image.len();
    let cols = input_image[0].len();
    let mut output_image = vec![vec!['.'; cols]; rows];

    println!("Original");
    print_image(&input_image);

    for _ in 0..steps {
        for i in 0..rows {
            for j in 0..cols {
                if i == 0 || i == rows - 1 || j == 0 || j == cols - 1 {
                    output_image[i][j] = match input_image[i][j] {
                        '.' => algorithm[0],
                        '#' => algorithm[511],
                        _ => panic!(),
                    };
                    continue;
                }

                let binary_str = input_image
                    .iter()
                    .skip(i - 1)
                    .take(3)
                    .flat_map(|row| row.iter().skip(j - 1).take(3))
                    .map(|&c| match c {
                        '.' => '0',
                        '#' => '1',
                        _ => panic!(),
                    })
                    .collect::<String>();
                let index = usize::from_str_radix(&binary_str, 2).unwrap();
                let output_char = algorithm[index];
                output_image[i][j] = output_char;
            }
        }

        input_image
            .iter_mut()
            .flatten()
            .zip(output_image.iter().flatten())
            .for_each(|(input_pixel, output_pixel)| *input_pixel = *output_pixel);
    }

    println!("Output after transform:");
    print_image(&output_image);

    let result = output_image
        .into_iter()
        .flatten()
        .filter(|&c| c == '#')
        .count();
    println!("Result: {}", result);
}

fn part1(algorithm: &Vec<char>, input_image: Vec<Vec<char>>) {
    solve(algorithm, input_image, 2);
}

fn part2(algorithm: &Vec<char>, input_image: Vec<Vec<char>>) {
    solve(algorithm, input_image, 50);
}

fn main() {
    let (algorithm, input_image) = parse_input(std::io::stdin().lock());
    part1(&algorithm, input_image.clone());
    part2(&algorithm, input_image);
}
