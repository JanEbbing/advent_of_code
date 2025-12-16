use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "12";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut shapes: Vec<Vec<Vec<char>>> = Vec::new();
        let mut cases: Vec<(usize, usize, Vec<usize>)> = Vec::new();
        let mut cur_shape: Vec<Vec<char>> = Vec::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            if line.is_empty() {
                shapes.push(cur_shape);
                cur_shape = Vec::new();
            } else if line.ends_with(":") {}
            else if line.contains("x") {
                let tokens: Vec<&str> = line.split(": ").collect();
                let dims: Vec<usize> = tokens[0].split("x").map(|d| d.parse::<usize>().unwrap()).collect();
                let required_shapes: Vec<usize> = tokens[1].split(" ").map(|r_s| r_s.parse::<usize>().unwrap()).collect();
                cases.push((dims[0], dims[1], required_shapes));
            } else {
                cur_shape.push(line.chars().collect());
            }
        }

        let mut result: usize = 0;
        for case in cases {
            let mut cur_offset_x: usize = 0;
            let mut cur_offset_y: usize = 0;
            let mut success = true;

            for i in 0..case.2.len() {
                let n = shapes[i].len(); // 3 x dim of shape
                let m = shapes[i][0].len(); // 3 y dim of shape
                let num_required = case.2[i]; // 2 shapes req
                if num_required == 0 {
                    continue;
                }

                let max_x_ind = (case.0 / n) * n;
                let max_y_ind = (case.1 / m) * m;

                for _ in 0..num_required {
                    if cur_offset_x + n > case.0 {
                        cur_offset_x = 0;
                        cur_offset_y += m;
                    }

                    if cur_offset_y + m > case.1 {
                        success = false;
                        break;
                    }

                    cur_offset_x += n;
                }
            }
            if success {
                result += 1;
            }
        }
        Ok(result)
    }

    // assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

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
    // // TODO: Set the expected answer for the test input
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
