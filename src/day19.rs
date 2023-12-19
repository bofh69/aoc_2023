// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};
// use rayon::prelude::*;
use std::collections::HashMap;
// use std::ops::Range;
use regex::Regex;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Action {
    Accept,
    Reject,
    Goto(String),
}

#[derive(Debug)]
pub struct Rule {
    field: u8,
    is_less_than: bool,
    value: i32,
    action: Action,
}

type InputType = (HashMap<String, Vec<Rule>>, Vec<[i32; 4]>);
type SolutionType = usize;

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> InputType {
    let workflow = Regex::new(r"([^{]+)\{(.+)\}").expect("Compilable regex");
    let rule_re = Regex::new(r"(.+)([<>])([0-9]+):(.+)").expect("Compilable regex");

    // qqz{s>2770:qs,m<1801:hdj,R}
    let workflows = input
        .lines()
        .take_while(|s| !s.is_empty())
        .map(|s| {
            let caps = workflow.captures(s).expect("Matching");
            let name = caps[1].to_string();
            let mut rules = vec![];
            for rule in caps[2].split(',') {
                if let Some(caps) = rule_re.captures(rule) {
                    let is_less_than = caps[2].eq("<");
                    let field = match &caps[1] {
                        "x" => 0,
                        "m" => 1,
                        "a" => 2,
                        "s" => 3,
                        _ => unreachable!("Unknown field"),
                    };
                    let value = caps[3].parse().expect("comparision number");
                    let action;
                    if &caps[4] == "A" {
                        action = Action::Accept;
                    } else if &caps[4] == "R" {
                        action = Action::Reject;
                    } else {
                        action = Action::Goto(caps[4].to_string());
                    }
                    rules.push(Rule {
                        field,
                        is_less_than,
                        value,
                        action,
                    });
                } else {
                    let action;
                    if rule == "A" {
                        action = Action::Accept;
                    } else if rule == "R" {
                        action = Action::Reject;
                    } else {
                        action = Action::Goto(rule.to_string());
                    }
                    rules.push(Rule {
                        field: 0,
                        is_less_than: true,
                        value: i32::MAX,
                        action,
                    });
                }
            }
            (name, rules)
        })
        .collect();

    let re =
        Regex::new(r"\{x=([0-9]*),m=([0-9]*),a=([0-9]*),s=([0-9]*)\}").expect("Compilable regex");

    let parts = input
        .lines()
        .filter_map(|s| {
            re.captures(s).map(|cap| [
                    cap[1].parse().expect("Number"),
                    cap[2].parse().expect("Number"),
                    cap[3].parse().expect("Number"),
                    cap[4].parse().expect("Number"),
                ])
        })
        .collect();

    (workflows, parts)
}

#[aoc(day19, part1)]
pub fn solve_part1(data: &InputType) -> SolutionType {
    let (workflows, parts) = data;

    parts
        .iter()
        .filter(|p| {
            let mut wf = workflows.get("in").unwrap();
            loop {
                'next_wf: for rule in wf {
                    if rule.is_less_than {
                        if p[rule.field as usize] >= rule.value {
                            continue;
                        }
                    } else if p[rule.field as usize] <= rule.value {
                        continue;
                    }
                    match &rule.action {
                        Action::Accept => return true,
                        Action::Reject => return false,
                        Action::Goto(s) => {
                            wf = workflows.get(s).unwrap();
                            break 'next_wf;
                        }
                    }
                }
            }
        })
        .map(|p| p.iter().sum::<i32>())
        .sum::<i32>() as SolutionType
}

#[aoc(day19, part2)]
pub fn solve_part2(_data: &InputType) -> SolutionType {
    0
}
