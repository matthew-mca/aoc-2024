use std::fs;
use std::hash::Hash;

fn main() {
    println!("Reading rule input...");
    let rule_input = fs::read_to_string("example_rules.txt").unwrap();
    let rules: Vec<&str> = rule_input.lines().collect();

    println!("Reading update input...");
    let update_input = fs::read_to_string("example_updates.txt").unwrap();
    let updates: Vec<&str> = update_input.lines().collect();

    println!("Commencing part 1...");
    part1(&rules, &updates);
}

fn part1(rules: &Vec<&str>, updates: &Vec<&str>) {
    let page_order: Vec<usize> = get_page_order(&rules);
    let mut middle_page_total = 0;
    for line in updates {
        let pages_to_print: Vec<usize> = line.split(",")
            .into_iter()
            .map(|x| x.parse().unwrap())
            .collect();
        // Go in print order and get the indexes of any pages in the
        // line with a rule. We can then check the ordering of these pages.
        let mut page_rule_indexes: Vec<usize> = vec![];
        for page_number in &pages_to_print {
            let potential_index = get_page_index(&page_order, *page_number);
            if potential_index.is_some() {
                page_rule_indexes.push(potential_index.unwrap())
            }
        }
        // Now check that the vec is ordered correctly.
        if page_rule_indexes.is_sorted() {
            println!("Found valid sequence of length {}", page_rule_indexes.len());
            middle_page_total += get_middle_page_number_index(&pages_to_print)
        }
    }
    println!("{}", middle_page_total)
}

fn get_page_order(rules: &Vec<&str>) -> Vec<usize> {
    println!("Determining correct page order...");
    let mut unique_nums: Vec<usize> = vec![];
    let mut rule_pairs: Vec<Vec<usize>> = vec![];
    // Populate...
    println!("Populating rule set...");
    for rule in rules {
        let rule_nums: Vec<usize> = rule.split("|")
            .into_iter()
            .map(|x| x.parse().unwrap())
            .collect();

        unique_nums.push(rule_nums[0]);
        unique_nums.push(rule_nums[1]);
        rule_pairs.push(rule_nums);
    }

    println!("Removing duplicate numbers from rule set...");
    // Remove dupes...
    unique_nums.sort();
    unique_nums.dedup();

    println!("Sorting rule set according to rules provided...");
    // Sort according to our rules...
    let mut swaps_needed: bool = true;
    let mut iteration_count = 0u128;
    while swaps_needed {
        println!("Rules not sorted; beginning iteration {}...", iteration_count);
        swaps_needed = false;

        for pair in &rule_pairs {
            let before = pair[0];
            let after = pair[1];

            let before_index = get_page_index(&unique_nums, before).unwrap();
            let after_index = get_page_index(&unique_nums, after).unwrap();

            if before_index > after_index {
                // Move the before down by one
                unique_nums[before_index] = after;
                // Move the after up by one
                unique_nums[after_index] = before;
                swaps_needed = true;
            }
        }
        iteration_count += 1;
    }
    // And return the finished result.
    unique_nums
}

fn get_page_index(vec: &Vec<usize>, element: usize) -> Option<usize> {
    for i in 0..vec.len() {
        if vec[i] == element {
            return Some(i)
        }
    }
    None
}

fn get_middle_page_number_index(order: &Vec<usize>) -> usize {
    let middle_index = (order.len() as f32 / 2f32).floor() as usize;
    order[middle_index]
}