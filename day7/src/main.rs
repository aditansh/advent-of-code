mod common;
mod part1;
mod part2;

use std::env;
use std::fs::File;
use std::io::Read;

struct Hand {
    pub value: String,
    pub bid: u64,
}

fn part1(input: &str) -> u64 {
    let mut winnings = 0;
    let mut lines = input.lines();
    let mut hands = common::parse_input(&mut lines);
    hands.sort_by(|a, b| part1::sort_hands(a, b));

    for i in 0..hands.len() {
        winnings += hands[i].bid * (i as u64 + 1);
    }
    winnings
}

fn part2(input: &str) -> u64 {
    let mut winnings = 0;
    let mut lines = input.lines();
    let mut hands = common::parse_input(&mut lines);
    hands.sort_by(|a, b| part2::sort_hands(a, b));

    for i in 0..hands.len() {
        winnings += hands[i].bid * (i as u64 + 1);
    }
    winnings
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

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 6440);
    }

    #[test]
    fn test_par2() {
        assert_eq!(part2(INPUT), 5905);
    }
}
