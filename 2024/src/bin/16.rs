use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp;
use std::cmp::Reverse;
use code_timing_macros::time_snippet;
use priority_queue::PriorityQueue;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "16";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";


const TEST2: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";


// This time code sensibly with helper methods etc and less copy paste :)

struct DirectedWeightedGraphNode<T, W> {
    value: Box<T>,
    edges: Vec<DirectedWeightedEdge<W>>,
}

struct DirectedWeightedEdge<W> {
    from: (usize, usize, usize),
    to: (usize, usize, usize),
    weight: W,
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn read_map<R: BufRead>(reader: R) -> Vec<Vec<char>> {
        let mut map: Vec<Vec<char>> = Vec::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            map.push(line.chars().collect());
        }
        map
    }

    fn construct_graph_from_map(map: Vec<Vec<char>>) -> (Vec<Vec<Vec<DirectedWeightedGraphNode<bool, u64>>>>, (usize, usize, usize), (usize, usize)) {
        let mut result: Vec<Vec<Vec<DirectedWeightedGraphNode<bool, u64>>>> = Vec::new();
        let n = map.len();
        let m = map[0].len();
        for _i in 0..n {
            let mut cur_row: Vec<Vec<DirectedWeightedGraphNode<bool, u64>>> = Vec::new();
            for _j in 0..m {
                // Convention: orientation is >, v, <, ^
                cur_row.push(vec![DirectedWeightedGraphNode { value: Box::new(true), edges: vec![] }, DirectedWeightedGraphNode { value: Box::new(true), edges: vec![] }, DirectedWeightedGraphNode { value: Box::new(true), edges: vec![] }, DirectedWeightedGraphNode { value: Box::new(true), edges: vec![] }]);
            }
            result.push(cur_row);
        }
        let mut start_pos: (usize, usize, usize) = (0,0,0);
        let mut end_pos: (usize, usize) = (0,0);

        for i in 0..n {
            for j in 0..m {
                if map[i][j] == 'S' {
                    start_pos = (i, j, 0);
                }
                if map[i][j] == 'E' {
                    end_pos = (i, j);
                }
                if map[i][j] == '#' {
                    continue;
                } else {
                    // Could write this a bit more succinctly with a for loop, but this is more explicit and not much more code
                    // Orientation >
                    let mut cur_node: &mut DirectedWeightedGraphNode<bool, u64> = &mut result[i][j][0];
                    if j < m-1 && map[i][j+1] != '#' {
                        cur_node.edges.push(DirectedWeightedEdge{ from: (i,j,0), to: (i,j+1,0), weight: 1u64 });
                    }
                    cur_node.edges.push(DirectedWeightedEdge { from: (i,j,0), to: (i,j,1), weight: 1000u64 });
                    cur_node.edges.push(DirectedWeightedEdge { from: (i,j,0), to: (i,j,3), weight: 1000u64 });
                    // Orientation v
                    cur_node = &mut result[i][j][1];
                    if i < n-1 && map[i+1][j] != '#' {
                        cur_node.edges.push(DirectedWeightedEdge { from: (i,j,1), to: (i+1,j,1), weight: 1u64 });
                    }
                    cur_node.edges.push(DirectedWeightedEdge { from: (i,j,1), to: (i,j,0), weight: 1000u64 });
                    cur_node.edges.push(DirectedWeightedEdge { from: (i,j,1), to: (i,j,2), weight: 1000u64 });
                    // Orientation <
                    cur_node = &mut result[i][j][2];
                    if j > 0 && map[i][j-1] != '#' {
                        cur_node.edges.push(DirectedWeightedEdge { from: (i,j,2), to: (i,j-1,2), weight: 1u64 });
                    }
                    cur_node.edges.push(DirectedWeightedEdge { from: (i,j,2), to: (i,j,3), weight: 1000u64 });
                    cur_node.edges.push(DirectedWeightedEdge { from: (i,j,2), to: (i,j,1), weight: 1000u64 });
                    // Orientation ^
                    cur_node = &mut result[i][j][3];
                    if i > 0 && map[i-1][j] != '#' {
                        cur_node.edges.push(DirectedWeightedEdge { from: (i,j,3), to: (i-1,j,3), weight: 1u64 });
                    }
                    cur_node.edges.push(DirectedWeightedEdge { from: (i,j,3), to: (i,j,2), weight: 1000u64 });
                    cur_node.edges.push(DirectedWeightedEdge { from: (i,j,3), to: (i,j,0), weight: 1000u64 });
                }
            }
        }
        (result, start_pos, end_pos)
    }

    fn find_shortest_path_length(graph: &Vec<Vec<Vec<DirectedWeightedGraphNode<bool, u64>>>>, from: (usize, usize, usize), to: (usize, usize, usize)) -> usize {
        let mut dist: Vec<Vec<Vec<u64>>> = Vec::new();
        let mut prev: Vec<Vec<Vec<Option<(usize, usize, usize)>>>> = Vec::new();
        let n = graph.len();
        let m = graph[0].len();
        let k = graph[0][0].len();
        let mut vertices = PriorityQueue::new();
        for i in 0..n {
            let mut cur_dist_row: Vec<Vec<u64>> = Vec::new();
            let mut cur_prev_row: Vec<Vec<Option<(usize, usize, usize)>>> = Vec::new();
            for j in 0..m {
                let mut cur_dist_col: Vec<u64> = Vec::new();
                let mut cur_prev_col: Vec<Option<(usize, usize, usize)>> = Vec::new();
                for o in 0..k {
                    cur_dist_col.push(u64::MAX);
                    cur_prev_col.push(None);
                    vertices.push((i, j, o), Reverse(u64::MAX));
                }
                cur_dist_row.push(cur_dist_col);
                cur_prev_row.push(cur_prev_col);
            }
            dist.push(cur_dist_row);
            prev.push(cur_prev_row);
        }
        dist[from.0][from.1][from.2] = 0u64;
        vertices.change_priority(&(from.0, from.1, from.2), Reverse(0u64));
        while !vertices.is_empty() {
            if let Some((u, _cur_priority)) = vertices.pop() {
                for e in &graph[u.0][u.1][u.2].edges {
                    let to_node = (e.to.0, e.to.1, e.to.2);
                    if vertices.get(&to_node).is_some() {
                        let alternative_length = dist[u.0][u.1][u.2] + e.weight;
                        if alternative_length < dist[to_node.0][to_node.1][to_node.2] {
                            dist[to_node.0][to_node.1][to_node.2] = alternative_length;
                            vertices.change_priority(&to_node, Reverse(alternative_length));
                            prev[to_node.0][to_node.1][to_node.2] = Some(u);
                        }
                    }
                }
            }
        }

        dist[to.0][to.1][to.2].try_into().unwrap()
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let map = read_map(reader);
        let (maze_graph, start_pos, end_pos) = construct_graph_from_map(map);
        let mut result: usize = usize::MAX;
        for orientation in 0..4 {
            result = cmp::min(result, find_shortest_path_length(&maze_graph, start_pos, (end_pos.0, end_pos.1, orientation)));
        }
        Ok(result)
    }

    assert_eq!(7036, part1(BufReader::new(TEST.as_bytes()))?);
    assert_eq!(11048, part1(BufReader::new(TEST2.as_bytes()))?);

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
