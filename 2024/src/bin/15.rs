use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "15";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
const TEST_LRG: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut robot_map: Vec<Vec<char>> = Vec::new();
        let mut robot_instructions: Vec<Vec<char>> = Vec::new();
        let mut map_mode = true;
        for line in reader.lines().map(|l| l.unwrap()) {
            if line.is_empty() {
                map_mode = false;
            }
            if map_mode {
                robot_map.push(line.chars().collect());
            } else {
                robot_instructions.push(line.chars().collect());
            }
        }

        let n = robot_map.len();
        let m = robot_map[0].len();
        let mut cur_robot_pos: (i32, i32) = (0,0);
        for i in 0..n {
            for j in 0..m {
                if robot_map[i][j] == '@' {
                    cur_robot_pos = (i as i32, j as i32);
                    break;
                }
            }
        }

        for instructions in robot_instructions {
            for instruction in instructions {
                let direction: (i32, i32) = match instruction {
                    '^' => (-1, 0),
                    '>' => (0, 1),
                    'v' => (1, 0),
                    '<' => (0, -1),
                    _ => (0,0)
                };
                let mut push_length: i32 = 1;
                let mut next_box = robot_map[(cur_robot_pos.0 + push_length * direction.0) as usize][(cur_robot_pos.1 + push_length * direction.1) as usize];
                while next_box == 'O' {
                    push_length += 1;
                    next_box = robot_map[(cur_robot_pos.0 + push_length * direction.0) as usize][(cur_robot_pos.1 + push_length * direction.1) as usize];
                }
                if next_box == '.' {
                    for i in (2..=push_length).rev() {
                        robot_map[(cur_robot_pos.0 + i * direction.0) as usize][(cur_robot_pos.1 + i * direction.1) as usize] = 'O';
                    }
                    // move robot
                    robot_map[(cur_robot_pos.0 + direction.0) as usize][(cur_robot_pos.1 + direction.1) as usize] = '@';
                    robot_map[cur_robot_pos.0 as usize][cur_robot_pos.1 as usize] = '.';
                    cur_robot_pos = (cur_robot_pos.0 + direction.0, cur_robot_pos.1 + direction.1);
                }
            }
        }

        let mut result: usize = 0;
        for i in 0..n {
            for j in 0..m {
                if robot_map[i][j] == 'O' {
                    result += 100 * i + j;
                }
            }
        }


        Ok(result)
    }

    assert_eq!(2028, part1(BufReader::new(TEST.as_bytes()))?);
    assert_eq!(10092, part1(BufReader::new(TEST_LRG.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        // You could also solve this by using the old map, introducing 0.5 as a coordinate step size, and handling the case that a box pushes multiple boxes in front of it
        let mut robot_map: Vec<Vec<char>> = Vec::new();
        let mut robot_instructions: Vec<Vec<char>> = Vec::new();
        let mut map_mode = true;
        for line in reader.lines().map(|l| l.unwrap()) {
            if line.is_empty() {
                map_mode = false;
            }
            if map_mode {
                robot_map.push(line.chars().map(|c| match c {
                    '#' => vec!['#', '#'],
                    '.' => vec!['.', '.'],
                    'O' => vec!['[', ']'],
                    '@' => vec!['@', '.'],
                    _ => panic!("Illegal symbol when converting map")
                }).flatten().collect());
            } else {
                robot_instructions.push(line.chars().collect());
            }
        }

        let n = robot_map.len();
        let m = robot_map[0].len();
        let mut cur_robot_pos: (i32, i32) = (0,0);
        for i in 0..n {
            for j in 0..m {
                if robot_map[i][j] == '@' {
                    cur_robot_pos = (i as i32, j as i32);
                    break;
                }
            }
        }

        for instructions in robot_instructions {
            for instruction in instructions {
                let direction: (i32, i32) = match instruction {
                    '^' => (-1, 0),
                    '>' => (0, 1),
                    'v' => (1, 0),
                    '<' => (0, -1),
                    _ => (0,0)
                };
                let mut next_coordinates_to_check: Vec<(i32, i32)> = vec![(cur_robot_pos.0 + direction.0, cur_robot_pos.1 + direction.1)];
                let mut next_grid_items_to_check: Vec<char> = next_coordinates_to_check.iter().map(|coord| robot_map[coord.0 as usize][coord.1 as usize]).collect();
                while next_grid_items_to_check.iter().any(|item| *item != '.' && *item != '#') {
                    next_coordinates_to_check = next_coordinates_to_check.iter().map(|coord| match robot_map[coord.0 as usize][coord.1 as usize] {
                        '[' => vec![(coord.0 + direction.0, coord.1 + direction.1), (coord.0 + direction.0, coord.1 + direction.1 + 1)],
                        ']' => vec![(coord.0 + direction.0, coord.1 + direction.1), (coord.0 + direction.0, coord.1 + direction.1 - 1)],
                        '.' => vec![],
                        _ => panic!("Illegal state when adding coordinates to check {}", robot_map[coord.0 as usize][coord.1 as usize])
                    }).flatten().collect();
                    next_grid_items_to_check = next_coordinates_to_check.iter().map(|coord| robot_map[coord.0 as usize][coord.1 as usize]).collect();
                }
                if next_grid_items_to_check.iter().any(|item| *item == '#') {
                    continue;
                }
                // TODO solve this nicely
                // TODO left/right move is same as above
                // move robot + boxes
                let mut cur_coordinates_to_move: Vec<(i32, i32)> = vec![(cur_robot_pos.0 + direction.0, cur_robot_pos.1 + direction.1)].iter().filter(|coord| robot_map[coord.0 as usize][coord.1 as usize] != '.').collect();
                while !cur_coordinates_to_check.is_empty() {
                    let mut next_coordinates_to_move: Vec<(i32, i32)>  = cur_coordinates_to_move.iter().map(|coord| match robot_map[coord.0 as usize][coord.1 as usize] {
                        '[' => vec![(coord.0 + direction.0, coord.1 + direction.1), (coord.0 + direction.0, coord.1 + direction.1 + 1)],
                        ']' => vec![(coord.0 + direction.0, coord.1 + direction.1 - 1), (coord.0 + direction.0, coord.1 + direction.1)],
                        '.' => vec![],
                        _ => panic!("Illegal state when adding coordinates to check {}", robot_map[coord.0 as usize][coord.1 as usize])
                    }).flatten().collect();
                    for coordinates_to_move in cur_coordinates_to_move {
                        robot_map[coordinates_to_move.0 as usize][coordinates_to_move.1 as usize] = '[';
                        robot_map[coordinates_to_move.0 as usize][(coordinates_to_move.1 + 1) as usize] = ']';
                    }
                    cur_coordinates_to_move = next_coordinates_to_move;
                }

                robot_map[(cur_robot_pos.0 + direction.0) as usize][(cur_robot_pos.1 + direction.1) as usize] = '@';
                robot_map[cur_robot_pos.0 as usize][cur_robot_pos.1 as usize] = '.';
                cur_robot_pos = (cur_robot_pos.0 + direction.0, cur_robot_pos.1 + direction.1);
            }
        }

        let mut result: usize = 0;
        for i in 0..n {
            for j in 0..m {
                if robot_map[i][j] == '[' {
                    result += 100 * i + j;
                }
            }
        }


        Ok(result)
    }
    
    assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
