// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

type InputType = (Vec<u8>, Vec<u8>);
type SolutionType = usize;

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    // .??..??...?##. 1,1,3
    input
        .lines()
        .map(|line| {
            let line = line.split_once(' ').expect("No game separator");
            let springs = Vec::from(line.0.as_bytes());
            let groups = line
                .1
                .split(',')
                .map(|n| n.parse().expect("number"))
                .collect();
            (springs, groups)
        })
        .collect()
}

fn is_valid(springs: &Vec<u8>, groups: &Vec<u8>) -> bool {
    let mut group_index = 0;
    let mut is_spring_group = springs[0] == b'#';
    let mut group_len = 0;
    for c in springs {
        // println!("Testing {}", *c as char);
        if *c == b'#' {
            if is_spring_group {
                group_len += 1;
                if group_len > groups[group_index] {
                    // println!("{:?} has too large ({}) group, should be {}", springs, group_len, groups[group_index]);
                    return false;
                }
            } else {
                if group_index >= groups.len() {
                    // println!("{:?} has too many ({}) groups, should be {}", springs, group_index, groups.len());
                    return false;
                }
                is_spring_group = true;
                group_len = 1;
            }
        } else if is_spring_group {
            is_spring_group = false;
            if group_len != groups[group_index] {
                // println!("{:?} has incorrect ({}) group size, should be {}", springs, group_len, groups[group_index]);
                return false;
            }
            group_len = 0;
            group_index += 1;
        }
    }
    if is_spring_group {
        group_len == groups[group_index] && group_index == groups.len() - 1
    } else {
        group_index == groups.len()
    }
}

#[aoc(day12, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    let mut scratch = vec![];
    data.iter()
        .map(|(springs, groups)| {
            let mut arrangements = 0;
            let unknowns = springs.iter().filter(|c| **c == b'?').count();
            if unknowns == 0 {
                if is_valid(springs, groups) {
                    arrangements = 1;
                }
            } else {
                for num in 0..(1 << unknowns) {
                    let mut num = num;
                    scratch.clear();
                    for c in springs {
                        scratch.push(if *c == b'?' {
                            let c = if num & 1 == 1 { b'#' } else { b'.' };
                            num >>= 1;
                            c
                        } else {
                            *c
                        });
                    }
                    if is_valid(&scratch, groups) {
                        /*
                        print!("VALID: ");
                        for c in &scratch {
                            print!("{}", *c as char);
                        }
                        println!();
                        */
                        arrangements += 1;
                    }
                }
            }
            // println!("{:?} - {} arrangements", springs, arrangements);
            arrangements
        })
        .sum()
}

#[aoc(day12, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    let mut scratch = vec![];
    data.iter()
        .map(|(spring, group)| {
            let mut spring2 = vec![];
            spring2.extend_from_slice(spring);
            spring2.push(b'?');
            spring2.extend_from_slice(spring);
            spring2.push(b'?');
            spring2.extend_from_slice(spring);
            spring2.push(b'?');
            spring2.extend_from_slice(spring);
            spring2.push(b'?');
            spring2.extend_from_slice(spring);
            let mut groups2 = vec![];
            groups2.extend_from_slice(group);
            groups2.extend_from_slice(group);
            groups2.extend_from_slice(group);
            groups2.extend_from_slice(group);
            groups2.extend_from_slice(group);
            (spring2, groups2)
        })
        .map(|(springs, groups)| {
            let mut arrangements = 0;
            let unknowns = springs.iter().filter(|c| **c == b'?').count();
            if unknowns == 0 {
                if is_valid(&springs, &groups) {
                    arrangements = 1;
                }
            } else {
                for num in 0..(1u128 << unknowns) {
                    let mut num = num;
                    scratch.clear();
                    for c in &springs {
                        scratch.push(if *c == b'?' {
                            let c = if num & 1 == 1 { b'#' } else { b'.' };
                            num >>= 1u128;
                            c
                        } else {
                            *c
                        });
                    }
                    /*
                    print!("Testing: ");
                    for c in &scratch {
                        print!("{}", *c as char);
                    }
                    println!(" {:?}", groups);
                    */
                    if is_valid(&scratch, &groups) {
                        /*
                        print!("VALID: ");
                        for c in &scratch {
                            print!("{}", *c as char);
                        }
                        println!();
                        */
                        arrangements += 1;
                    }
                }
            }
            // println!("{:?} - {} arrangements", springs, arrangements);
            arrangements
        })
        .sum()
}
