use anyhow::*;
use std::collections::{HashMap,HashSet};
use std::collections::hash_map::Entry;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut map: Vec<Vec<char>> = Vec::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            map.push(line.chars().collect());
        }
        let n: i32 = map.len().try_into().unwrap();
        let m: i32 = map[0].len().try_into().unwrap();
        let mut frequency_to_antenna_positions: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
        for i in 0i32..n {
            for j in 0i32..m {
                let frequency = map[i as usize][j as usize];
                if frequency != '.' {
                    match frequency_to_antenna_positions.entry(frequency) {
                        Entry::Vacant(e) => { e.insert(vec![(i, j)]); }
                        Entry::Occupied(mut e) => { e.get_mut().push((i, j)); }
                    }
                }
            }
        }

        let mut antinode_locations: HashSet<(i32, i32)> = HashSet::new();
        for frequency in frequency_to_antenna_positions.keys() {
            let cur_antennas = &frequency_to_antenna_positions[frequency];
            let num_antennas: i32 = (cur_antennas.len()).try_into().unwrap();
            for i in 0i32..num_antennas {
                for j in i+1..num_antennas {
                    let candidate_1 = (cur_antennas[i as usize].0 + 2 * (cur_antennas[j as usize].0 - cur_antennas[i as usize].0), cur_antennas[i as usize].1 + 2 * (cur_antennas[j as usize].1 - cur_antennas[i as usize].1));
                    let candidate_2 = (cur_antennas[j as usize].0 + 2 * (cur_antennas[i as usize].0 - cur_antennas[j as usize].0), cur_antennas[j as usize].1 + 2 * (cur_antennas[i as usize].1 - cur_antennas[j as usize].1));
                    if candidate_1.0 >= 0 && candidate_1.0 < n && candidate_1.1 >= 0 && candidate_1.1 < m {
                        antinode_locations.insert(candidate_1);
                    }
                    if candidate_2.0 >= 0 && candidate_2.0 < n && candidate_2.1 >= 0 && candidate_2.1 < m {
                        antinode_locations.insert(candidate_2);
                    }
                }
            }
        }
        Ok(antinode_locations.len())
    }

    assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);

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
        let mut frequency_to_antenna_positions: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
        for i in 0i32..n {
            for j in 0i32..m {
                let frequency = map[i as usize][j as usize];
                if frequency != '.' {
                    match frequency_to_antenna_positions.entry(frequency) {
                        Entry::Vacant(e) => { e.insert(vec![(i, j)]); }
                        Entry::Occupied(mut e) => { e.get_mut().push((i, j)); }
                    }
                }
            }
        }

        let mut antinode_locations: HashSet<(i32, i32)> = HashSet::new();
        for frequency in frequency_to_antenna_positions.keys() {
            let cur_antennas = &frequency_to_antenna_positions[frequency];
            let num_antennas: i32 = (cur_antennas.len()).try_into().unwrap();
            for i in 0i32..num_antennas {
                for j in i+1..num_antennas {
                    antinode_locations.insert(cur_antennas[i as usize]);
                    antinode_locations.insert(cur_antennas[j as usize]);

                    let mut c1_multiplier = 2;
                    let mut c2_multiplier = 2;
                    let mut candidate_1 = (cur_antennas[i as usize].0 + c1_multiplier * (cur_antennas[j as usize].0 - cur_antennas[i as usize].0), cur_antennas[i as usize].1 + c1_multiplier * (cur_antennas[j as usize].1 - cur_antennas[i as usize].1));
                    let mut candidate_2 = (cur_antennas[j as usize].0 + c2_multiplier * (cur_antennas[i as usize].0 - cur_antennas[j as usize].0), cur_antennas[j as usize].1 + c2_multiplier * (cur_antennas[i as usize].1 - cur_antennas[j as usize].1));
                    while candidate_1.0 >= 0 && candidate_1.0 < n && candidate_1.1 >= 0 && candidate_1.1 < m {
                        antinode_locations.insert(candidate_1);
                        c1_multiplier += 1;
                        candidate_1 = (cur_antennas[i as usize].0 + c1_multiplier * (cur_antennas[j as usize].0 - cur_antennas[i as usize].0), cur_antennas[i as usize].1 + c1_multiplier * (cur_antennas[j as usize].1 - cur_antennas[i as usize].1));
                    }
                    while candidate_2.0 >= 0 && candidate_2.0 < n && candidate_2.1 >= 0 && candidate_2.1 < m {
                        antinode_locations.insert(candidate_2);
                        c2_multiplier += 1;
                        candidate_2 = (cur_antennas[j as usize].0 + c2_multiplier * (cur_antennas[i as usize].0 - cur_antennas[j as usize].0), cur_antennas[j as usize].1 + c2_multiplier * (cur_antennas[i as usize].1 - cur_antennas[j as usize].1));
                    }
                }
            }
        }
        Ok(antinode_locations.len())
    }
    
    assert_eq!(34, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
