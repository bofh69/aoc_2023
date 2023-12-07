// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

type InputType = ([u8; 5], SolutionType);
type SolutionType = usize;

use std::cmp::Ordering;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    // 32T3K 795
    input
        .lines()
        .map(|line| {
            let line = line.split_once(' ').expect("No hand separator");
            let mut hand = [0; 5];
            for (i, c) in line.0.chars().enumerate() {
                hand[i] = match c {
                    'A' => 14u8,
                    'K' => 13u8,
                    'Q' => 12u8,
                    'J' => 11u8,
                    'T' => 10u8,
                    '1'..='9' => c.to_digit(10).expect("digit") as u8,
                    _ => panic!("Unexpected char {}", c),
                }
            }
            (hand, line.1.parse().expect("bet"))
        })
        .collect()
}

fn get_type(a: &[u8; 5]) -> u8 {
    let mut count = [0; 5];
    let mut card: [u8; 5] = [0; 5];
    let mut n_cards = 0;

    'cards: for current_card in a {
        for j in 0..n_cards {
            if *current_card == card[j] {
                count[j] += 1;
                continue 'cards;
            }
        }
        card[n_cards] = *current_card;
        count[n_cards] = 1;
        n_cards += 1;
    }
    count.sort();
    if count[4] == 5 {
        // Five of a kind
        return 7;
    }
    if count[4] == 4 {
        // Four of a kind
        return 6;
    }
    if count[4] == 3 && count[3] == 2 {
        // Full house
        return 5;
    }
    if count[4] == 3 {
        // Three of a kind
        return 4;
    }
    if count[4] == 2 && count[3] == 2 {
        // Two pair
        return 3;
    }
    if count[4] == 2 {
        // One pair
        return 2;
    }
    1
}

fn compare_type(a: &[u8; 5], b: &[u8; 5]) -> Ordering {
    get_type(a).cmp(&get_type(b))
}

fn compare_card(a: u8, b: u8) -> Ordering {
    a.cmp(&b)
}

fn compare(a: &[u8; 5], b: &[u8; 5]) -> Ordering {
    let ord = compare_type(a, b);
    if ord == Ordering::Equal {
        for i in 0..5 {
            let ord = compare_card(a[i], b[i]);
            if ord != Ordering::Equal {
                return ord;
            }
        }
    }
    ord
}

#[aoc(day7, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    let mut data = Vec::from(data);
    data.sort_by(|a, b| compare(&a.0, &b.0));
    data.iter()
        .enumerate()
        .map(|(rank, (_, bet))| (rank + 1) * *bet)
        .sum()
}

fn get_type2(a: &[u8; 5]) -> u8 {
    let mut count = [0; 5];
    let mut card: [u8; 5] = [0; 5];
    let mut n_cards = 0;
    let mut jokers = 0;

    'cards: for current_card in a {
        if *current_card == 11 {
            jokers += 1;
            continue 'cards;
        }
        for j in 0..n_cards {
            if *current_card == card[j] {
                count[j] += 1;
                continue 'cards;
            }
        }
        card[n_cards] = *current_card;
        count[n_cards] = 1;
        n_cards += 1;
    }
    count.sort();
    if jokers + count[4] == 5 {
        // Five of a kind
        return 7;
    }
    if jokers + count[4] == 4 {
        // Four of a kind
        return 6;
    }
    if (jokers + count[4] == 3 && count[3] == 2) || (jokers + count[3] == 3 && count[4] == 2) {
        // Full house
        return 5;
    }
    if jokers + count[4] == 3 {
        // Three of a kind
        return 4;
    }
    if count[4] == 2 && (jokers + count[3] == 2) {
        // Two pair
        return 3;
    }
    if jokers + count[4] == 2 {
        // One pair
        return 2;
    }
    1
}

fn compare_type2(a: &[u8; 5], b: &[u8; 5]) -> Ordering {
    get_type2(a).cmp(&get_type2(b))
}

fn compare_card2(a: u8, b: u8) -> Ordering {
    let mut a = a;
    let mut b = b;
    if a == 11 {
        a = 1;
    }
    if b == 11 {
        b = 1;
    }
    a.cmp(&b)
}

fn compare2(a: &[u8; 5], b: &[u8; 5]) -> Ordering {
    let ord = compare_type2(a, b);
    if ord == Ordering::Equal {
        for i in 0..5 {
            let ord = compare_card2(a[i], b[i]);
            if ord != Ordering::Equal {
                return ord;
            }
        }
    }
    ord
}

#[aoc(day7, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    let mut data = Vec::from(data);
    data.sort_by(|a, b| compare2(&a.0, &b.0));
    data.iter()
        .enumerate()
        .map(|(rank, (_, bet))| (rank + 1) * *bet)
        .sum()
}

#[cfg(test)]
mod test {
    #[test]
    fn compare_first_and_second() {}
}
