use std::fs::File;
use std::io::{self, BufRead};

const INPUT_PATH: &str =
    "input";
    // "test";
const NUM_CONNECTIONS: usize =
    1000;
    // 10;

fn main() {
    let file =
        File::open(INPUT_PATH).expect("failed to open input file, be sure to run main in src/");
    let coordinates: Vec<Vec<i64>> = io::BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .map(|coord| {
            coord
                .split(',')
                .map(|num| num.parse().expect("failed to parse coordinate"))
                .collect()
        })
        .collect();
    let mut distances: Vec<(i64, usize, usize)> = coordinates
        .iter()
        .enumerate()
        .map(|(i, c1)| {
            let distances: Vec<(i64, usize, usize)> = coordinates
                .iter()
                .enumerate()
                .filter(|(j, _)| *j < i)
                .map(|(j, c2)| (distance_squared(&c1, &c2), i, j))
                .collect();
            distances
        })
        .flatten()
        .collect();
    distances.sort();
    let mut reps: Vec<usize> = (0..coordinates.len()).collect();
    distances
        .iter()
        .take(NUM_CONNECTIONS)
        .for_each(|(_, i, j)| union(&mut reps, *i, *j));
    let mut clique_sizes: Vec<u32> = vec![0; reps.len()];
    reps.iter()
        .map(|i| find(&reps, *i))
        .for_each(|rep| clique_sizes[rep] += 1);
    clique_sizes.sort();
    let p1_largest_clique_sizes_product: u32 = clique_sizes.iter().rev().take(3).product();
    println!("p1 three largest clique sizes product: {p1_largest_clique_sizes_product}");
    let mut p2_last_connection_x_product: i64 = -1;
    for (_, i, j) in distances.iter().skip(NUM_CONNECTIONS) {
        union(&mut reps, *i, *j);
        let true_reps: Vec<usize> = reps.iter().map(|i| find(&reps, *i)).collect();
        let first = true_reps[0];
        let all_same_rep = true_reps.iter().filter(|rep| **rep != first).count() == 0;
        if all_same_rep {
            p2_last_connection_x_product = coordinates[*i][0] * coordinates[*j][0];
            break;
        }
    }
    println!("p2 last connection x product: {p2_last_connection_x_product}");
}

fn distance_squared(c1: &Vec<i64>, c2: &Vec<i64>) -> i64 {
    c1.iter()
        .zip(c2)
        .take(3)
        .map(|(n1, n2)| i64::pow(n1 - n2, 2))
        .sum()
}

fn union(reps: &mut Vec<usize>, i: usize, j: usize) {
    let rep_i = find_with_path_compression(reps, i);
    let rep_j = find_with_path_compression(reps, j);
    if rep_i != rep_j {
        reps[rep_i] = rep_j;
    }
}

fn find_with_path_compression(reps: &mut Vec<usize>, i: usize) -> usize {
    if reps[i] != i {
        reps[i] = find_with_path_compression(reps, reps[i]);
    }
    reps[i]
}

fn find(reps: &Vec<usize>, i: usize) -> usize {
    if reps[i] != i {
        find(reps, reps[i])
    } else {
        reps[i]
    }
}
