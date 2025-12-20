use std::fs::File;
use std::io::{self, BufRead};

const INPUT_PATH: &str = 
"./input";
// "./test";

fn main() {
    let file = File::open(INPUT_PATH).expect("failed to open input file, pls run in src/");
    let battery_banks = io::BufReader::new(file).lines();
    let mut p1_total_joltage = 0;
    let mut p2_total_joltage = 0;
    for battery_bank in battery_banks.map_while(Result::ok) {
        assert!(battery_bank.is_ascii(), "bank is not ascii only");
        let joltages: Vec<u32> = battery_bank
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect();
        assert!(joltages.len() >= 2, "less than 2 joltage ratings in battery bank");
        p1_total_joltage += p1_bank_joltage(&joltages);
        p2_total_joltage += p2_bank_joltage(&joltages);
    }
    println!("p1 total joltage: {p1_total_joltage}");
    println!("p2 total joltage: {p2_total_joltage}");
}

fn p1_bank_joltage(joltages: &Vec<u32>) -> u32 {
    let mut first_joltage = joltages[0];
    let mut second_joltage = joltages[1];
    for curr_joltage in joltages[2..].iter() {
        if *curr_joltage > second_joltage {
            if second_joltage > first_joltage{
                first_joltage = second_joltage;
            }
            second_joltage = *curr_joltage;
        } else if second_joltage > first_joltage {
            first_joltage = second_joltage;
            second_joltage = *curr_joltage;
        }
    }
    return first_joltage * 10 + second_joltage;
}

fn p2_bank_joltage(joltages: &Vec<u32>) -> u32 {
    let mut first_joltage = joltages[0];
    let mut second_joltage = joltages[1];
    for curr_joltage in joltages[2..].iter() {
        if *curr_joltage > second_joltage {
            if second_joltage > first_joltage{
                first_joltage = second_joltage;
            }
            second_joltage = *curr_joltage;
        } else if second_joltage > first_joltage {
            first_joltage = second_joltage;
            second_joltage = *curr_joltage;
        }
    }
    return first_joltage * 10 + second_joltage;
}
