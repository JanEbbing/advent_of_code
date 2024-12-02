use anyhow::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let mut left_list: Vec<i32> = Vec::new();
        let mut right_list: Vec<i32> = Vec::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            let location_ids: Vec<&str> = line.split("   ").collect::<Vec<&str>>();
            left_list.push(location_ids[0].parse::<i32>().unwrap());
            right_list.push(location_ids[1].parse::<i32>().unwrap());
        }
        left_list.sort();
        right_list.sort();
        let both_lists = left_list.iter().zip(right_list.iter());
        let mut result = 0;
        for (left_entry, right_entry) in both_lists {
            result += (right_entry - left_entry).abs();
        }

        return Ok(result);
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        // TODO factor out common part1 and part2 behaviour, e.g. parsing an input file
        let mut left_list: Vec<i32> = Vec::new();
        let mut right_list: Vec<i32> = Vec::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            let location_ids: Vec<&str> = line.split("   ").collect::<Vec<&str>>();
            left_list.push(location_ids[0].parse::<i32>().unwrap());
            right_list.push(location_ids[1].parse::<i32>().unwrap());
        }
        let mut right_list_counts: HashMap<i32, i32> = HashMap::new();
        let mut result = 0;
        for x in right_list {
            *right_list_counts.entry(x).or_default() += 1;
        }
        for location_id in left_list {
            result += location_id * (*right_list_counts.entry(location_id).or_default());
        }

        return Ok(result);
    }
    
    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
