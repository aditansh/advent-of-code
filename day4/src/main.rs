use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

fn get_winners(input: &str, winners: &mut Vec<u32>, entries: &mut Vec<u32>) {
    let part = input.split(|c| c == ':' || c == '|').collect::<Vec<&str>>();

    for i in part[1].split_whitespace() {
        let num = i.parse::<u32>().unwrap();
        if num > 0 {
            winners.push(num);
        }
    }

    for i in part[2].split_whitespace() {
        let num = i.parse::<u32>().unwrap();
        if num > 0 {
            entries.push(num);
        }
    }
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut score = 0;
        let mut winners = Vec::new();
        let mut entries = Vec::new();
        get_winners(line, &mut winners, &mut entries);

        for i in entries.iter() {
            if winners.contains(i) {
                if score == 0 {
                    score = 1;
                } else {
                    score = 2 * score;
                }
            }
        }
        sum += score;
    }
    sum
}

fn get_multiplier(cards: &HashMap<usize, u32>, i: usize) -> u32 {
    if let Some(value) = cards.get(&i) {
        *value
    } else {
        1
    }
}

fn part2(input: &str) -> u32 {
    let mut sum = 0;
    let mut cards = HashMap::new();
    for i in 0..input.lines().count() {
        cards.insert(i, 1);
    }

    for i in 0..input.lines().count() {
        let multiplier = get_multiplier(&cards, i);
        let mut score = 1;
        let mut winners = Vec::new();
        let mut entries = Vec::new();
        get_winners(input.lines().nth(i).unwrap(), &mut winners, &mut entries);

        for j in entries.iter() {
            if winners.contains(j) {
                if let Some(value) = cards.get_mut(&(i + score)) {
                    *value = *value + multiplier;
                } else {
                    println!("Key not found in the dictionary");
                }
                score += 1;
            }
        }
    }

    for i in cards.values() {
        sum += i;
    }

    sum
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
        println!("Error: could not get the current directory");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part1(input), 13);
    }

    #[test]
    fn test_part2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part2(input), 30);
    }
}
