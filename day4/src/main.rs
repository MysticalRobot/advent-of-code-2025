use std::fs::File;
use std::io::{self, BufRead};

const INPUT_PATH: &str = 
"./input";
// "./test";

fn main() {
    let file = File::open(INPUT_PATH).expect("failed to open input file, be sure to run main in src/");
    let mut grid: Vec<Vec<u8>> = io::BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .map(|row| row.into_bytes())
        .collect();
    let mut p1_accessible_paper_count = 0;
    let mut p2_accessible_paper_count = 0;
    let mut accessible_papers: Vec<(usize, usize)> = Vec::new();
    loop {
        let first_iteration = p1_accessible_paper_count == 0;
        for (i, row) in grid.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell == b'@' && is_paper_accessible(&grid, (i, j)) {
                    p1_accessible_paper_count += if first_iteration { 1 } else { 0 };
                    accessible_papers.push((i, j));
                }
            }
        }
        if accessible_papers.len() == 0 {
            break;
        }
        for (i, j) in accessible_papers.iter() {
            p2_accessible_paper_count += 1;
            grid[*i][*j] = b'.';
        }
        accessible_papers.clear();
    }
    println!("p1_accessible paper count: {p1_accessible_paper_count}");
    println!("p2_accessible paper count: {p2_accessible_paper_count}");
}


fn is_paper_accessible(grid: &Vec<Vec<u8>>, paper: (usize, usize)) -> bool {
    let (row, col) = paper;
    let m = grid.len();
    assert!(m != 0, "grid is empty");
    assert!(row < m, "row out of bounds");
    let n = grid[0].len();
    assert!(n != 0, "grid row is empty");
    assert!(col < n, "col out of bounds");
    let has_left = col != 0;
    let has_right = col < n - 1;
    let has_top = row != 0;
    let has_bottom = row < m - 1;
    let mut adjacent_roll_count = 0;
    if has_top {
        let top_row = row - 1;
        if has_left && grid[top_row][col - 1] == b'@' {
            adjacent_roll_count += 1;
        }
        if grid[top_row][col] == b'@' {
            adjacent_roll_count += 1;
        }
        if has_right && grid[top_row][col + 1] == b'@' {
            adjacent_roll_count += 1;
        }
    }
    if has_left && grid[row][col - 1] == b'@' {
        adjacent_roll_count += 1;
    }
    if has_right && grid[row][col + 1] == b'@' {
        adjacent_roll_count += 1;
    }
    if has_bottom {
        let bottom_row = row + 1;
        if has_left && grid[bottom_row][col - 1] == b'@' {
            adjacent_roll_count += 1;
        }
        if grid[bottom_row][col] == b'@' {
            adjacent_roll_count += 1;
        }
        if has_right && grid[bottom_row][col + 1] == b'@' {
            adjacent_roll_count += 1;
        }
    }
    adjacent_roll_count < 4
}
