use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use adv_code_2025::*;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut result: usize = 0;
        let mut ranges: Vec<(u64, u64)> = Vec::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            for range in line.split(",") {
                let tokens = range.split("-").collect::<Vec<&str>>();
                let left_side_of_range = tokens[0].parse::<u64>().unwrap();
                let right_side_of_range = tokens[1].parse::<u64>().unwrap();
                ranges.push((left_side_of_range, right_side_of_range));
            }
        }

        for (left_side, right_side) in ranges {
            for i in left_side..right_side+1 {
                let mut num_digits = ((i as f64).log10() as u64 + 1).try_into().unwrap();
                num_digits = num_digits / 2;
                let first_num: u64 = i / 10u64.pow(num_digits);
                let second_num: u64 = i % 10u64.pow(num_digits);
                //println!("left side {} right side {} num_digits {} first_num {} second_num {}", left_side, right_side, num_digits, first_num, second_num);
                if first_num == second_num {
                    result += i as usize;
                }
            }
        }
        Ok(result)
    }

    assert_eq!(1227775554, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn vec_is_all_same(arr: &[u64]) -> bool {
        if arr.is_empty() {
            return true;
        }
        let first_item = arr[0];
        return arr.iter().all(|&item| item == first_item);
    }
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut result: usize = 0;
        let mut used_numbers: HashSet<u64> = HashSet::new();
        let mut ranges: Vec<(u64, u64)> = Vec::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            for range in line.split(",") {
                let tokens = range.split("-").collect::<Vec<&str>>();
                let left_side_of_range = tokens[0].parse::<u64>().unwrap();
                let right_side_of_range = tokens[1].parse::<u64>().unwrap();
                ranges.push((left_side_of_range, right_side_of_range));
            }
        }

        for (left_side, right_side) in ranges {
            for i in left_side..right_side+1 {
                let num_digits: u64 = ((i as f64).log10() as u64 + 1).try_into().unwrap();
                for num_length in 1..((num_digits / 2) + 1) {
                    if num_digits % num_length != 0 {
                        continue;
                    }
                    let mut nums: Vec::<u64> = Vec::new();
                    for idx in 0..num_digits/num_length {
                        let current_num: u64 = (i / 10u64.pow((num_length * idx).try_into().unwrap())) % 10u64.pow(num_length.try_into().unwrap());
                        nums.push(current_num);
                    }
                    if vec_is_all_same(&nums) && !used_numbers.contains(&i) {
                        println!("Adding {}", i);
                        result += i as usize;
                        used_numbers.insert(i);
                    }
                } 
            }
        }
        Ok(result)
    }
    
    assert_eq!(4174379265, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
