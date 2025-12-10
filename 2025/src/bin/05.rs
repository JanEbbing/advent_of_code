use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut result: usize = 0;
        let mut fresh_ranges: Vec<(usize, usize)> = Vec::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            match line.find('-') {
                Some(_) => {
                    let tokens: Vec<usize> = line.split("-").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();
                    fresh_ranges.push((tokens[0], tokens[1]));
                },
                None => {
                    if !line.is_empty() {
                        let cur_num = line.parse::<usize>().unwrap();
                        for (range_start, range_end) in &fresh_ranges {
                            if cur_num >= *range_start && cur_num <= *range_end {
                                result += 1;
                                break;
                            }
                        }
                    }
                }
            }
        }
        Ok(result)
    }

    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut result: usize = 0;
        let mut fresh_ranges: Vec<(usize, usize)> = Vec::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            match line.find('-') {
                Some(_) => {
                    let tokens: Vec<usize> = line.split("-").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();
                    fresh_ranges.push((tokens[0], tokens[1]));
                },
                None => {
                    break;
                }
            }
        }
        fresh_ranges.sort();
        let mut previous_range: Option<(usize, usize)> = None;
        for (cur_start, cur_end) in fresh_ranges {
            match previous_range {
                Some((previous_start, previous_end)) => {
                    if cur_start >= previous_start && cur_start <= previous_end {
                        previous_range = Some((cmp::min(previous_start, cur_start), cmp::max(cur_end, previous_end)));
                    } else {
                        result += previous_end - previous_start + 1;
                        previous_range = Some((cur_start, cur_end));
                    }
                },
                None => {
                    previous_range = Some((cur_start, cur_end));
                }
            }
        }
        match previous_range {
            Some((start, end)) => result += end - start + 1,
            None => (),
        } 
        Ok(result)
    }

    assert_eq!(14, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
