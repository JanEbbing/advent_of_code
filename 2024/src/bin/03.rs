use anyhow::*;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn compute_mul_statement(mul_statement: &str) -> i32 {
        let halves = mul_statement.split(",").collect::<Vec<&str>>();
        let left_side_digits: String = halves[0].chars().filter(|c| c.is_digit(10)).collect();
        let right_side_digits: String = halves[1].chars().filter(|c| c.is_digit(10)).collect();
        let result = left_side_digits.parse::<i32>().unwrap() * right_side_digits.parse::<i32>().unwrap();
        result
    }

    fn compute_multiplications_in_corrupted_string(corrupted_string: &str) -> i32 {
        let re = Regex::new(r"(mul\([0-9][0-9]?[0-9]?,[0-9][0-9]?[0-9]?\))").unwrap();
        let mut result = 0;
        for (_, [mul_statement]) in re.captures_iter(corrupted_string).map(|c| c.extract()) {
            result += compute_mul_statement(mul_statement);
        }
        result
    }

    fn compute_multiplications_in_corrupted_string_with_enabling(corrupted_string: &str) -> i32 {
        let re = Regex::new(r"(mul\([0-9][0-9]?[0-9]?,[0-9][0-9]?[0-9]?\))|(do\(\))|(don't\(\))").unwrap();
        let mut result = 0;
        let mut enabled = true;
        for capture in re.captures_iter(corrupted_string) {
            if let Some(_do_match) = capture.get(2) {
                enabled = true;
            } else if let Some(_dont_match) = capture.get(3) {
                enabled = false;
            } else if let Some(mul_match) = capture.get(1) {
                if enabled {
                    result += compute_mul_statement(mul_match.as_str());
                }
            }
        }
        result
    }

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let mut result = 0;
        for line in reader.lines().map(|l| l.unwrap()) {
            result += compute_multiplications_in_corrupted_string(line.as_str());
        }
        Ok(result)
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let mut result = 0;
        // Concatenate all lines as do/dont instructions influence the next line
        let line = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>().join("");
        result += compute_multiplications_in_corrupted_string_with_enabling(line.as_str());
        Ok(result)
    }
    
    assert_eq!(48, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
