use anyhow::*;
use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";


fn check_point(point1_id: usize, top_left_x: i64, top_left_y: i64, bottom_right_x: i64, bottom_right_y: i64, num_points: usize, points: &Vec<(i64, i64)>) -> bool {
    for j in 0..num_points - 1 {
        let potential_first_point_id = (point1_id + j) % num_points;
        let potential_second_point_id = (point1_id + j + 1) % num_points;
        
        if points[potential_first_point_id].1 == points[potential_second_point_id].1 && points[potential_first_point_id].1 > top_left_y && points[potential_first_point_id].1 < bottom_right_y && ((top_left_x >= points[potential_first_point_id].0 && top_left_x < points[potential_second_point_id].0) || (bottom_right_x <= points[potential_first_point_id].0 && bottom_right_x > points[potential_second_point_id].0)) {
            return true;
        }
        if points[potential_first_point_id].0 == points[potential_second_point_id].0 && points[potential_first_point_id].0 > top_left_x && points[potential_first_point_id].0 < bottom_right_x && ((top_left_y >= points[potential_first_point_id].1 && top_left_y < points[potential_second_point_id].1) || (bottom_right_y <= points[potential_first_point_id].1 && bottom_right_y > points[potential_second_point_id].1)) {
            return true;
        }
    }
    return false;
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut points: Vec<(i64, i64)> = Vec::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            let cur_line_nums: Vec<i64> = line.split(",").map(|tok| tok.parse::<i64>().unwrap()).collect();
            points.push((cur_line_nums[0], cur_line_nums[1]));
        }
        let mut max_area: i64 = -1;
        for p1 in &points {
            for p2 in &points {
                if p1.0 < p2.0 && p1.1 < p2.1 {
                    max_area = cmp::max(max_area, (p2.0 - p1.0 + 1) * (p2.1 - p1.1 + 1));
                }
                if p1.0 < p2.0 && p1.1 > p2.1 {
                    max_area = cmp::max(max_area, (p2.0 - p1.0 + 1) * (p1.1 - p2.1 + 1));
                }
            }
        }
        Ok(max_area as usize)
    }
    assert_eq!(50, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut points: Vec<(i64, i64)> = Vec::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            let cur_line_nums: Vec<i64> = line.split(",").map(|tok| tok.parse::<i64>().unwrap()).collect();
            points.push((cur_line_nums[0], cur_line_nums[1]));
        }
        let num_points: usize = points.len();
        let mut max_area: i64 = -1;
        for point1_id in 0..num_points {
            for point2_id in 0..num_points {
                if points[point1_id].0 <= points[point2_id].0 && points[point1_id].1 <= points[point2_id].1 {
                    // top left to bot right
                    let top_left_x = points[point1_id].0;
                    let top_left_y = points[point1_id].1;
                    let bottom_right_x = points[point2_id].0;
                    let bottom_right_y = points[point2_id].1;
                    let cur_area: i64 = (bottom_right_x - top_left_x + 1) * (bottom_right_y - top_left_y  + 1);

                    let violation_found = check_point(point1_id, top_left_x, top_left_y, bottom_right_x, bottom_right_y, num_points, &points);
                    if !violation_found {
                        max_area = cmp::max(max_area, cur_area);
                    }
                }

                if points[point1_id].0 <= points[point2_id].0 && points[point1_id].1 >= points[point2_id].1 {
                    // bottom left to top right
                    // 7,3 and 11,1
                    let top_left_x = points[point1_id].0;
                    let top_left_y = points[point2_id].1;
                    let bottom_right_x = points[point2_id].0;
                    let bottom_right_y = points[point1_id].1;
                    let cur_area: i64 = (bottom_right_x - top_left_x + 1) * (bottom_right_y - top_left_y  + 1);

                    let violation_found = check_point(point1_id, top_left_x, top_left_y, bottom_right_x, bottom_right_y, num_points, &points);
                    if !violation_found {
                        max_area = cmp::max(max_area, cur_area);
                    }
                }
            }
        }
        Ok(max_area as usize)
    }
    
    assert_eq!(24, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
