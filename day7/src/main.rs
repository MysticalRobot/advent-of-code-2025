use std::fs::File;
use std::io::{self, BufRead};

const INPUT_PATH: &str =
    "./input";
    // "./test";

fn main() {
    let file =
        File::open(INPUT_PATH).expect("failed to open input file, be sure to run main in src/");
    let grid: Vec<Vec<u8>> = io::BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .map(|line| line.bytes().collect())
        .collect();
    let start_i = 0;
    let start_j = 'init_j: loop {
        for (j, spot) in grid[start_i].iter().enumerate() {
            if *spot == b'S' {
                break 'init_j j;
            }
        }
        panic!("failed to find start column index");
    };
    let mut visited: Vec<Vec<u64>> = Vec::new();
    init_visited(&mut visited, grid.len(), grid[0].len());
    let p1_splits = p1_dfs(&grid, &mut visited, start_i, start_j);
    visited.clear();
    init_visited(&mut visited, grid.len(), grid[0].len());
    let p2_splits = p2_dfs(&grid, &mut visited, start_i, start_j);
    println!("p1 splits {p1_splits}");
    println!("p2 splits {p2_splits}");
}

fn init_visited(visited: &mut Vec<Vec<u64>>, m: usize, n: usize) {
    for _ in 0..m {
        let mut row = Vec::new();
        for _ in 0..n {
            row.push(0);
        }
        visited.push(row);
    }
}

fn p1_dfs(grid: &Vec<Vec<u8>>, visited: &mut Vec<Vec<u64>>, i: usize, j: usize) -> u64 {
    visited[i][j] = 1;
    if i == grid.len() - 1 {
        return 0;
    }
    if grid[i + 1][j] == b'.' {
        return p1_dfs(grid, visited, i + 1, j);
    }
    let mut split_either_dir = false;
    let mut result = 0;
    if j > 0 && visited[i + 1][j - 1] == 0 {
        split_either_dir = true;
        result += p1_dfs(grid, visited, i + 1, j - 1);
    }
    if j < grid.len() - 1 && visited[i + 1][j + 1] == 0 {
        split_either_dir = true;
        result += p1_dfs(grid, visited, i + 1, j + 1);
    }
    if split_either_dir {
        result += 1;
    } 
    result
}

fn p2_dfs(grid: &Vec<Vec<u8>>, visited: &mut Vec<Vec<u64>>, i: usize, j: usize) -> u64 {
    if i == grid.len() - 1 {
        visited[i][j] = 1;
        return visited[i][j];
    }
    if grid[i + 1][j] == b'.' {
        visited[i][j] = p2_dfs(grid, visited, i + 1, j);
        return visited[i][j];
    }
    if j > 0 {
        if visited[i + 1][j - 1] == 0 {
            visited[i][j] = p2_dfs(grid, visited, i + 1, j - 1);
        } else {
            visited[i][j] = visited[i + 1][j - 1];
        }
    }
    if j < grid.len() - 1 {
        if visited[i + 1][j + 1] == 0 {
            visited[i][j] += p2_dfs(grid, visited, i + 1, j + 1);
        } else {
            visited[i][j] += visited[i + 1][j + 1];
        }
    }
    visited[i][j]
}
