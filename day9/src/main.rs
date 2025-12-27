use std::fs::File;
use std::io::{self, BufRead};

const INPUT_PATH: &str =
    "input";
    // "test";

fn main() {
    let input =
        File::open(INPUT_PATH).expect("failed to open input file, be sure to run main in src/");
    let red_tiles: Vec<(i64, i64)> = io::BufReader::new(input)
        .lines()
        .map_while(Result::ok)
        .map(|tile| {
            let (x, y) = tile
                .split_once(',')
                .expect("failed to split tile coordinate");
            (
                y.parse().expect("failed to parse tile y"),
                x.parse().expect("failed to parse tile x")
            )
        })
        .collect();
    let largest_area = red_tiles.iter().enumerate()
        .map(|(i, (y1, x1))| {
            red_tiles.iter().enumerate().filter(|(j, (_, _))| i != *j)
                .map(|(_, (y2, x2))| (i64::abs(*y1 - *y2) + 1) * (i64::abs(*x1 - *x2) + 1))
                .max()
                .expect("failed to get largest area")
        })
        .max()
        .expect("failed to get largest area");
    println!("largest area: {largest_area}");
}
