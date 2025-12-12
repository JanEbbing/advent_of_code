use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

enum Operand {
    Addition,
    Multiplication
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut number_grid: Vec<Vec<usize>> = Vec::new();
        let mut operators: Vec<Operand> = Vec::new();
        let mut result: usize = 0;
        for line in reader.lines().map(|l| l.unwrap()) {
            if line.starts_with('*') || line.starts_with('+') {
                operators = line.split(" ").filter_map(|tok| match tok {
                    "+" => Some(Operand::Addition),
                    "*" => Some(Operand::Multiplication),
                    _ => None,
                }).collect();
            } else {
                number_grid.push(line.split(" ").filter_map(|tok| tok.parse::<usize>().ok()).collect());
            }
        }
        for i in 0..operators.len() {
            let mut interim_result: usize = match operators[i] {
                Operand::Addition => 0,
                Operand::Multiplication => 1,
            };
            for number_line in &number_grid {
                interim_result = match operators[i] {
                    Operand::Addition => interim_result + number_line[i],
                    Operand::Multiplication => interim_result * number_line[i],
                }
            }
            result += interim_result;
        }
        Ok(result)
    }

    assert_eq!(4277556, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut input_grid: Vec<Vec<char>> = Vec::new();
        let mut result: usize = 0;
        for line in reader.lines().map(|l| l.unwrap()) {
            input_grid.push(line.chars().collect());
        }
        let n = input_grid.len();
        let m = input_grid[n-1].len();
        let mut i: usize = 0;
        while i < m-1 {
            let mut interim_result: usize = match input_grid[n-1][i] {
                '+' => 0,
                '*' => 1,
                _ => panic!("Invalid input_grid value")
            };
            let mut j = i+1;
            while j < m-2 && input_grid[n-1][j+1] == ' ' {
                j += 1;
            }
            // ugly index hack - last column does not have an extra column of spaces
            if j == m-2 {
                j = m;
            }

            for col_ind in i..j {
                let mut cur_num: usize = 0;
                for row_ind in 0..n-1 {
                    if input_grid[row_ind][col_ind] != ' ' {
                        cur_num = cur_num * 10 + input_grid[row_ind][col_ind].to_digit(10).unwrap() as usize;
                    }
                }
                interim_result = match input_grid[n-1][i] {
                    '+' => interim_result + cur_num,
                    '*' => interim_result * cur_num,
                    _ => panic!("Invalid input_grid value")
                };
            }
            result += interim_result;
            i = j+1;
        }
        Ok(result)
    }
    
    assert_eq!(3263827, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
