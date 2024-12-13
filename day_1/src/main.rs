use std::collections::HashMap;
use library::read_file;
use library::get_values;

fn main() {
    let content = match read_file("input.txt") {
        Ok(content) => content,
        Err(error) => panic!("Problem reading the file: {error:?}"),
    };

    let mut v1: Vec<u32> = Vec::new();
    let mut v2: Vec<u32> = Vec::new();

    for line in content.lines() {
        let split = match get_values(line) {
            Ok(split) => split,
            Err(error) => panic!("Problem getting values: {error:?}"),
        };

        v1.push(split[0]);
        v2.push(split[1]);
    }

    v1.sort();
    v2.sort();

    let mut distance = 0;

    for i in 0..v1.len() {
        distance += v1[i].abs_diff(v2[i]);
    }

    println!("Total distance: {}", distance);

    let mut v2_map = HashMap::new();

    for value in &v2 {
        let count = v2_map.entry(value).or_insert(0);
        *count += 1;
    }

    let mut similarity = 0;

    for value in &v1 {
        similarity += value * v2_map.get(value).copied().unwrap_or(0);
    }

    println!("Similarity score: {}", similarity);
}