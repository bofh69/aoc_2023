// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashMap;
use regex::Regex;

type InputType = (String, HashMap<String, (String, String)>);
type SolutionType = usize;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> InputType {
    // RL
    //
    // AAA = (BBB, CCC)
    let mut input = input.lines();
    let path = input.next().expect("Path").to_string();
    input.next();

    let re = Regex::new(r"(...) = \((...), (...)\)").expect("Compilable regex");

    let tree = input
        .map(|s| {
            let caps = re
                .captures(s)
                .unwrap_or_else(|| panic!("Didn't match: {}", s));
            (
                caps[1].to_string(),
                (caps[2].to_string(), caps[3].to_string()),
            )
        })
        .collect();
    (path, tree)
}

#[aoc(day8, part1)]
pub fn solve_part1(data: &InputType) -> SolutionType {
    let mut pos = "AAA";

    let mut length = 0;
    for c in data.0.chars().cycle() {
        if pos == "ZZZ" {
            break;
        }
        length += 1;
        pos = match c {
            'L' => &data.1.get(pos).expect("Find next").0,
            'R' => &data.1.get(pos).expect("Find next").1,
            _ => panic!("Unknown direction"),
        };
    }

    length
}

/*
fn find_cycle(pos: &str, data: &InputType) -> (u64, u64) {
    println!("Solving for {}", pos);
    let mut pos = pos;
    let mut path: Vec<&str> = vec![pos];
    for c in data.0.chars().cycle() {
        pos = match c {
            'L' => &data.1.get(pos).expect("Find next").0,
            'R' => &data.1.get(pos).expect("Find next").1,
            _ => panic!("Unknown direction"),
        };
        if path.iter().filter(|x| **x == pos).count() == 2 && pos.as_bytes()[2] == b'Z' {
            let mut length = 0;
            let mut offset = 0;
            for old_pos in &path {
                if old_pos == &pos {
                    if offset == 0 {
                        println!("Cycle started at {}", length);
                        offset = length;
                    } else {
                        println!(
                            "Cycle starts again at {}, diff={}, before offset={}, before={}",
                            length,
                            length - offset,
                            path[offset as usize - 1],
                            path[length as usize - 1]
                        );
                    }
                } else if old_pos.as_bytes()[2] == b'Z' {
                    println!("Goal at {}, diff={}", length, length - offset);
                }
                length += 1;
            }
            println!(
                "Total length: {}, diff={}, before={}",
                length,
                length - offset,
                path[length as usize - 1]
            );
            return (offset, length - offset);
        }
        path.push(pos);
    }
    panic!("Shouldn't reach here");
}
*/

fn find_first_goal(pos: &str, data: &InputType) -> u64 {
    // The periods, in the input, are always equal to the length from
    // the start to the first goal, making it possible to skip finding
    // the real loop and also makes it possible to use lcm on the results.
    let mut pos = pos;
    let mut length = 0;
    for c in data.0.chars().cycle() {
        pos = match c {
            'L' => &data.1.get(pos).expect("Find next").0,
            'R' => &data.1.get(pos).expect("Find next").1,
            _ => panic!("Unknown direction"),
        };
        length += 1;
        if pos.as_bytes()[2] == b'Z' {
            return length;
        }
    }
    panic!("Shouldn't reach here");
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut max = a.max(b);
    let mut min = a.min(b);

    loop {
        let r = max % min;
        if r == 0 {
            return min;
        }
        max = min;
        min = r;
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}

#[aoc(day8, part2)]
pub fn solve_part2(data: &InputType) -> u64 {
    let positions: Vec<_> = data.1.keys().filter(|s| s.as_bytes()[2] == b'A').collect();

    let results: Vec<_> = positions
        .iter()
        .map(|pos| find_first_goal(pos, data))
        .collect();
    results.iter().fold(1, |acc, &v| lcm(acc, v))
}
