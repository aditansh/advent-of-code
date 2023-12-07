mod common;

use std::env;
use std::fs::File;
use std::io::Read;

use common::get_mappings;
use common::get_seeds;
use common::get_shortest_path;

fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let seeds = get_seeds(&mut lines);
    let mappings = get_mappings(&mut lines);
    let shortest = get_shortest_path(seeds.iter().cloned(), &mappings);

    shortest
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let seeds_line = get_seeds(&mut lines);
    let mappings = get_mappings(&mut lines);
    let mut seeds = Vec::new();
    let mut start: u64 = 0;
    for (i, &seed) in seeds_line.iter().enumerate() {
        if i % 2 == 0 {
            start = seed;
        } else {
            seeds.push(start..(start + seed))
        }
    }
    seeds
        .into_iter()
        .map(|range| get_shortest_path(range.into_iter(), &mappings))
        .min()
        .expect("No min")
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

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 35);
    }

    #[test]
    fn test_par2() {
        assert_eq!(part2(INPUT), 46);
    }
}
