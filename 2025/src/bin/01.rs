use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut dial_position: i32 = 50;
        let mut result: usize = 0;
        for line in reader.lines().map(|l| l.unwrap()) {
            let move_amount: i32 = line.chars().skip(1).collect::<String>().parse::<i32>().unwrap();
            let move_direction: i32 = if line.chars().take(1).collect::<String>() == "L" { -1 } else { 1 };
            dial_position += move_direction * move_amount;
            dial_position = dial_position % 100;
            if dial_position == 0 {
                result += 1;
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
        let mut dial_position: i32 = 50;
        let mut result: usize = 0;
        for line in reader.lines().map(|l| l.unwrap()) {
            let sign_before = dial_position.signum();
            let move_amount: i32 = line.chars().skip(1).collect::<String>().parse::<i32>().unwrap();
            let move_direction: i32 = if line.chars().take(1).collect::<String>() == "L" { -1 } else { 1 };
            dial_position += move_direction * move_amount;
            let sign_after = dial_position.signum();
            let num_times_dial_pointed_at_zero: usize = (((dial_position / 100).abs()) + if dial_position == 0 { 1 } else { 0 } + if (sign_before == 1 && sign_after == -1) || (sign_before == -1 && sign_after == 1) { 1 } else { 0 }).try_into().unwrap();
            result += num_times_dial_pointed_at_zero;
            dial_position = dial_position % 100;
        }

        Ok(result)
    }
    
    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
