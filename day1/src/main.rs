use std::fs;

// Enjoy my very rushed and DRY-violating day 1
// that was thrown together in the last hour or two of the day.

fn main() {
    part1();
    part2();
}

fn part1() {
    let challenge_input = fs::read_to_string("input.txt").unwrap();
    let mut left_hand_items: Vec<i32> = Vec::new();
    let mut right_hand_items: Vec<i32> = Vec::new();

    let rows: Vec<&str> = challenge_input.lines().collect();
    for row in &rows {
        let row_parts: Vec<&str> = row.split_whitespace().collect();
        let num1: i32 = row_parts[0].parse().unwrap();
        let num2: i32 = row_parts[1].parse().unwrap();
        left_hand_items.push(num1);
        right_hand_items.push(num2);
    }
    left_hand_items.sort();
    right_hand_items.sort();
    let num_rows = left_hand_items.len();
    assert_eq!(num_rows, right_hand_items.len());

    let mut distances: Vec<i32> = Vec::new();
    for i in 0..num_rows {
        let distance = left_hand_items[i] - right_hand_items[i];
        distances.push(distance.abs())
    }

    let distance_sum: i32 = distances.iter().sum();
    println!("{}", distance_sum);
}

fn part2() {
    let challenge_input = fs::read_to_string("input.txt").unwrap();
    let mut left_hand_items: Vec<i32> = Vec::new();
    let mut right_hand_items: Vec<i32> = Vec::new();

    let rows: Vec<&str> = challenge_input.lines().collect();
    for row in &rows {
        let row_parts: Vec<&str> = row.split_whitespace().collect();
        let num1: i32 = row_parts[0].parse().unwrap();
        let num2: i32 = row_parts[1].parse().unwrap();
        left_hand_items.push(num1);
        right_hand_items.push(num2);
    }
    left_hand_items.sort();
    right_hand_items.sort();

    let mut similarity_scores: Vec<i32> = Vec::new();

    for num in &left_hand_items {
        let num_appearances = right_hand_items.iter().filter(|&x| x == num).count() as i32;
        similarity_scores.push(num * num_appearances);
    }

    let total_similarity_score: i32 = similarity_scores.iter().sum();
    println!("{}", total_similarity_score);
}
