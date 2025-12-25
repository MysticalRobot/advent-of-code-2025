use std::fs::File;
use std::io::{self, BufRead};

const INPUT_PATH: &str =
    "./input";
    // "./test";

fn main() {
    let file =
        File::open(INPUT_PATH).expect("failed to open input file, be sure to run main in src/");
    let input: Vec<String> = io::BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .collect();
    let (operations, grid) = input
        .split_last()
        .expect("failed to split numbers from operations");
    let p2_grid = p2_grid(operations, grid);
    let operations: Vec<&str> = (*operations).split_whitespace().collect();
    println!("p1 total: {}", solve_problems(&operations, p1_grid(&grid)));
    println!("p2 total: {}", solve_problems(&operations, p2_grid));
}

fn p1_grid(grid: &[String]) -> Vec<Vec<u128>> {
    let grid: Vec<Vec<u128>> = grid
        .iter()
        .map(|nums| {
            nums.split_whitespace()
                .map(|num| num.parse().expect("failed to parse number"))
                .collect()
        })
    .collect();
    let mut transpose: Vec<Vec<u128>> = Vec::new();
    for _ in 0..grid[0].len() {
        transpose.push(Vec::new());
    }
    for nums in grid {
        for (i, num) in nums.iter().enumerate() {
            transpose[i].push(*num);
        }
    }
    transpose
}

fn p2_grid(operations: &String, grid: &[String]) -> Vec<Vec<u128>> {
    let old_grid = grid;
    let mut grid: Vec<Vec<u128>> = Vec::new();
    let problem_count = operations.chars().filter(|c| *c != ' ').count();
    for _ in 0..problem_count {
        grid.push(Vec::new());
    }
    for nums in old_grid {
        let mut i = 0;
        let mut j = 0;
        for (op, digit) in operations.chars().zip(nums.chars()) {
            if op != ' ' && j != 0 {
                i += 1;
            } 
            if op != ' ' {
                j = 0;
            }
            let is_digit = digit.is_digit(10);
            let digit = if !is_digit { 0 } else { digit.to_digit(10).expect("failed to parse digit") };
            let digit = u128::from(digit);
            if grid[i].len() <= j {
                grid[i].push(digit);
            } else if is_digit {
                grid[i][j] = grid[i][j] * 10 + digit;
            } 
            j += 1;
        }
    }
    grid
}

fn solve_problems(operations: &Vec<&str>, grid: Vec<Vec<u128>>) -> u128 {
    let mut total = 0;
    for (nums, op) in grid.iter().zip(operations) {
        // filter out any extra zeros from p2
        let nums = nums.iter().filter(|num| **num != 0);
        if *op == "*" {
            let problem_result: u128 = nums.product();
            total += problem_result;
        } else if *op == "+" {
            let problem_result: u128 = nums.sum();
            total += problem_result;
        } else {
            panic!("unexpected operation {op}");
        }
    }
    total
}
