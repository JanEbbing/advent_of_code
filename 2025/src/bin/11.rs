use anyhow::*;
use std::fs::File;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

const TEST2: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut node_labels_to_ids: HashMap<String, usize> = HashMap::new();
        let mut edge_strings: Vec<String> = Vec::new();
        let mut edges: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut node_ids_to_num_ways_to_exit: HashMap<usize, usize> = HashMap::new();
        let mut cur_id: usize = 0;
        for line in reader.lines().map(|l| l.unwrap()) {
            let tokens: Vec<&str> = line.split(": ").collect();
            node_labels_to_ids.insert(tokens[0].to_string(), cur_id);
            cur_id += 1;
            edge_strings.push(tokens[1].to_string());
        }
        node_labels_to_ids.insert("out".to_string(), cur_id);
        node_ids_to_num_ways_to_exit.insert(cur_id, 1);
        for i in 0..edge_strings.len() {
            edges.insert(i, edge_strings[i].split(" ").map(|t_l| node_labels_to_ids[t_l]).collect());
        }
        let mut target_ids: Vec<usize> = vec![cur_id];

        while !node_ids_to_num_ways_to_exit.contains_key(&node_labels_to_ids["you"]) {
            for (source_id, edge_target_ids) in edges.iter() {
                if target_ids.iter().any(|t_i| edge_target_ids.contains(t_i)) {
                    if edge_target_ids.iter().all(|e_t_i| node_ids_to_num_ways_to_exit.contains_key(e_t_i)) && !node_ids_to_num_ways_to_exit.contains_key(source_id) {
                        node_ids_to_num_ways_to_exit.insert(*source_id, edge_target_ids.iter().map(|e_t_i| node_ids_to_num_ways_to_exit[e_t_i]).sum());
                        target_ids.push(*source_id);
                    }
                }
            }
        }

        Ok(node_ids_to_num_ways_to_exit[&node_labels_to_ids["you"]])
    }

    assert_eq!(5, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut node_labels_to_ids: HashMap<String, usize> = HashMap::new();
        let mut edge_strings: Vec<String> = Vec::new();
        let mut edges: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut node_ids_to_num_ways_to_exit: HashMap<usize, (usize, usize, usize, usize)> = HashMap::new(); // both DAC + FFT, DAC only, FFT only, neither
        let mut cur_id: usize = 0;
        for line in reader.lines().map(|l| l.unwrap()) {
            let tokens: Vec<&str> = line.split(": ").collect();
            node_labels_to_ids.insert(tokens[0].to_string(), cur_id);
            cur_id += 1;
            edge_strings.push(tokens[1].to_string());
        }

        let dac_id = node_labels_to_ids["dac"];
        let fft_id = node_labels_to_ids["fft"];
        node_labels_to_ids.insert("out".to_string(), cur_id);
        node_ids_to_num_ways_to_exit.insert(cur_id, (0,0,0,1));
        for i in 0..edge_strings.len() {
            edges.insert(i, edge_strings[i].split(" ").map(|t_l| node_labels_to_ids[t_l]).collect());
        }
        let mut target_ids: Vec<usize> = vec![cur_id];

        while !node_ids_to_num_ways_to_exit.contains_key(&node_labels_to_ids["svr"]) {
            for (source_id, edge_target_ids) in edges.iter() {
                if target_ids.iter().any(|t_i| edge_target_ids.contains(t_i)) {
                    if edge_target_ids.iter().all(|e_t_i| node_ids_to_num_ways_to_exit.contains_key(e_t_i)) && !node_ids_to_num_ways_to_exit.contains_key(source_id) {
                        let mut num_ways_to_exit = edge_target_ids.iter().map(|e_t_i| node_ids_to_num_ways_to_exit[e_t_i]).fold((0, 0, 0, 0), |a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3));
                        if *source_id == dac_id {
                            num_ways_to_exit = (num_ways_to_exit.0 + num_ways_to_exit.2, num_ways_to_exit.1 + num_ways_to_exit.3, 0, 0);
                        }
                        if *source_id == fft_id {
                            num_ways_to_exit = (num_ways_to_exit.0 + num_ways_to_exit.1, 0, num_ways_to_exit.2 + num_ways_to_exit.3, 0);
                        }

                        node_ids_to_num_ways_to_exit.insert(*source_id, num_ways_to_exit);
                        target_ids.push(*source_id);
                    }
                }
            }
        }

        Ok(node_ids_to_num_ways_to_exit[&node_labels_to_ids["svr"]].0)
    }
    
    assert_eq!(2, part2(BufReader::new(TEST2.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
