mod part1;
mod part2;

use std::env;
use std::fs::File;
use std::io::Read;


fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let times = part1::get_times(&mut lines);
    let distances = part1::get_distances(&mut lines);
    let ways = part1::ways(&times, &distances);
    
    ways.iter().fold(1, |acc, &x| acc * x)
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let time = part2::get_time(&mut lines);
    let distance = part2::get_distance(&mut lines);
    let ways = part2::ways(time, distance);
    
    ways
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

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 288);
    }

    #[test]
    fn test_par2() {
        assert_eq!(part2(INPUT), 71503);
    }
}
