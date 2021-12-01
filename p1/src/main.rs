use std::io::{self, BufRead};

fn main() {
    let depth_list = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut result = 0;
    let mut last_window_sum = i32::max_value();
    for i in 0..depth_list.len() - 2 {
        let window_sum = depth_list[i] + depth_list[i + 1] + depth_list[i + 2];
        if window_sum > last_window_sum {
            result += 1;
        }
        last_window_sum = window_sum;
    }

    println!("{}", result);
}
