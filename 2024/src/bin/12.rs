use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "12";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut map: Vec<Vec<char>> = Vec::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            map.push(line.chars().collect());
        }

        let n = map.len();
        let m = map[0].len();
        let mut visited: Vec<Vec<bool>> = Vec::new();
        for _i in 0..n {
            let mut visited_row: Vec<bool> = Vec::new();
            for _j in 0..m {
                visited_row.push(false);
            }
            visited.push(visited_row);
        }
        let mut result: usize = 0;

        for i in 0..n {
            for j in 0..m {
                if visited[i][j] {
                    continue;
                }

                let cur_plant = map[i][j];
                let mut cur_nodes: Vec<(usize, usize)> = vec![(i, j)];
                let mut cur_area: usize = 0;
                let mut cur_perimeter: usize = 0;
                while !cur_nodes.is_empty() {
                    let Some((cur_x, cur_y)) = cur_nodes.pop() else { break; };
                    if visited[cur_x][cur_y] {
                        continue;
                    }
                    visited[cur_x][cur_y] = true;
                    cur_area += 1;
                    if cur_x > 0 && !visited[cur_x-1][cur_y] && map[cur_x-1][cur_y] == cur_plant {
                        cur_nodes.push((cur_x-1, cur_y));
                    } else if cur_x == 0 || map[cur_x-1][cur_y] != cur_plant {
                        cur_perimeter += 1;
                    }
                    if cur_y > 0 && !visited[cur_x][cur_y-1] && map[cur_x][cur_y-1] == cur_plant {
                        cur_nodes.push((cur_x, cur_y-1));
                    } else if cur_y == 0 || map[cur_x][cur_y-1] != cur_plant {
                        cur_perimeter += 1;
                    }
                    if cur_x < n-1 && !visited[cur_x+1][cur_y] && map[cur_x+1][cur_y] == cur_plant {
                        cur_nodes.push((cur_x+1, cur_y));
                    } else if cur_x == n-1 || map[cur_x+1][cur_y] != cur_plant {
                        cur_perimeter += 1;
                    }
                    if cur_y < m-1 && !visited[cur_x][cur_y+1] && map[cur_x][cur_y+1] == cur_plant {
                        cur_nodes.push((cur_x, cur_y+1));
                    } else if cur_y == m-1 || map[cur_x][cur_y+1] != cur_plant {
                        cur_perimeter += 1;
                    }
                }
                result += cur_area * cur_perimeter;
            }
        }

        Ok(result)
    }

    assert_eq!(1930, part1(BufReader::new(TEST.as_bytes()))?);

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

        let n = map.len();
        let m = map[0].len();
        let mut visited: Vec<Vec<bool>> = Vec::new();
        for _i in 0..n {
            let mut visited_row: Vec<bool> = Vec::new();
            for _j in 0..m {
                visited_row.push(false);
            }
            visited.push(visited_row);
        }
        let mut result: usize = 0;

        for i in 0..n {
            for j in 0..m {
                if visited[i][j] {
                    continue;
                }

                let cur_plant = map[i][j];
                let mut cur_nodes: Vec<(usize, usize)> = vec![(i, j)];
                let mut cur_area: usize = 0;
                let mut cur_corners: usize = 0;
                while !cur_nodes.is_empty() {
                    let Some((cur_x, cur_y)) = cur_nodes.pop() else { break; };
                    if visited[cur_x][cur_y] {
                        continue;
                    }
                    visited[cur_x][cur_y] = true;
                    cur_area += 1;
                    let mut has_horizontal_neighbor_1 = false;
                    let mut has_horizontal_neighbor_2 = false;
                    let mut has_vertical_neighbor_1 = false;
                    let mut has_vertical_neighbor_2 = false;
                    if cur_x > 0 && map[cur_x-1][cur_y] == cur_plant {
                        has_horizontal_neighbor_1 = true;
                    }
                    if cur_x < n-1 && map[cur_x+1][cur_y] == cur_plant {
                        has_horizontal_neighbor_2 = true;
                    }
                    if cur_y > 0 && map[cur_x][cur_y-1] == cur_plant {
                        has_vertical_neighbor_1 = true;
                    }
                    if cur_y < m-1 && map[cur_x][cur_y+1] == cur_plant {
                        has_vertical_neighbor_2 = true;
                    }

                    // Top left corner
                    if (!has_horizontal_neighbor_1 && !has_vertical_neighbor_1) || (has_horizontal_neighbor_1 && has_vertical_neighbor_1 && map[cur_x-1][cur_y-1] != cur_plant) {
                        cur_corners += 1;
                    }
                    // Bottom left corner
                    if (!has_horizontal_neighbor_1 && !has_vertical_neighbor_2) || (has_horizontal_neighbor_1 && has_vertical_neighbor_2 && map[cur_x-1][cur_y+1] != cur_plant) {
                        cur_corners += 1;
                    }
                    // Top right corner
                    if (!has_horizontal_neighbor_2 && !has_vertical_neighbor_1) || (has_horizontal_neighbor_2 && has_vertical_neighbor_1 && map[cur_x+1][cur_y-1] != cur_plant) {
                        cur_corners += 1;
                    }
                    // Bottom left corner
                    if (!has_horizontal_neighbor_2 && !has_vertical_neighbor_2) || (has_horizontal_neighbor_2 && has_vertical_neighbor_2 && map[cur_x+1][cur_y+1] != cur_plant) {
                        cur_corners += 1;
                    }

                    if cur_x > 0 && !visited[cur_x-1][cur_y] && map[cur_x-1][cur_y] == cur_plant {
                        cur_nodes.push((cur_x-1, cur_y));
                    }
                    if cur_y > 0 && !visited[cur_x][cur_y-1] && map[cur_x][cur_y-1] == cur_plant {
                        cur_nodes.push((cur_x, cur_y-1));
                    }
                    if cur_x < n-1 && !visited[cur_x+1][cur_y] && map[cur_x+1][cur_y] == cur_plant {
                        cur_nodes.push((cur_x+1, cur_y));
                    }
                    if cur_y < m-1 && !visited[cur_x][cur_y+1] && map[cur_x][cur_y+1] == cur_plant {
                        cur_nodes.push((cur_x, cur_y+1));
                    }
                }
                result += cur_area * cur_corners;
            }
        }   

        Ok(result)
    }
    
    assert_eq!(1206, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
