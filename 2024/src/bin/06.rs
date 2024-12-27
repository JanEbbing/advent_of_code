use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn get_next_direction(cur_direction: (i32, i32)) -> (i32, i32) {
        if cur_direction.0 == -1 {
            return (0, 1);
        } else if cur_direction.0 == 1 {
            return (0, -1);
        } else if cur_direction.1 == -1 {
            return (-1, 0);
        } else {
            return (1, 0);
        }
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut map: Vec<Vec<char>> = Vec::new();
        let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            map.push(line.chars().collect());
        }
        let n: i32 = map.len().try_into().unwrap();
        let m: i32 = map[0].len().try_into().unwrap();
        let mut guard_pos: (i32, i32) = (0,0);
        for i in 0..n {
            for j in 0..m {
                if map[i as usize][j as usize] == '^' {
                    guard_pos = (i.try_into().unwrap(), j.try_into().unwrap());
                    break;
                }
            }
        }
        visited_positions.insert(guard_pos);
        let mut direction: (i32, i32) = (-1, 0);
        let mut next_pos = (guard_pos.0 + direction.0, guard_pos.1 + direction.1);
        while next_pos.0 >= 0 && next_pos.0 < n && next_pos.1 >= 0 && next_pos.1 < m {
            while map[next_pos.0 as usize][next_pos.1 as usize] == '#' {
                direction = get_next_direction(direction);
                next_pos = (guard_pos.0 + direction.0, guard_pos.1 + direction.1);
            }
            next_pos = (guard_pos.0 + direction.0, guard_pos.1 + direction.1);
            guard_pos = next_pos;
            next_pos = (guard_pos.0 + direction.0, guard_pos.1 + direction.1);
            visited_positions.insert(guard_pos);
        }

        Ok(visited_positions.len().try_into().unwrap())
    }

    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut map: Vec<Vec<char>> = Vec::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            map.push(line.chars().collect());
        }
        let n: i32 = map.len().try_into().unwrap();
        let m: i32 = map[0].len().try_into().unwrap();
        let mut guard_pos: (i32, i32) = (0,0);
        for i in 0..n {
            for j in 0..m {
                if map[i as usize][j as usize] == '^' {
                    guard_pos = (i.try_into().unwrap(), j.try_into().unwrap());
                    break;
                }
            }
        }
        let starting_guard_pos = guard_pos;
        let starting_direction = (-1, 0);
        let mut direction: (i32, i32);

        let mut result: usize = 0;
        for i in 0..n {
            for j in 0..m {
                // println!("Trying out next point");
                if map[i as usize][j as usize] != '.' {
                    continue;
                }
                map[i as usize][j as usize] = '#';

                let mut visited_positions_and_direction: HashSet<(i32, i32, i32, i32)> = HashSet::new();
                direction = starting_direction;
                guard_pos = starting_guard_pos;
                let mut next_pos = (guard_pos.0 + direction.0, guard_pos.1 + direction.1);
                visited_positions_and_direction.insert((guard_pos.0, guard_pos.1, direction.0, direction.1));
                while next_pos.0 >= 0 && next_pos.0 < n && next_pos.1 >= 0 && next_pos.1 < m {
                    while map[next_pos.0 as usize][next_pos.1 as usize] == '#' {
                        direction = get_next_direction(direction);
                        next_pos = (guard_pos.0 + direction.0, guard_pos.1 + direction.1);
                    }
                    next_pos = (guard_pos.0 + direction.0, guard_pos.1 + direction.1);
                    guard_pos = next_pos;
                    next_pos = (guard_pos.0 + direction.0, guard_pos.1 + direction.1);
                    if !visited_positions_and_direction.contains(&(guard_pos.0, guard_pos.1, direction.0, direction.1)) {
                        visited_positions_and_direction.insert((guard_pos.0, guard_pos.1, direction.0, direction.1));
                    } else {
                        result += 1;
                        break;
                    }
                }


                map[i as usize][j as usize] = '.';
            }
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
