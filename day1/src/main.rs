use std::fs::File;
use std::io::{self, BufRead};

const INPUT_PATH: &str = 
"./input";
// "./test";

fn main() {
    p1();
    p2();
}

fn p1() {
    let file = File::open(INPUT_PATH).expect("failed to open input file, be sure to run main in src/");
    let lines = io::BufReader::new(file).lines();
    let mut dial_value = 50;
    let mut zero_count = 0;
    for dial_rotation in lines.map_while(Result::ok) {
        assert!(dial_value <= 99);
        let (dial_direction, click_count) = dial_rotation.split_at(1);
        let click_count: u32 = click_count.parse().expect("failed to parse click count");
        // rotation of k clicks == rotation of k + (i * 100) clicks, for i = 0..n
        let click_count = click_count % 100;
        match dial_direction {
            // rotation of less than 100 (k) clicks left == rotation of 100 - k clicks right
            "L" => dial_value += 100 - click_count,
            "R" => dial_value += click_count,
            _ => panic!("unexpected direction: {dial_direction}")
        };
        dial_value %= 100;
        if dial_value == 0 {
            zero_count += 1;
        }
    }
    println!("p1 zero count: {zero_count}");
}

fn p2() {
    let file = File::open(INPUT_PATH).expect("failed to open input file, pls run in src/");
    let lines = io::BufReader::new(file).lines();
    let mut dial_value = 50;
    let mut zero_count = 0;
    for dial_rotation in lines.map_while(Result::ok) {
        assert!(dial_value >= 0 && dial_value <= 99);
        let (dial_direction, click_count) = dial_rotation.split_at(1);
        let click_count: i32 = click_count.parse().expect("failed to parse click count");
        // rotation of k clicks == rotation of k + (i * 100) clicks, for i = 0..n
        zero_count += click_count / 100;
        let click_count = click_count % 100;
        let old_dial_value = dial_value;
        match dial_direction {
            "L" => {
                dial_value -= click_count;
                if dial_value < 0 {
                    dial_value += 100;
                    // if dial started at 0, the prev iteration already accounted for it 
                    // if the dial ended at 0, the end of the current iteration will handle it 
                    if old_dial_value != 0 && dial_value != 0 {
                        zero_count += 1;
                    }
                }
            },
            "R" => {
                dial_value += click_count;
                if dial_value > 99 {
                    dial_value -= 100;
                    // same reasoning as in "L" match arm
                    if old_dial_value != 0 && dial_value != 0 {
                        zero_count += 1;
                    }
                }
            },
            _ => panic!("unexpected direction: {dial_direction}")
        };
        if dial_value == 0 {
            zero_count += 1;
        }
    }
    println!("p2 zero count: {zero_count}");
}
