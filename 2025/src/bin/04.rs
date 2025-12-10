use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

fn get_valid_neighbors(n: usize, m: usize, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    let mut x_offsets: Vec<i32> = Vec::new();
    let mut y_offsets: Vec<i32> = Vec::new();
    x_offsets.push(0);
    y_offsets.push(0);
    if x > 0 {
        x_offsets.push(-1);
    }
    if y > 0 {
        y_offsets.push(-1);
    }
    if x < n - 1 {
        x_offsets.push(1);
    }
    if y < m - 1 {
        y_offsets.push(1);
    }
    for x_offset in x_offsets {
        for y_offset in &y_offsets {
            if x_offset != 0 || *y_offset != 0i32 {
                result.push((((x as i32) + x_offset) as usize, ((y as i32) + *y_offset) as usize));
            }
        }
    }
    return result;
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut result: usize = 0;
        let mut grid: Vec<Vec<char>> = Vec::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            grid.push(line.chars().collect());
        }
        let n = grid.len();
        let m = grid[0].len();
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == '.' {
                    continue;
                }
                let valid_coords: Vec<(usize, usize)> = get_valid_neighbors(n, m, i, j);
                let num_accessible_rolls: usize = valid_coords.iter().map(|(x, y)| if grid[*x][*y] == '@' { 1 } else { 0 }).sum();
                if num_accessible_rolls < 4 {
                    result += 1;
                }
            }
        }
        Ok(result)
    }

    assert_eq!(13, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut result: usize = 0;
        let mut grid: Vec<Vec<char>> = Vec::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            grid.push(line.chars().collect());
        }
        let n = grid.len();
        let m = grid[0].len();
        let mut cur_grid = grid.clone();
        let mut roll_indices_to_remove: Vec<(usize, usize)>;
        while {
            roll_indices_to_remove = Vec::new();
            for i in 0..n {
                for j in 0..m {
                    if cur_grid[i][j] == '.' {
                        continue;
                    }
                    let valid_coords: Vec<(usize, usize)> = get_valid_neighbors(n, m, i, j);
                    let num_accessible_rolls: usize = valid_coords.iter().map(|(x, y)| if cur_grid[*x][*y] == '@' { 1 } else { 0 }).sum();
                    if num_accessible_rolls < 4 {
                        roll_indices_to_remove.push((i, j));
                    }
                }
            }
            for (a, b) in &roll_indices_to_remove {
                cur_grid[*a][*b] = '.';
            }
            result += roll_indices_to_remove.len();
            !roll_indices_to_remove.is_empty()
        } {}
        Ok(result)
    }
    
    assert_eq!(43, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
