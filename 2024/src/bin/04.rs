use anyhow::*;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
.S.
SAM
.M.
";
const TEST2: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let mut lines: Vec<String> = Vec::new();
        let re_forward = Regex::new(r"XMAS").unwrap();
        let re_backward = Regex::new(r"SAMX").unwrap();

        let mut result: i32 = 0;
        for line in reader.lines().map(|l| l.unwrap()) {
            result += i32::try_from(re_forward.find_iter(line.as_str()).count()).unwrap();
            result += i32::try_from(re_backward.find_iter(line.as_str()).count()).unwrap();
            lines.push(line);
        }
        let n = lines.len();
        let m = lines[0].len();
        for i in 0..n {
            for j in 0..m {
                // downwards
                if i + 3 < n && lines[i].chars().nth(j) == Some('X') && lines[i+1].chars().nth(j) == Some('M') && lines[i+2].chars().nth(j) == Some('A') && lines[i+3].chars().nth(j) == Some('S') {
                    result += 1;
                }
                if i >= 3 && lines[i].chars().nth(j) == Some('X') && lines[i-1].chars().nth(j) == Some('M') && lines[i-2].chars().nth(j) == Some('A') && lines[i-3].chars().nth(j) == Some('S') {
                    result += 1;
                }
                // diagonal
                if i + 3 < n && j + 3 < m && lines[i].chars().nth(j) == Some('X') && lines[i+1].chars().nth(j+1) == Some('M') && lines[i+2].chars().nth(j+2) == Some('A') && lines[i+3].chars().nth(j+3) == Some('S') {
                    result += 1;
                }
                if i >= 3 && j >= 3 && lines[i].chars().nth(j) == Some('X') && lines[i-1].chars().nth(j-1) == Some('M') && lines[i-2].chars().nth(j-2) == Some('A') && lines[i-3].chars().nth(j-3) == Some('S') {
                    result += 1;
                }
                if i + 3 < n && j >= 3 && lines[i].chars().nth(j) == Some('X') && lines[i+1].chars().nth(j-1) == Some('M') && lines[i+2].chars().nth(j-2) == Some('A') && lines[i+3].chars().nth(j-3) == Some('S') {
                    result += 1;
                }
                if i >= 3 && j + 3 < m && lines[i].chars().nth(j) == Some('X') && lines[i-1].chars().nth(j+1) == Some('M') && lines[i-2].chars().nth(j+2) == Some('A') && lines[i-3].chars().nth(j+3) == Some('S') {
                    result += 1;
                }
            }
        }
        Ok(result)
    }

    assert_eq!(18, part1(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let mut lines: Vec<String> = Vec::new();
        let re_forward = Regex::new(r"MAS").unwrap();
        let re_backward = Regex::new(r"SAM").unwrap();
        let mut row: usize = 0;
        let mut horizontal_captures: Vec<(usize, usize)> = Vec::new();
        let mut vertical_captures: HashSet<(usize, usize)> = HashSet::new();
        let mut diagonal_1_captures: Vec<(usize, usize)> = Vec::new();
        let mut diagonal_2_captures: HashSet<(usize, usize)> = HashSet::new();

        for line in reader.lines().map(|l| l.unwrap()) {
            let line_captures: Vec<(usize, usize)> = re_forward.captures_iter(&line)
                .chain(re_backward.captures_iter(&line))
                .map(|c| (row, c.get(0).unwrap().start()))
                .collect();
            horizontal_captures.extend(line_captures);

            lines.push(line);
            row += 1;
        }
        let n = lines.len();
        let m = lines[0].len();
        for i in 0..n {
            for j in 0..m {
                if i + 2 < n && lines[i].chars().nth(j) == Some('M') && lines[i+1].chars().nth(j) == Some('A') && lines[i+2].chars().nth(j) == Some('S') {
                    vertical_captures.insert((i,j));
                }
                if i >= 2 && lines[i].chars().nth(j) == Some('M') && lines[i-1].chars().nth(j) == Some('A') && lines[i-2].chars().nth(j) == Some('S') {
                    vertical_captures.insert((i-2,j));
                }
                if i + 2 < n && j + 2 < m && lines[i].chars().nth(j) == Some('M') && lines[i+1].chars().nth(j+1) == Some('A') && lines[i+2].chars().nth(j+2) == Some('S') {
                    diagonal_1_captures.push((i,j));
                }
                if i >= 2 && j >= 2 && lines[i].chars().nth(j) == Some('M') && lines[i-1].chars().nth(j-1) == Some('A') && lines[i-2].chars().nth(j-2) == Some('S') {
                    diagonal_1_captures.push((i-2,j-2));
                }
                if i + 2 < n && j >= 2 && lines[i].chars().nth(j) == Some('M') && lines[i+1].chars().nth(j-1) == Some('A') && lines[i+2].chars().nth(j-2) == Some('S') {
                    diagonal_2_captures.insert((i,j));
                }
                if i >= 2 && j + 2 < m && lines[i].chars().nth(j) == Some('M') && lines[i-1].chars().nth(j+1) == Some('A') && lines[i-2].chars().nth(j+2) == Some('S') {
                    diagonal_2_captures.insert((i-2,j+2));
                }
            }
        }
        let mut result: i32 = 0;
        // for (row, col) in horizontal_captures {
        //     if row >= 1 && col+1 < m && vertical_captures.contains(&(row-1,col+1)) {
        //         result += 1;
        //     }
        // }
        for (row, col) in diagonal_1_captures {
            if col < m - 2 && diagonal_2_captures.contains(&(row,col+2)) {
                result += 1;
            }
        }
        Ok(result)
    }

    assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    assert_eq!(9, part2(BufReader::new(TEST2.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
