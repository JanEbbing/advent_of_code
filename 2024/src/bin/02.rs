use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let mut reports: Vec<Vec<i32>> = Vec::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            reports.push(line.split(" ").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>());
        }
        let mut result = 0;
        for report in reports {
            if report.len() < 2 {
                result += 1;
                continue;
            }
            let decreasing: bool = report[0] > report[1];
            let mut report_is_safe: bool = true;
            for i in 1..report.len() {
                let mut cur_difference = report[i] - report[i-1];
                if decreasing {
                    cur_difference = -cur_difference;
                }
                if cur_difference < 1 || cur_difference > 3 {
                    report_is_safe = false;
                    break;
                }
            }
            if report_is_safe {
                result += 1;
            }
        }
        return Ok(result);
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
