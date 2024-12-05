use anyhow::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn adheres_to_order(update: &Vec<usize>, rules: &HashMap<usize, Vec<usize>>, update_to_indices: &HashMap<usize, usize>) -> bool {
        for i in 0..update.len() {
            let entry = update[i];
            let rules_for_entry = rules.get(&entry);
            if let Some(entries_must_be_after_entry) = rules_for_entry {
                for entry_must_be_after_entry in entries_must_be_after_entry {
                    let maybe_index_of_rule_partner = update_to_indices.get(entry_must_be_after_entry);
                    if let Some(update_index) = maybe_index_of_rule_partner {
                        if update_index < &i {
                            return false;
                        }
                    }
                }
            }
        }
        return true;
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut updates: Vec<Vec<usize>> = Vec::new();
        let mut updates_to_indices: Vec<HashMap<usize, usize>> = Vec::new();
        let mut rule_mode = true;
        for line in reader.lines().map(|l| l.unwrap()) {
            if line.trim().is_empty() {
                rule_mode = false;
                continue;
            }
            if rule_mode {
                let tokens: Vec<&str> = line.split("|").collect();
                let key: usize = tokens[0].parse::<usize>().unwrap();
                let val_to_append: usize = tokens[1].parse::<usize>().unwrap();
                if rules.contains_key(&key) {
                    rules.get_mut(&key).expect("I just checked the key exists").push(val_to_append);
                } else {
                    let rule_val: Vec<usize> = vec![val_to_append];
                    rules.insert(key, rule_val);
                }
            } else {
                let update: Vec<usize> = line.split(",").map(|s| s.parse::<usize>().unwrap()).collect();
                let n = update.len();
                let mut update_to_indices: HashMap<usize, usize> = HashMap::new();
                for i in 0..n {
                    update_to_indices.insert(update[i], i);
                }
                updates_to_indices.push(update_to_indices);
                updates.push(update);
            }
        }
        
        let mut valid_updates: Vec<&Vec<usize>> = Vec::new();
        for i in 0..updates.len() {
            if adheres_to_order(&updates[i], &rules, &updates_to_indices[i]) {
                valid_updates.push(&updates[i]);
            }
        }
        let mut result: usize = 0;
        for valid_update in valid_updates {
            result += valid_update[valid_update.len() / 2];
        }
        Ok(result)
    }

    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn make_updates_valid(mut invalid_updates: Vec<Vec<usize>>, inverted_rules: &HashMap<usize, Vec<usize>>) -> Vec<Vec<usize>> {
        let mut result: Vec<Vec<usize>> = Vec::new();
        for invalid_update in invalid_updates.iter_mut() {
            let mut valid_update: Vec<usize> = Vec::new();
            let n = invalid_update.len();
            while valid_update.len() != n {
                let next_entry_index = invalid_update.iter().position(|u| 
                    !inverted_rules.contains_key(u) || 
                    inverted_rules.get(u).expect("Just checked its present").iter().all(|p| valid_update.contains(p) || !invalid_update.contains(p))
                ).expect("Puzzle is well-defined");
                
                valid_update.push(invalid_update[next_entry_index]);
                invalid_update.remove(next_entry_index);
            }
            result.push(valid_update);
        }

        result
    }
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut inverted_rules: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut updates: Vec<Vec<usize>> = Vec::new();
        let mut updates_to_indices: Vec<HashMap<usize, usize>> = Vec::new();
        let mut rule_mode = true;
        for line in reader.lines().map(|l| l.unwrap()) {
            if line.trim().is_empty() {
                rule_mode = false;
                continue;
            }
            if rule_mode {
                let tokens: Vec<&str> = line.split("|").collect();
                let key: usize = tokens[0].parse::<usize>().unwrap();
                let val_to_append: usize = tokens[1].parse::<usize>().unwrap();
                if rules.contains_key(&key) {
                    rules.get_mut(&key).expect("I just checked the key exists").push(val_to_append);
                } else {
                    let rule_val: Vec<usize> = vec![val_to_append];
                    rules.insert(key, rule_val);
                }
                if inverted_rules.contains_key(&val_to_append) {
                    inverted_rules.get_mut(&val_to_append).expect("I just checked the key exists").push(key);
                } else {
                    let rule_val: Vec<usize> = vec![key];
                    inverted_rules.insert(val_to_append, rule_val);
                }
            } else {
                let update: Vec<usize> = line.split(",").map(|s| s.parse::<usize>().unwrap()).collect();
                let n = update.len();
                let mut update_to_indices: HashMap<usize, usize> = HashMap::new();
                for i in 0..n {
                    update_to_indices.insert(update[i], i);
                }
                updates_to_indices.push(update_to_indices);
                updates.push(update);
            }
        }
        
        let mut invalid_updates: Vec<Vec<usize>> = Vec::new();
        for i in 0..updates.len() {
            if !adheres_to_order(&updates[i], &rules, &updates_to_indices[i]) {
                invalid_updates.push(updates[i].clone());
            }
        }
        let valid_updates = make_updates_valid(invalid_updates, &inverted_rules);
        let mut result: usize = 0;
        for valid_update in valid_updates {
            result += valid_update[valid_update.len() / 2];
        }
        Ok(result)
    }
    
    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
