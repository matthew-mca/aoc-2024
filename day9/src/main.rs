use std::fs;

fn main() {
    let challenge_input = fs::read_to_string("input.txt").unwrap();
    let input_chars: Vec<char> = challenge_input.chars().collect();
    let mut expanded_disk: Vec<String> = vec![];
    // Create expanded representation of disk
    let mut current_file_id = 0;

    for i in 0..input_chars.len() {
        match i % 2 {
            // File
            0 => {
                let block_length: u32 = input_chars[i].to_digit(10).unwrap();
                for _ in 0..block_length {
                    expanded_disk.push(current_file_id.to_string());
                }
                current_file_id += 1;
            },
            // Empty space
            1 => {
                let block_length: u32 = input_chars[i].to_digit(10).unwrap();
                for _ in 0..block_length {
                    expanded_disk.push(String::from("."));
                }
            },
            // Empty default to keep compiler happy
            _ => {}
        }
    }

    let part_1_disk = expanded_disk.clone();
    part1(part_1_disk);

    let part_2_disk = expanded_disk.clone();
    part2(part_2_disk);
}

fn part1(mut expanded_disk: Vec<String>) {
    let mut checksum: u128 = 0;

    // Get a count of number of file blocks in order to verify they're still intact as we go.
    let block_count = expanded_disk.clone().into_iter().filter(|x| x.parse::<u64>().is_ok()).count() as u128;

    // Move file blocks
    while get_consecutive_block_count(&expanded_disk) != block_count {
        let earliest_space_index = expanded_disk.iter().position(|x| x.as_str() == ".").unwrap();
        let rightmost_element;

        'find_right: loop {
            if expanded_disk[expanded_disk.len() - 1].as_str() != "." {
                rightmost_element = expanded_disk.pop().unwrap();
                break 'find_right;
            }
            else {
                expanded_disk.pop();
            }
        }

        let disk_length = expanded_disk.len();

        if earliest_space_index >= disk_length {
            expanded_disk[disk_length - 1] = rightmost_element;
        }
        else {
            expanded_disk[earliest_space_index] = rightmost_element;
        }
    }

    for i in 0..expanded_disk.len() {
        if expanded_disk[i].as_str() == "." {
            break;
        }
        checksum += i as u128 * &expanded_disk[i].parse::<u128>().unwrap();
    }

    assert_eq!(block_count, get_consecutive_block_count(&expanded_disk));
    println!("Checksum: {}", checksum)
}

fn part2(expanded_disk: Vec<String>) {

}

fn get_consecutive_block_count(disk: &Vec<String>) -> u128 {
    let mut consecutive_block_count = 0u128;

    for str in disk {
        match str.as_str() {
            "." => return consecutive_block_count,
            _ => consecutive_block_count += 1,
        }
    }

    consecutive_block_count
}
