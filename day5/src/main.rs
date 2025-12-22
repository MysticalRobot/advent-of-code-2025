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
    let mut input = input.split(|line| line.len() == 0);
    let mut fresh_id_ranges: Vec<(u128, u128)> = input
        .next()
        .expect("failed to get fresh ingredient id ranges")
        .iter()
        .map(|range| {
            let (start, end) = range.split_once("-").expect("failed to split id range");
            (
                start.parse().expect("failed to parse id range start"),
                end.parse().expect("failed to parse id range start"),
            )
        })
        .collect();
    fresh_id_ranges.sort();
    let fresh_id_ranges = merged_overlapping_intervals(fresh_id_ranges);
    let available_ids: Vec<u128> = input
        .next()
        .expect("failed to get available ingredient ids")
        .iter()
        .map(|id| id.parse().expect("failed to parse available id"))
        .collect();
    assert!(
        input.next().is_none(),
        "failed to split input into two parts"
    );
    let p1_fresh_available_ids = available_ids
        .iter()
        .filter(|id| {
            fresh_id_ranges
            .binary_search_by(|(a, b)| {
                if *a <= **id && **id <= *b {
                    std::cmp::Ordering::Equal
                } else if **id < *a {
                    std::cmp::Ordering::Greater
                } else {
                    // *b < **id
                    std::cmp::Ordering::Less
                }
            })
            .is_ok()
        })
        .count();
    let p2_fresh_available_ids = fresh_id_ranges
        .iter()
        .fold(0, |acc, (a, b)| acc + (*b - *a + 1));
    println!("p1 fresh available ids: {p1_fresh_available_ids}");
    println!("p2 fresh available ids: {p2_fresh_available_ids}");
}

fn merged_overlapping_intervals(mut fresh_id_ranges: Vec<(u128, u128)>) -> Vec<(u128, u128)> {
    if fresh_id_ranges.len() != 0 {
        let unmerged_fresh_id_ranges = fresh_id_ranges;
        let (first, rest) = unmerged_fresh_id_ranges
            .split_first()
            .expect("failed to get first and rest of ranges");
        fresh_id_ranges = vec![*first];
        for (s2, e2) in rest.iter() {
            let (s1, e1) = fresh_id_ranges.pop().expect("failed to get last of ranges");
            if *s2 <= e1 {
                let new_e = if e1 <= *e2 { *e2 } else { e1 };
                fresh_id_ranges.push((s1, new_e));
            } else {
                fresh_id_ranges.push((s1, e1));
                fresh_id_ranges.push((*s2, *e2));
            }
        }
    } 
    fresh_id_ranges
}
