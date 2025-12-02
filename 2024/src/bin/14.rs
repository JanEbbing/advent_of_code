use anyhow::*;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "14";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R, width: i32, height: i32) -> Result<usize> {
        let mut robot_positions_and_velocities: Vec<(i32,i32,i32,i32)> = Vec::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            let tokens: Vec<&str> = line.split(" ").collect();
            let position_nums = tokens[0].split("p=").collect::<Vec<&str>>()[1].split(",").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            let velocity_nums = tokens[1].split("v=").collect::<Vec<&str>>()[1].split(",").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            robot_positions_and_velocities.push((position_nums[0], position_nums[1], velocity_nums[0], velocity_nums[1]));
        }

        let mut top_left_quadrant_count: usize = 0;
        let mut bottom_left_quadrant_count: usize = 0;
        let mut top_right_quadrant_count: usize = 0;
        let mut bottom_right_quadrant_count: usize = 0;
        let num_iterations: i32 = 100;

        for (pos_x, pos_y, velocity_x, velocity_y) in robot_positions_and_velocities {
            let mut pos_x_in_future = pos_x + num_iterations * velocity_x;
            let mut pos_y_in_future = pos_y + num_iterations * velocity_y;
            if pos_x_in_future < 0 {
                pos_x_in_future -= width * (pos_x_in_future/width - 1);
            }
            if pos_y_in_future < 0 {
                pos_y_in_future -= height * (pos_y_in_future/height - 1);
            }
            pos_x_in_future = pos_x_in_future % width;
            pos_y_in_future = pos_y_in_future % height;
            if pos_x_in_future < width/2 && pos_y_in_future < height/2 {
                bottom_left_quadrant_count += 1;
            } else if pos_x_in_future > width/2 && pos_y_in_future < height/2 {
                top_left_quadrant_count += 1;
            } else if pos_x_in_future < width/2 && pos_y_in_future > height/2 {
                bottom_right_quadrant_count += 1;
            } else if pos_x_in_future > width/2 && pos_y_in_future > height/2 {
                top_right_quadrant_count += 1;
            }
        }

        Ok(top_left_quadrant_count * bottom_left_quadrant_count * top_right_quadrant_count * bottom_right_quadrant_count)
    }

    assert_eq!(12, part1(BufReader::new(TEST.as_bytes()), 11, 7)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 101, 103)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R, width: i32, height: i32) -> Result<usize> {
        let mut robot_positions_and_velocities: Vec<(i32,i32,i32,i32)> = Vec::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            let tokens: Vec<&str> = line.split(" ").collect();
            let position_nums = tokens[0].split("p=").collect::<Vec<&str>>()[1].split(",").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            let velocity_nums = tokens[1].split("v=").collect::<Vec<&str>>()[1].split(",").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            robot_positions_and_velocities.push((position_nums[0], position_nums[1], velocity_nums[0], velocity_nums[1]));
        }

        let max_iterations: i32 = 10000;
        let mut result: usize = 0;

        for i in 0i32..max_iterations {
            let mut x_positions_to_y_positions: HashMap<i32, Vec<i32>> = HashMap::new();
            let mut all_positions: Vec<(i32, i32)> = Vec::new();
            for (pos_x, pos_y, velocity_x, velocity_y) in &robot_positions_and_velocities {
                let mut pos_x_in_future = pos_x + i * velocity_x;
                let mut pos_y_in_future = pos_y + i * velocity_y;
                if pos_x_in_future < 0 {
                    pos_x_in_future -= width * (pos_x_in_future/width - 1);
                }
                if pos_y_in_future < 0 {
                    pos_y_in_future -= height * (pos_y_in_future/height - 1);
                }
                pos_x_in_future = pos_x_in_future % width;
                pos_y_in_future = pos_y_in_future % height;
                match x_positions_to_y_positions.entry(pos_x_in_future) {
                    Entry::Vacant(e) => { e.insert(vec![pos_y_in_future]); }
                    Entry::Occupied(mut e) => { e.get_mut().push(pos_y_in_future); }
                }
                all_positions.push((pos_x_in_future, pos_y_in_future));
            }
            let mut max_sequence_length: usize = 0;
            for y_positions in x_positions_to_y_positions.values_mut() {
                y_positions.sort();
                for i in 0..y_positions.len() {
                    let mut j = i;
                    let mut cur_sequence_length = 1;
                    while j+1 < y_positions.len() && y_positions[j] == y_positions[j+1] - 1 {
                        j += 1;
                        cur_sequence_length += 1;
                    }
                    max_sequence_length = cmp::max(max_sequence_length, cur_sequence_length);
                }
            }
            if max_sequence_length >= (height/10).try_into().unwrap() {
                println!("Generation {}", i);
                result = i as usize;
                for j in 0..height {
                    for i in 0..width {
                        if all_positions.contains(&(i as i32, j as i32)) {
                            print!("1");
                        } else {
                            print!(".");
                        }
                    }
                    print!("\n");
                }
                print!("\n\n\n");
            }
        }
        Ok(result)
    }
    
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()), 11, 7)?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file, 101, 103)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
