use std::{collections::HashMap, str::Lines};

use crate::Map;

pub fn get_directions(line: &str) -> Vec<char> {
    let mut directions: Vec<char> = Vec::new();
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        if c == 'R' || c == 'L' {
            directions.push(c);
        }
    }
    directions
}

pub fn get_map(lines: &mut Lines) -> HashMap<String, Map> {
    let mut map: HashMap<String, Map> = HashMap::new();

    for line in lines {
        if line.contains("=") {
            let split = line.split("=").collect::<Vec<&str>>();
            let left = split[0].trim().to_string();
            let right = split[1]
                .trim()
                .strip_prefix("(")
                .unwrap()
                .strip_suffix(")")
                .unwrap()
                .split(",")
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>();
            map.insert(
                left.clone(),
                Map {
                    left: right[0].clone(),
                    right: right[1].clone(),
                },
            );
        }
    }
    map
}
