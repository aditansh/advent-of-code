use std::str::Lines;

use crate::Hand;

pub fn parse_input(input: &mut Lines) -> Vec<Hand> {
    let mut hands: Vec<Hand> = Vec::new();
    for line in input {
        let mut hand: Hand = Hand {
            value: String::new(),
            bid: 0,
        };
        let split = line
            .split(" ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        hand.value = split[0].clone();
        hand.bid = split[1].parse::<u64>().unwrap();

        hands.push(hand);
    }

    hands
}
