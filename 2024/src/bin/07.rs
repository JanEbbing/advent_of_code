use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut equations: Vec<(usize, Vec<usize>)> = Vec::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            let tokens: Vec<&str> = line.split(": ").collect();
            let eq_result: usize = tokens[0].parse::<usize>().unwrap();
            equations.push((eq_result, tokens[1].split(" ").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>()));
        }

        let mut result: usize = 0;
        for (eq_result, eq_values) in equations {
            for variation in 0..2usize.checked_pow((eq_values.len()-1).try_into().unwrap()).expect("Input by AOC should be solvable") {
                let mut variation_result = eq_values[0];
                for i in 1..eq_values.len() {
                    if ((variation >> (eq_values.len() - 1 - i)) & 1) == 1 {
                        variation_result *= eq_values[i];
                    } else {
                        variation_result += eq_values[i];
                    }
                }
                if variation_result == eq_result {
                    result += eq_result;
                    break;
                }
            }
        }
        Ok(result)
    }

    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
   
    fn eq_is_possible_to_solve_part2(eq_result: usize, eq_values: &Vec<usize>, cur_index: usize, acc_value: usize) -> bool {
        if cur_index == eq_values.len() {
            return eq_result == acc_value;
        }
        let concatenated_val = acc_value * 10usize.checked_pow(eq_values[cur_index].checked_ilog10().expect("AOC should be solvable") + 1).expect("AOC should be solvable") + eq_values[cur_index];
        return eq_is_possible_to_solve_part2(eq_result, eq_values, cur_index+1, acc_value * eq_values[cur_index]) || eq_is_possible_to_solve_part2(eq_result, eq_values, cur_index+1, acc_value + eq_values[cur_index]) || eq_is_possible_to_solve_part2(eq_result, eq_values, cur_index+1, concatenated_val);
    }
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut equations: Vec<(usize, Vec<usize>)> = Vec::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            let tokens: Vec<&str> = line.split(": ").collect();
            let eq_result: usize = tokens[0].parse::<usize>().unwrap();
            equations.push((eq_result, tokens[1].split(" ").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>()));
        }

        let mut result: usize = 0;
        for (eq_result, eq_values) in equations {
            if eq_is_possible_to_solve_part2(eq_result, &eq_values, 1, eq_values[0]) {
                result += eq_result;
            }
        }
        Ok(result)
    }
    
    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
