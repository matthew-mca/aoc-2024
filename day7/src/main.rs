use std::fs;

enum Operator{
    Add,
    Multiply,
    Concatenate,
}

fn main() {
    let challenge_input = fs::read_to_string("input.txt").unwrap();

    let mut calibration_total_part_1 = 0;
    let mut calibration_total_part_2 = 0;

    for line in challenge_input.as_str().lines() {
        let equation_parts: Vec<&str> = line.split(":").collect();

        let part_1_operators = vec![
            Operator::Add,
            Operator::Multiply,
        ];

        let part_2_operators = vec![
            Operator::Add,
            Operator::Multiply,
            Operator::Concatenate,
        ];

        let target: u128 = equation_parts[0].parse().unwrap();
        let test_values: Vec<u128> = equation_parts[1].trim()
            .split(" ")
            .map(|x| x.parse().unwrap())
            .collect();

        // We will pass in a Vec and try all the combinations of operators, then return it out.
        // The vec will double in size each time.
        let mut values_to_test_part_1 = vec![test_values[0]];
        let mut values_to_test_part_2 = vec![test_values[0]];

        for value in &test_values[1..] {
            values_to_test_part_1 = apply_operators_to_test_values(&part_1_operators, values_to_test_part_1, value);
            values_to_test_part_2 = apply_operators_to_test_values(&part_2_operators, values_to_test_part_2, value);
        }

        // Once all combinations of final numbers are calculated, check if any of them is the target
        // value.
        if values_to_test_part_1.contains(&target) {
            calibration_total_part_1 += target;
        }
        if values_to_test_part_2.contains(&target) {
            calibration_total_part_2 += target;
        }
    }
    println!("{}", calibration_total_part_1);
    println!("{}", calibration_total_part_2);
}

fn apply_operators_to_test_values(operators: &Vec<Operator>, test_values: Vec<u128>, next: &u128) -> Vec<u128> {
    let mut new_test_values: Vec<u128> = vec![];

    for value in test_values {
        for operator in operators {
            match operator {
                Operator::Multiply => new_test_values.push(value * next),
                Operator::Add => new_test_values.push(value + next),
                Operator::Concatenate => {
                    let concatenated_num: u128 = format!("{}{}", value, next).parse().unwrap();
                    new_test_values.push(concatenated_num);
                },
            }
        }
    }

    new_test_values
}
