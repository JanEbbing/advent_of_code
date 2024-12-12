use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut map: Vec<Vec<u32>> = Vec::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            map.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
        }
        let n: usize = map.len();
        let m: usize = map[0].len();
        let mut trailhead_positions: Vec<(usize, usize)> = Vec::new();
        for i in 0..n {
            for j in 0..m {
                if map[i][j] == 0 {
                    trailhead_positions.push((i, j));
                }
            }
        }
        let mut result: usize = 0;
        for (trailhead_x, trailhead_y) in trailhead_positions {
            let mut visited: Vec<Vec<bool>> = Vec::new();
            for i in 0..n {
                let mut visited_row: Vec<bool> = Vec::new();
                for j in 0..m {
                    visited_row.push(false);
                }
                visited.push(visited_row);
            }
            let mut cur_nodes: Vec<(usize, usize)> = vec![(trailhead_x, trailhead_y)];
            let mut trailhead_score: usize = 0;
            while !cur_nodes.is_empty() {
                let Some((cur_x, cur_y)) = cur_nodes.pop() else { break; };
                if visited[cur_x][cur_y] {
                    continue;
                } else {
                    visited[cur_x][cur_y] = true;
                }
                if map[cur_x][cur_y] == 9 {
                    trailhead_score += 1;
                }
                let cur_height = map[cur_x][cur_y];
                if cur_x > 0 && map[cur_x-1][cur_y] == cur_height + 1 {
                    cur_nodes.push((cur_x-1, cur_y));
                }
                if cur_x < n-1 && map[cur_x+1][cur_y] == cur_height + 1 {
                    cur_nodes.push((cur_x+1, cur_y));
                }
                if cur_y > 0 && map[cur_x][cur_y-1] == cur_height + 1 {
                    cur_nodes.push((cur_x, cur_y-1));
                }
                if cur_y < m-1 && map[cur_x][cur_y+1] == cur_height + 1 {
                    cur_nodes.push((cur_x, cur_y+1));
                }
            }

            result += trailhead_score;
        }

        Ok(result)
    }

    assert_eq!(36, part1(BufReader::new(TEST.as_bytes()))?);

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
