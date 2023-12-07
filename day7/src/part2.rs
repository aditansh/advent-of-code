use std::cmp::Ordering;

use crate::Hand;

fn categorize_hand(cards: &str) -> u8 {
    let card_counts: std::collections::HashMap<char, usize> =
        cards
            .chars()
            .fold(std::collections::HashMap::new(), |mut counts, card| {
                *counts.entry(card).or_insert(0) += 1;
                counts
            });

    let max_count = card_counts.values().cloned().max().unwrap_or(0);
    let unique_labels = card_counts.len();

    match (max_count, unique_labels) {
        (5, 1) => 6,
        (4, 2) => {
            for i in card_counts.keys() {
                if *i == 'J' {
                    return 6;
                }
            }
            return 5;
        }
        (3, 2) => {
            for i in card_counts.keys() {
                if *i == 'J' {
                    return 6;
                }
            }
            return 4;
        }
        (3, 3) => {
            for i in card_counts.keys() {
                if *i == 'J' {
                    return 5;
                }
            }
            return 3;
        }
        (2, 3) => {
            for i in card_counts.iter() {
                if *i.0 == 'J' {
                    if *i.1 == 2 {
                        return 5;
                    } else {
                        return 4;
                    }
                }
            }
            return 2;
        }
        (2, 4) => {
            for i in card_counts.keys() {
                if *i == 'J' {
                    return 3;
                }
            }
            return 1;
        }
        _ => {
            for i in card_counts.keys() {
                if *i == 'J' {
                    return 1;
                }
            }
            return 0;
        }
    }
}

pub fn card_value(card: char) -> u8 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        _ => card.to_digit(10).unwrap() as u8,
    }
}

fn compare_values(a: &str, b: &str) -> Ordering {
    let a_value = a.chars().collect::<Vec<char>>();
    let b_value = b.chars().collect::<Vec<char>>();

    for i in 0..5 {
        if a_value[i] == b_value[i] {
            continue;
        } else {
            if card_value(a_value[i]) > card_value(b_value[i]) {
                return Ordering::Greater;
            } else {
                return Ordering::Less;
            }
        }
    }

    Ordering::Equal
}

pub fn sort_hands(a: &Hand, b: &Hand) -> Ordering {
    let a_hand = categorize_hand(&a.value);
    let b_hand = categorize_hand(&b.value);

    if a_hand == b_hand {
        compare_values(&a.value, &b.value)
    } else {
        if a_hand > b_hand {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}
