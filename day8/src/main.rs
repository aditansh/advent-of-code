mod common;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

use common::{get_directions, get_map};

struct Map {
    left: String,
    right: String,
}

fn part1(input: &str) -> u64 {
    let mut steps = 0;
    let mut lines = input.lines();
    let mut directions: Vec<char> = get_directions(lines.next().unwrap());
    let map: HashMap<String, Map> = get_map(&mut lines);

    let mut curr = "AAA".to_string();
    let end = "ZZZ".to_string();

    while curr != end {
        let map = map.get(&curr).unwrap();
        let mut next = String::new();
        if directions[0] == 'R' {
            next = map.right.clone();
        } else if directions[0] == 'L' {
            next = map.left.clone();
        }
        let x = directions.remove(0);
        directions.push(x);
        curr = next;
        steps += 1;
    }

    steps
}

fn part2(input: &str) -> u64 {
    let mut steps = 0;
    let mut lines = input.lines();
    let mut directions: Vec<char> = get_directions(lines.next().unwrap());
    let map: HashMap<String, Map> = get_map(&mut lines);

    let mut curr: Vec<String> = Vec::new();
    for i in map.keys() {
        if i.ends_with("A") {
            curr.push(i.to_string());
        }
    }

    while !curr.iter().all(|s| s.ends_with("Z")) {
        let maps: Vec<&Map> = curr
            .iter()
            .map(|s| map.get(s).unwrap())
            .collect::<Vec<&Map>>();
        let mut next: Vec<String> = Vec::new();
        if directions[0] == 'R' {
            for m in maps {
                next.push(m.right.clone());
            }
        } else if directions[0] == 'L' {
            for m in maps {
                next.push(m.left.clone());
            }
        }
        let x = directions.remove(0);
        directions.push(x);
        curr = next;
        steps += 1;
    }

    steps
}

fn main() {
    if let Ok(current_dir) = env::current_dir() {
        let path = current_dir.join("input.txt");
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        println!("Part 1: {}", part1(&contents));
        println!("Part 2: {}", part2(&contents));
    } else {
        println!("Error: could not get current directory");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        let inpur = "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)";

        assert_eq!(part1(inpur), 2);
    }

    #[test]
    fn test_part1_2() {
        let inpur = "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)";

        assert_eq!(part1(inpur), 6);
    }

    #[test]
    fn test_part2() {
        let input = "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)";

        assert_eq!(part2(input), 6);
    }
}
