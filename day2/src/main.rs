use std::fs;

fn main() {
    let challenge_input = fs::read_to_string("input.txt").unwrap();
    let rows: Vec<&str> = challenge_input.lines().collect();

    part1(&rows);
    part2(&rows);
}

fn part1(input: &Vec<&str>) {
    let mut valid_report_count: u32 = 0;
    for row in input {
        let report_numbers: Vec<i32> = row.split_whitespace()
            .into_iter()
            .map(|x| x.parse().unwrap())
            .collect();

         if _is_valid_report(&report_numbers) {
             valid_report_count += 1;
         }
    }
    println!("{}", valid_report_count);
}

fn part2(input: &Vec<&str>) {
    let mut valid_report_count: u32 = 0;
    for row in input {
        let report_numbers: Vec<i32> = row.split_whitespace()
            .into_iter()
            .map(|x| x.parse().unwrap())
            .collect();

        if _is_valid_report_problem_dampener(&report_numbers) {
            valid_report_count += 1;
        }
    }
    println!("{}", valid_report_count);
}

fn _is_valid_report_problem_dampener(numbers: &Vec<i32>) -> bool {
    if _is_valid_report(numbers) {
        return true;
    }
    // If the initial report isn't valid, try checking if any of the reduced versions are valid
    // instead.
    for i in 0..numbers.len() {
        let mut reduced_vec = numbers.to_vec();
        reduced_vec.remove(i);
        if _is_valid_report(&reduced_vec) {
            return true;
        }
    }
    false
}

fn _is_valid_report(numbers: &Vec<i32>) -> bool {
    let mut all_diffs: Vec<i32> = Vec::new();

    for i in 0..numbers.len() - 1 {
        let current_diff = numbers[i] - numbers[i + 1];

        // Check difference constraint
        if current_diff.abs() < 1 || current_diff.abs() > 3 {
            return false;
        }
        all_diffs.push(current_diff);
    }

    // If all the diffs are negative, each number is smaller than the next.
    let all_increasing: bool = all_diffs.iter().all(|&x| x < 0);
    // If all positive, each number is larger than the next.
    let all_decreasing: bool = all_diffs.iter().all(|&x| x > 0);

    if !(all_increasing || all_decreasing) {
        return false;
    }
    true
}