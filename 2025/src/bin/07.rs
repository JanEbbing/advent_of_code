use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut result: usize = 0;
        let mut beams: VecDeque<(usize, usize)> = VecDeque::new();
        let mut splitter_rows_per_column: Vec<Vec<usize>> = Vec::new();
        let mut grid: Vec<Vec<char>> = Vec::new();
        let mut deactivated_splitters_map: Vec<Vec<bool>> = Vec::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            grid.push(line.chars().collect());
        }
        beams.push_back((0, grid[0].iter().position(|c| *c == 'S').unwrap()));
        for i in 0..grid[0].len() {
            let mut splitter_rows_in_cur_column: Vec<usize> = Vec::new();
            for j in 0..grid.len() {
                if grid[j][i] != '^' {
                    continue;
                }

                splitter_rows_in_cur_column.push(j);
            }
            splitter_rows_per_column.push(splitter_rows_in_cur_column);
        }
        for i in 0..grid.len() {
            deactivated_splitters_map.push(vec![false; grid[i].len()]);
        }

        while !beams.is_empty() {
            let cur_beam = beams.pop_front().unwrap();
            let candidate_splitters: Vec<&usize> = splitter_rows_per_column[cur_beam.1].iter().filter(|row_ind| cur_beam.0 < **row_ind).collect();
            if !candidate_splitters.is_empty() {
                if !deactivated_splitters_map[*candidate_splitters[0]][cur_beam.1] {
                    result += 1;
                    deactivated_splitters_map[*candidate_splitters[0]][cur_beam.1] = true;
                    if cur_beam.1 > 0 {
                        beams.push_back((*candidate_splitters[0], cur_beam.1-1));
                    }
                    if cur_beam.1 < grid[0].len() - 1 {
                        beams.push_back((*candidate_splitters[0], cur_beam.1+1));
                    }
                }
            }
        }
        Ok(result)
    }

    assert_eq!(21, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut splitter_rows_per_column: Vec<Vec<usize>> = Vec::new();
        let mut ways_per_splitter: Vec<Vec<usize>> = Vec::new();
        let mut grid: Vec<Vec<char>> = Vec::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            grid.push(line.chars().collect());
        }
        let n = grid.len();
        let m = grid[0].len();
        let starting_beam_spot: (usize, usize) = (0, grid[0].iter().position(|c| *c == 'S').unwrap());
        for i in 0..m {
            let mut splitter_rows_in_cur_column: Vec<usize> = Vec::new();
            for j in 0..n {
                if grid[j][i] != '^' {
                    continue;
                }
                splitter_rows_in_cur_column.push(j);
            }
            splitter_rows_per_column.push(splitter_rows_in_cur_column);
        }
        for i in 0..n {
            ways_per_splitter.push(vec![0usize; grid[i].len()]);
        }

        for i in 0..n {
            for j in 0..grid[n-1-i].len() {
                if grid[n-1-i][j] != '^' {
                    continue;
                }
                if ways_per_splitter[n-1-i][j] != 0 {
                    continue;
                }

                let mut ways_for_this_splitter: usize = 0;
                if j > 0 {
                    let candidate_splitters: Vec<&usize> = splitter_rows_per_column[j-1].iter().filter(|row_ind| n-1-i < **row_ind).collect();
                    if candidate_splitters.is_empty() {
                        ways_for_this_splitter += 1;
                    } else {
                        ways_for_this_splitter += ways_per_splitter[*candidate_splitters[0]][j-1];
                    }

                }
                if j < grid[n-1-i].len() - 1 {
                    let candidate_splitters: Vec<&usize> = splitter_rows_per_column[j+1].iter().filter(|row_ind| n-1-i < **row_ind).collect();
                    if candidate_splitters.is_empty() {
                        ways_for_this_splitter += 1;
                    } else {
                        ways_for_this_splitter += ways_per_splitter[*candidate_splitters[0]][j+1];
                    }
                }
                ways_per_splitter[n-1-i][j] = ways_for_this_splitter;
            }
        }
        let candidate_splitters: Vec<&usize> = splitter_rows_per_column[starting_beam_spot.1].iter().filter(|row_ind| starting_beam_spot.0 < **row_ind).collect();
        let result: usize = if candidate_splitters.is_empty() { 0 } else { ways_per_splitter[*candidate_splitters[0]][starting_beam_spot.1] };
        
        Ok(result)
    }
    
    assert_eq!(40, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
