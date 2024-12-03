use std::fs;
use regex::Regex;

fn main() {
    let challenge_input = fs::read_to_string("input.txt").unwrap();
    let corrupt_str = challenge_input.as_str();
    part1(corrupt_str);
    part2(corrupt_str);
}

fn part1(input: &str) {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let mul_commands: Vec<&str> = re.find_iter(input)
        .map(|x| x.as_str())
        .collect();
    let mut mul_total: i32 = 0;
    for str in mul_commands {
        mul_total += _execute_mul_command(str);
    }
    println!("{}", mul_total);
}

fn part2(input: &str) {
    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|do(n't)?\(\))").unwrap();
    let mul_commands: Vec<&str> = re.find_iter(input)
        .map(|x| x.as_str())
        .collect();

    let mut do_mul_commands: bool = true;
    let mut mul_total: i32 = 0;

    for str in mul_commands {
        match str {
            "do()" => if !do_mul_commands { do_mul_commands = true; },
            "don't()" => if do_mul_commands { do_mul_commands = false; },
            _ => mul_total += _maybe_execute_mul_command(str, &do_mul_commands),
        }
    }
    println!("{}", mul_total);
}

fn _maybe_execute_mul_command(command: &str, &do_mul_commands: &bool) -> i32 {
    if do_mul_commands {
        return _execute_mul_command(command);
    }
    0
}

fn _execute_mul_command(command: &str) -> i32 {
    let re = Regex::new(r"\d{1,3}").unwrap();
    let nums: Vec<i32> = re.find_iter(command)
        .map(|x| x.as_str().parse().unwrap())
        .collect();

    nums[0] * nums[1]
}
