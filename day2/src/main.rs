use std::fs::File;
use std::io::{self, BufRead};

const INPUT_PATH: &str = 
"./input";
// "./test";

fn main() {
    let file = File::open(INPUT_PATH).expect("failed to open input file, pls run in src/");
    let mut id_ranges = String::new();
    io::BufReader::new(file).read_line(&mut id_ranges).expect("failed to read input file");
    let id_ranges: Vec<&str> = id_ranges.trim().split(',').collect();
    let mut p1_invalid_ids_sum: u128 = 0;
    let mut p2_invalid_ids_sum: u128 = 0;
    for id_range in id_ranges {
        let interval_limits: Vec<&str> = id_range.split('-').collect();
        if interval_limits.len() != 2 {
            break;
        }
        let (start, end) = (interval_limits[0], interval_limits[1]);
        let (start, end): (u128, u128) = (
            start.parse().expect("failed to parse id range start"), 
            end.parse().expect("failed to parse id range start"),
        );
        p1_invalid_ids_sum += (start..=end).filter(p1_is_invalid_id).sum::<u128>();
        p2_invalid_ids_sum += (start..=end).filter(p2_is_invalid_id).sum::<u128>();
    }
    println!("p1 invalid ids sum: {p1_invalid_ids_sum}");
    println!("p2 invalid ids sum: {p2_invalid_ids_sum}");
}


fn p1_is_invalid_id(id: &u128) -> bool {
    let id = id.to_string();
    let len = id.len();
    // only even length ids can consists of a sequence that repeats twice
    if len % 2 == 1 {
        return false;
    }
    let middle = len/2;
    if id[..middle] == id[middle..] {
        return true;
    }
    return false;
}

fn p2_is_invalid_id(id: &u128) -> bool {
    let id = id.to_string();
    let len = id.len();
    if len <= 1 {
        return false;
    }
    for k in 1..=len/2 {
        // the stringified id should be divisible into k equal length sequences 
        if len % k != 0 || len / k < 2 {
            continue;
        }
        // if all the sequences are identical, the id is invalid
        let sequence = &id[0..k];
        let mut is_valid = true;
        for start in (0..len).step_by(k) {
            if &id[start..start+k] != sequence {
                is_valid = false;
                break;
            }
        }
        if is_valid {
            return true;
        }
    }
    return false;
}
