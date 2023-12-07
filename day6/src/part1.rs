use std::str::{FromStr, Lines};

type Val = u64;

pub fn get_times(input: &mut Lines) -> Vec<Val> {
    let time_line = input.next().unwrap();

    time_line[5..]
        .split_whitespace()
        .map(|s| Val::from_str(s).unwrap())
        .collect::<Vec<Val>>()
}

pub fn get_distances(input: &mut Lines) -> Vec<Val> {
    let distance_line = input.next().unwrap();

    distance_line[10..]
        .split_whitespace()
        .map(|s| Val::from_str(s).unwrap())
        .collect::<Vec<Val>>()
}

pub fn ways(times: &[Val], distances: &[Val]) -> Vec<Val> {
    let mut ways = Vec::new();

    for (&time, &distance) in times.iter().zip(distances.iter()) {
        let mut count = 0;
        for i in 1..time {
            if (time - i) * i > distance {
                count += 1;
            }
        }
        ways.push(count);
    }
    ways
}
