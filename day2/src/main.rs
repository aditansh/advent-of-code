use std::cmp::max;
use std::fs::File;
use std::io::Read;

const RED: i32 = 12;
const GREEN: i32 = 13;
const BLUE: i32 = 14;

fn verify(color: &str, value: i32) -> bool {
    match color {
        "red" => value <= RED,
        "green" => value <= GREEN,
        "blue" => value <= BLUE,
        _ => false,
    }
}

fn part1(input: &str) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut valid = true;
        let mut split = line.split(":");
        let game = split.next().unwrap().trim();
        let parts = game.split(" ").collect::<Vec<&str>>();
        let id = parts.get(1).unwrap_or(&"0").parse::<i32>().unwrap();
        let games = split
            .next()
            .unwrap()
            .trim()
            .split(";")
            .collect::<Vec<&str>>();
        for game in games {
            let game_parts = game.split(",").collect::<Vec<&str>>();
            for game_part in game_parts {
                if let [value_str, color] = game_part
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .as_slice()
                {
                    let color = color.trim();
                    let value = value_str.trim().parse::<i32>().unwrap_or(0);
                    if !verify(color, value) {
                        valid = false;
                    }
                }
            }
        }
        if valid {
            sum += id;
        }
    }
    sum
}

fn part2(input: &str) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        let mut split = line.split(":");
        let game = split.next().unwrap().trim();
        let _parts = game.split(" ").collect::<Vec<&str>>();
        let games = split
            .next()
            .unwrap()
            .trim()
            .split(";")
            .collect::<Vec<&str>>();
        for game in games {
            let game_parts = game.split(",").collect::<Vec<&str>>();
            for game_part in game_parts {
                if let [value_str, color] = game_part
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .as_slice()
                {
                    let color = color.trim();
                    let value = value_str.trim().parse::<i32>().unwrap_or(0);
                    match color {
                        "red" => red = max(red, value),
                        "green" => green = max(green, value),
                        "blue" => blue = max(blue, value),
                        _ => (),
                    }
                }
            }
        }
        sum += red * green * blue;
    }
    sum
}


fn main() {
    let mut file = File::open("./../input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2(&contents));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
                     Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
                     Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
                     Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
                     Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part1(input), 8);
    }

    #[test]
    fn test_part2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
                     Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
                     Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
                     Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
                     Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part2(input), 2286);
    }


}