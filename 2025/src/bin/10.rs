use anyhow::*;
use std::fs::File;
use good_lp::{constraint, default_solver, Solution, SolverModel, variable, variables, Variable, Expression};
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

fn min_presses_to_solve(goal: Vec<usize>, actions: &Vec<Vec<usize>>) -> usize {
    variables!{vars: 0 <= x <= 0; } // The variable x will always be present
    let mut action_vars = Vec::new();
    for _ in 0..actions.len() {
        action_vars.push(vars.add(variable().min(0).integer()));
    }
    let objective: Expression = action_vars.iter().sum();
    let mut problem = vars.minimise(objective).using(default_solver);
    for i in 0..goal.len() {
        let num_desired_presses = goal[i] as i32;
        problem = problem.with(constraint!(actions.iter().enumerate().filter(|(_, a)| a.contains(&i)).map(|(k, _)| action_vars[k]).sum::<Expression>() <= num_desired_presses));
        problem = problem.with(constraint!(actions.iter().enumerate().filter(|(_, a)| a.contains(&i)).map(|(k, _)| action_vars[k]).sum::<Expression>() >= num_desired_presses));
    }
    let solution = problem.solve().expect("AOC should be solvable");
    return action_vars.iter().map(|a_v| solution.value(*a_v).round() as usize).sum();
}

fn min_steps_to_solve(goal: &str, actions: &Vec<Vec<usize>>) -> usize {
    let mut result: usize = 0;
    let mut cur_pool: Vec<String> = vec![goal.to_string()];
    loop {
        let mut next_pool: Vec<String> = Vec::new();
        for cur_goal in cur_pool {
            if cur_goal.chars().all(|c| c == '.') {
                return result;
            }
            let indices_to_change = cur_goal.chars().enumerate().filter(|(_, c)| *c == '#').map(|(i, _)| i).collect::<Vec<usize>>();
            for a in actions {
                if a.iter().any(|a_i| indices_to_change.contains(a_i)) {
                    let mut new_goal: Vec<char> = Vec::new();
                    for i in 0..cur_goal.len() {
                        new_goal.push(if a.contains(&i) {if cur_goal.chars().nth(i) == Some('.') {'#'} else {'.'}} else {cur_goal.chars().nth(i).expect("AoC should be solvable")});
                    }
                    next_pool.push(new_goal.into_iter().collect::<String>());
                }
            }
        }
        result += 1;
        cur_pool = next_pool;
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut result: usize = 0;
        for line in reader.lines().map(|l| l.unwrap()) {
            let tokens: Vec<&str> = line.split(" ").collect();
            let goal: &str = &tokens[0][1..tokens[0].len()-1];
            let mut actions: Vec<Vec<usize>> = Vec::new();
            for i in 1..tokens.len() {
                if tokens[i].starts_with("(") {
                    let nums_token: &str = &tokens[i][1..&tokens[i].len()-1];
                    actions.push(nums_token.split(",").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>());
                }
            }

            result += min_steps_to_solve(goal, &actions);
        }
        Ok(result)
    }

    assert_eq!(7, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut result: usize = 0;
        for line in reader.lines().map(|l| l.unwrap()) {
            let tokens: Vec<&str> = line.split(" ").collect();
            let goal_token: &str = &tokens[tokens.len() - 1][1..tokens[tokens.len()-1].len()-1];
            let goal: Vec<usize> = goal_token.split(",").map(|t| t.parse::<usize>().unwrap()).collect();
            let mut actions: Vec<Vec<usize>> = Vec::new();
            for i in 1..tokens.len() {
                if tokens[i].starts_with("(") {
                    let nums_token: &str = &tokens[i][1..&tokens[i].len()-1];
                    actions.push(nums_token.split(",").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>());
                }
            }

            result += min_presses_to_solve(goal, &actions);
        }
        Ok(result)
    }
    
    assert_eq!(33, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
