use std::str::{FromStr, Lines};

type Val = u64;

pub fn get_time(input: &mut Lines) -> Val {
    let time_line = input.next().unwrap();

    let times = time_line[5..]
        .split_whitespace()
        .map(|s| Val::from_str(s).unwrap())
        .collect::<Vec<Val>>();

    let number = times
        .into_iter()
        .map(|num| num.to_string())
        .collect::<String>();

    number.parse::<Val>().unwrap()
}

pub fn get_distance(input: &mut Lines) -> Val {
    let distance_line = input.next().unwrap();

    let distances = distance_line[10..]
        .split_whitespace()
        .map(|s| Val::from_str(s).unwrap())
        .collect::<Vec<Val>>();

    let distance = distances
        .into_iter()
        .map(|num| num.to_string())
        .collect::<String>();

    distance.parse::<Val>().unwrap()
}

pub fn ways(time: Val, distance: Val) -> Val {
    let mut count = 0;
    for i in 1..time {
        if (time - i) * i > distance {
            count += 1;
        }
    }
    count
}
