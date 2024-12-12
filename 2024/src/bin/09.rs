use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
2333133121414131402
";

const TEST2: &str = "\
54321
"; 

const TEST3: &str = "\
17814589183
"; 

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn find_first_empty_block(filesystem: &Vec<(u64, bool, i32)>) -> Option<usize> {
        for i in 0..filesystem.len() {
            if !filesystem[i].1 {
                return Some(i as usize);
            }
        }
        return None;
    }

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        let mut rle: Vec<(u64, bool, i32)> = Vec::new();
        let mut file_id: i32 = 0;
        let mut is_file = true;
        for line in reader.lines().map(|l| l.unwrap()) {
            for c in line.chars() {
                let c = c.to_digit(10).unwrap() as u64;
                if c != 0 {
                    rle.push((c, is_file, if is_file { file_id } else { -1 }));
                }
                if is_file {
                    file_id += 1;
                }
                is_file = !is_file;
            }
        }

        let mut j = rle.len() - 1;
        let mut i = find_first_empty_block(&rle);
        while let Some(index) = i {
            let available_free_space = rle[index].0;
            let last_block_size = rle[j].0;
            if available_free_space == last_block_size {
                rle[index] = (available_free_space, true, rle[j].2);
                rle.pop();
            } else if available_free_space > last_block_size {
                rle.insert(index, (last_block_size, true, rle[j].2));
                rle[index + 1] = (available_free_space - last_block_size, false, -1);
                rle.pop();
            } else {
                rle[index] = (available_free_space, true, rle[j].2);
                rle[j] = (last_block_size - available_free_space, true, rle[j].2);
            }

            j = rle.len() - 1;
            while !rle[j].1 {
                rle.pop();
                j -= 1;
            }
            i = find_first_empty_block(&rle);
        }
        let mut result: u64 = 0;
        let mut cur_index: u64 = 0;
        // println!("{:?}", rle);
        for (size, _is_file, file_id) in rle {
            let n = cur_index + size;
            result += (file_id as u64) * ((cur_index..n).sum::<u64>());
            // let big_square = (n * (n-1)) / 2;
            // let small_square = if cur_index == 0 {0} else { (cur_index * (cur_index-1)) / 2 };
            // result += (file_id as u64) * (big_square - small_square);
            cur_index = n;
        }

        Ok(result)
    }

    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<u64> {
        let mut rle: Vec<(u64, bool, i32)> = Vec::new();
        let mut running_file_id: i32 = 0;
        let mut is_file = true;
        for line in reader.lines().map(|l| l.unwrap()) {
            for c in line.chars() {
                let c = c.to_digit(10).unwrap() as u64;
                if c != 0 {
                    rle.push((c, is_file, if is_file { running_file_id } else { -1 }));
                }
                if is_file {
                    running_file_id += 1;
                }
                is_file = !is_file;
            }
        }

        let mut j = rle.len() - 1;
        while j > 0 {
            if !rle[j].1 {
                j -= 1;
                continue;
            }
            let last_block_size = rle[j].0;
            let mut found = false;
            let mut insertion_index = 0;
            for i in 0..j {
                if !rle[i].1 && rle[i].0 >= last_block_size {
                    found = true;
                    insertion_index = i;
                    break;
                }
            }
            if found {
                rle.insert(insertion_index, (last_block_size, true, rle[j].2));
                j += 1;
                rle[j] = (last_block_size, false, -1);
                if last_block_size == rle[insertion_index+1].0 {
                    rle.remove(insertion_index+1);
                    j -= 1;
                } else {
                    rle[insertion_index+1] = (rle[insertion_index+1].0 - last_block_size, false, -1);
                }
            }
            j -= 1;
        }
        // println!("{:?}", rle);
        let mut result: u64 = 0;
        let mut cur_index: u64 = 0;
        for (size, is_file, file_id) in rle {
            if !is_file {
                cur_index += size;
                continue;
            }
            let n: u64 = cur_index + size;
            result += (file_id as u64) * ((cur_index..n).sum::<u64>());
            cur_index = n;
        }

        Ok(result)
    }
    
    assert_eq!(2858, part2(BufReader::new(TEST.as_bytes()))?);
    assert_eq!(31, part2(BufReader::new(TEST2.as_bytes()))?);
    assert_eq!(994, part2(BufReader::new(TEST3.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
