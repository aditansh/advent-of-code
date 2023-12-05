mod part1;
mod part2;

use part1::is_part;
use part2::get_gear_ratio;
use std::env;
use std::fs::File;
use std::io::Read;

fn part1(input: &str) -> u32 {
    let mut sum = 0;

    for (i, line) in input.lines().enumerate() {
        let mut number = 0;

        for (j, char) in line.chars().enumerate() {
            if char.is_numeric() {
                number = number * 10 + char.to_digit(10).unwrap();
            }

            if (j + 1 == line.len() || !line.chars().nth(j + 1).unwrap().is_numeric())
                && number != 0
            {
                if is_part(input, number, i, j) {
                    sum += number;
                }
                number = 0;
            }
        }
    }

    sum
}

fn part2(input: &str) -> u32 {
    let mut sum = 0;

    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char == '*' {
                sum += get_gear_ratio(input, i, j);
            }
        }
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
        let input = "467..114..\n\
                        ...*......\n\
                        ..35..633.\n\
                        ......#...\n\
                        617*......\n\
                        .....+.58.\n\
                        ..592.....\n\
                        ......755.\n\
                        ...$.*....\n\
                        .664.598..";
        assert_eq!(part1(input), 4361);
    }

    #[test]
    fn test_part2() {
        let input = "467..114..\n\
                        ...*......\n\
                        ..35..633.\n\
                        ......#...\n\
                        617*......\n\
                        .....+.58.\n\
                        ..592.....\n\
                        ......755.\n\
                        ...$.*....\n\
                        .664.598..";
        assert_eq!(part2(input), 467835);
    }
}
