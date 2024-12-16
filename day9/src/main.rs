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

    // let part_1_disk = expanded_disk.clone();
    // part1(part_1_disk);

    part2(input_chars.clone());
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

#[derive(Clone)]
struct Span {
    id: String,
    length: u32,
}

struct Disk {
    blocks: Vec<Span>,
    already_visited_ids: Vec<String>,
}

impl Disk {
    fn print_disk_contents(&self) {
        for block in &self.blocks {
            for _ in 0..block.length {
                print!("{}", block.id);
            }
        }
    }

    fn disk_contents_to_str(&self) -> String {
        let mut contents = String::from("");
        for block in &self.blocks {
            for _ in 0..block.length {
                contents.push_str(block.id.as_str())
            }
        }

        contents
    }

    // fn get_consecutive_block_count(&self) -> u128 {
    //     let mut consecutive_block_count = 0u128;
    //
    //     for block in &self.blocks {
    //         match block.id.as_str() {
    //             "." => return consecutive_block_count,
    //             _ => consecutive_block_count += block.length as u128,
    //         }
    //     }
    //
    //     consecutive_block_count
    // }

    fn get_first_valid_space_index(&self, length_needed: u32) -> Option<usize> {
        for i in 0..self.blocks.len() {
            let current = &self.blocks[i];
            if current.id.as_str() == "." && current.length >= length_needed {
                return Some(i);
            }
        }
        None
    }

    fn get_span_index_by_id(&self, target_id: &String) -> Option<usize> {
        for i in 0..self.blocks.len() {
            let current = &self.blocks[i];
            if current.id.as_str() == target_id.as_str() {
                return Some(i);
            }
        }
        None
    }

    fn compact_disk_files_single_pass(self) -> Disk {
        let disk_length = self.blocks.len();

        // To avoid weird happenings with looping over a vec while it is being modified,
        // any time there is a modification, completely restart the backwards traversal.
        // We need to keep track of the ids already visited to avoid processing them again.
        let mut disk_clone = self.blocks.clone();
        let mut visited_ids: Vec<&str> = vec![];
        for str in &self.already_visited_ids {
            visited_ids.push(str.as_str())
        }

        for i in 1..disk_length + 1 {
            let current_block = &self.blocks[disk_length - i];
            if current_block.id.as_str() == "." {
                continue;
            }
            if visited_ids.contains(&current_block.id.as_str()){
                continue
            }

            // At this point, we have found an unvisited file block.
            // Consider the ID now visited before we go any further.
            visited_ids.push(current_block.id.as_str());

            // Now we need to find a valid empty space and make sure it's to the left of the block.
            let span_index = self.get_span_index_by_id(&current_block.id).unwrap();
            let first_valid_space_index_result = self.get_first_valid_space_index(current_block.length);
            let first_valid_space_index;
            match first_valid_space_index_result {
                Some(index) => {
                    // Continue if the space is on the right of the span
                    if index > span_index {
                        continue;
                    }
                    first_valid_space_index = index;
                },
                None => continue,
            }

            // At this point, there is a valid space.
            // We now make the move into it and restart the wider loop.
            // Start by swapping the block with empty space.
            let block_to_move = disk_clone.remove(span_index);
            disk_clone.insert(span_index, Span{
                id: String::from("."),
                length: current_block.length,
            });

            // Then replace the valid empty space with both the file block. If the empty space
            // has a greater length than the file block, also replace an empty space with
            // the remaining length.
            let removed_space = disk_clone.remove(first_valid_space_index);
            let leftover_space = removed_space.length - block_to_move.length;
            if leftover_space > 0 {
                disk_clone.insert(first_valid_space_index, Span{
                    id: String::from("."),
                    length: leftover_space,
                });
            }
            disk_clone.insert(first_valid_space_index, block_to_move);
            break;
        }

        Disk{
            blocks: disk_clone,
            already_visited_ids: self.already_visited_ids.into_iter().map(|x| x.to_string()).collect(),
        }
    }
}

fn part2(disk_chars: Vec<char>) {
    let mut checksum = 0u128;
    let mut blocks: Vec<Span> = vec![];
    let mut current_file_id = 0;

    for i in 0..disk_chars.len() {
        match i % 2 {
            // File
            0 => {
                blocks.push(Span{
                    id: current_file_id.to_string(),
                    length: disk_chars[i].to_digit(10).unwrap(),
                });
                current_file_id += 1;
            },
            // Empty space
            1 => {
                blocks.push(Span{
                    id: String::from("."),
                    length: disk_chars[i].to_digit(10).unwrap(),
                });
            },
            // Empty default to keep compiler happy
            _ => {}
        }
    }

    let mut expanded_disk = Disk{blocks, already_visited_ids: vec![]};
    loop {
        let prev_disk = expanded_disk.disk_contents_to_str();
        expanded_disk = expanded_disk.compact_disk_files_single_pass();
        let next_disk = expanded_disk.disk_contents_to_str();

        if prev_disk.as_str() == next_disk.as_str() {
            break;
        }
    }

    // println!("Final: {}", expanded_disk.disk_contents_to_str());

    let final_disk_chars: Vec<char> = expanded_disk.disk_contents_to_str().chars().collect();
    let mut index = 0u128;
    for char in &final_disk_chars {
        if *char != '.' {
            let digit = char.to_digit(10).unwrap() as u128;
            checksum += index * digit;
        }
        index += 1;
    }

    println!("Checksum: {}", checksum)

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
