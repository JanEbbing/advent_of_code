use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::Ordering;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut result: usize = 0;
        for line in reader.lines().map(|l| l.unwrap()) {
            let chars_in_cur_line: Vec<u32> = line.chars().map(|c| c.to_digit(10u32).unwrap()).collect();
            let index_of_max_num: &usize = &chars_in_cur_line.iter().enumerate().max_by(|(ind_a, a), (ind_b, b)| if a.cmp(b) == Ordering::Equal { if ind_a < ind_b { Ordering::Greater } else { Ordering::Less } } else { a.cmp(b) }).map(|(i, _)| i).unwrap();
            let is_last_num_in_row: bool = index_of_max_num + 1 == chars_in_cur_line.len();
            if is_last_num_in_row {
                let index_of_second_largest_num: &usize = &chars_in_cur_line[..*index_of_max_num].iter().enumerate().max_by(|(ind_a, a), (ind_b, b)| if a.cmp(b) == Ordering::Equal { if ind_a < ind_b { Ordering::Greater } else { Ordering::Less } } else { a.cmp(b) }).map(|(i, _)| i).unwrap();
                result += (10 * chars_in_cur_line[*index_of_second_largest_num] + chars_in_cur_line[*index_of_max_num]) as usize;
            } else {
                let mut index_of_second_largest_num: usize = chars_in_cur_line[(*index_of_max_num + 1)..].iter().enumerate().max_by(|(ind_a, a), (ind_b, b)| if a.cmp(b) == Ordering::Equal { if ind_a < ind_b { Ordering::Greater } else { Ordering::Less } } else { a.cmp(b) }).map(|(i, _)| i).unwrap();
                index_of_second_largest_num += index_of_max_num + 1;
                result += (10 * chars_in_cur_line[*index_of_max_num] + chars_in_cur_line[index_of_second_largest_num]) as usize;
            }
        }
        Ok(result)
    }

    assert_eq!(357, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut result: usize = 0;
        let k: usize = 12;
        for line in reader.lines().map(|l| l.unwrap()) {
            let chars_in_cur_line: Vec<u32> = line.chars().map(|c| c.to_digit(10u32).unwrap()).collect();
            // largest number possible to form: Take largest digit from indices 0 to (length - k). Suppose it is at index i
            // Then take largest digit from indices i to (length + 1 - k). Suppose it is at index i'
            // Then take largest digit from indices i' to (length + 2 - k). Etc until k digits taken
            // Sum them all up multiplied with 10^(k-j) with j a running index over the chosen digits

            let mut start_index: usize = 0;
            for i in 0..k {
                let cur_slice = &chars_in_cur_line[start_index..chars_in_cur_line.len() - (k-1) + i];
                let index_of_max_num: usize = cur_slice.iter().enumerate().max_by(|(ind_a, a), (ind_b, b)| if a.cmp(b) == Ordering::Equal { if ind_a < ind_b { Ordering::Greater } else { Ordering::Less } } else { a.cmp(b) }).map(|(i, _)| i).unwrap();
                result += (cur_slice[index_of_max_num] as usize) * usize::pow(10, (k - i - 1).try_into().unwrap());
                start_index += index_of_max_num + 1;
            }
        }

        Ok(result)
    }
    
    assert_eq!(3121910778619, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
