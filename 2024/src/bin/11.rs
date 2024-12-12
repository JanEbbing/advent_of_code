use anyhow::*;
use cached::proc_macro::cached;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
125 17
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn simulate_blink(stones: Vec<i64>) -> Vec<i64> {
        let mut result: Vec<i64> = Vec::new();
        for stone in stones {
            if stone == 0 {
                result.push(1);
            } else {
                let stone_str = stone.to_string();
                if stone_str.len() % 2 == 0 {
                    let (first_half, second_half) = stone_str.split_at(stone_str.len() / 2);
                    result.push(first_half.parse::<i64>().unwrap());
                    result.push(second_half.parse::<i64>().unwrap());
                } else {
                    result.push(stone * 2024);
                }
            }
        }

        result
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut stones: Vec<i64> = Vec::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            stones = line.split(" ").map(|n| n.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        }
        for _i in 0..25 {
            stones = simulate_blink(stones);
        }
        Ok(stones.len())
    }

    assert_eq!(55312, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    #[cached]
    fn get_num_element_after_blinks(stone: i64, num_blinks: i32) -> usize {
        if num_blinks == 0 {
            return 1;
        }
        if stone == 0 {
            return get_num_element_after_blinks(1, num_blinks - 1);
        } else {
            let stone_str = stone.to_string();
            if stone_str.len() % 2 == 0 {
                let (first_half, second_half) = stone_str.split_at(stone_str.len() / 2);
                return get_num_element_after_blinks(first_half.parse::<i64>().unwrap(), num_blinks - 1) + get_num_element_after_blinks(second_half.parse::<i64>().unwrap(), num_blinks - 1);
            } else {
                return get_num_element_after_blinks(stone * 2024, num_blinks - 1);
            }
        }
    }


    fn get_num_elements_after_blinks(stones: Vec<i64>, num_blinks: i32) -> usize {
        let mut result: usize = 0;
        for stone in stones {
            result += get_num_element_after_blinks(stone, num_blinks);
        }

        result
    }
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut stones: Vec<i64> = Vec::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            stones = line.split(" ").map(|n| n.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        }
        let result = get_num_elements_after_blinks(stones, 75);
        Ok(result)
    }

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
