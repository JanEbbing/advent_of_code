use anyhow::*;
use std::fs::File;
use union_find::{UnionFind, UnionBySize, QuickFindUf};
use std::collections::VecDeque;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
// use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

fn compute_largest_3_circuits_result(uf: &mut QuickFindUf::<UnionBySize>, n: usize) -> usize {
    let mut union_sizes: HashMap<usize, usize> = HashMap::new();

    for i in 0..n {
        match union_sizes.entry(uf.find(i)) {
            Entry::Vacant(e) => {
                e.insert_entry(1);
            }
            Entry::Occupied(mut e) => {
                *e.get_mut() += 1;
            }
        }
    }

    let mut circuit_sizes: Vec<usize> = Vec::new();
    for (_, val) in union_sizes.iter() {
        circuit_sizes.push(*val);
    }
    circuit_sizes.sort_by(|a, b| b.cmp(a));
    return circuit_sizes[0] * circuit_sizes[1] * circuit_sizes[2];
}

fn is_connected(box_id1: usize, box_id2: usize, uf: &mut QuickFindUf::<UnionBySize>) -> bool {
    return uf.find(box_id1) == uf.find(box_id2);
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R, num_connections_to_make: usize) -> Result<usize> {
        let mut box_positions: Vec<(f32, f32, f32)> = Vec::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            let cur_line_nums: Vec<f32> = line.split(",").map(|tok| tok.parse::<f32>().unwrap()).collect();
            box_positions.push((cur_line_nums[0], cur_line_nums[1], cur_line_nums[2]));
        }

        let num_boxes: usize = box_positions.len();
        let mut connections: Vec<(usize, usize)> = Vec::new();
        let mut uf = QuickFindUf::<UnionBySize>::new(num_boxes);
        let mut box_distances: VecDeque<(f32, usize, usize)> = VecDeque::new();
        for i in 0..num_boxes {
            for j in i+1..num_boxes {
                box_distances.push_back((((box_positions[i].0 - box_positions[j].0).powi(2) + (box_positions[i].1 - box_positions[j].1).powi(2) + (box_positions[i].2 - box_positions[j].2).powi(2)).sqrt(), i, j))
            }
        }
        box_distances.make_contiguous().sort_by(|d, e| d.0.partial_cmp(&e.0).unwrap());
        for _ in 0..num_connections_to_make {
            let (_, circuit1, circuit2): (f32, usize, usize) = box_distances.pop_front().expect("AOC input should be valid");
            // while is_connected(circuit1, circuit2, &reachability) {
            //     (_, circuit1, circuit2) = box_distances.pop_front().expect("AOC input should be valid");
            // }
            if is_connected(circuit1, circuit2, &mut uf) {
                continue;
            }

            connections.push((circuit1, circuit2));
            uf.union(circuit1, circuit2);
        }

        Ok(compute_largest_3_circuits_result(&mut uf, num_boxes))
    }

    assert_eq!(40, part1(BufReader::new(TEST.as_bytes()), 10)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 1000)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut box_positions: Vec<(f32, f32, f32)> = Vec::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            let cur_line_nums: Vec<f32> = line.split(",").map(|tok| tok.parse::<f32>().unwrap()).collect();
            box_positions.push((cur_line_nums[0], cur_line_nums[1], cur_line_nums[2]));
        }

        let num_boxes: usize = box_positions.len();
        let mut connected_boxes: usize = 1;
        let mut uf = QuickFindUf::<UnionBySize>::new(num_boxes);
        let mut box_distances: VecDeque<(f32, usize, usize)> = VecDeque::new();
        for i in 0..num_boxes {
            for j in i+1..num_boxes {
                box_distances.push_back((((box_positions[i].0 - box_positions[j].0).powi(2) + (box_positions[i].1 - box_positions[j].1).powi(2) + (box_positions[i].2 - box_positions[j].2).powi(2)).sqrt(), i, j))
            }
        }
        box_distances.make_contiguous().sort_by(|d, e| d.0.partial_cmp(&e.0).unwrap());
        loop {
            let (_, circuit1, circuit2): (f32, usize, usize) = box_distances.pop_front().expect("AOC input should be valid");
            // while is_connected(circuit1, circuit2, &reachability) {
            //     (_, circuit1, circuit2) = box_distances.pop_front().expect("AOC input should be valid");
            // }
            if is_connected(circuit1, circuit2, &mut uf) {
                continue;
            }

            connected_boxes += 1;
            if connected_boxes == num_boxes {
                return Ok(box_positions[circuit1].0 as usize * box_positions[circuit2].0 as usize);
            }
            uf.union(circuit1, circuit2);
        }
    }
    
    assert_eq!(25272, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
