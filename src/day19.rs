// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashMap;
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
    value: i16,
    action: Action,
}

type InputType = (HashMap<String, Vec<Rule>>, Vec<[i16; 4]>);
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
                        value: i16::MAX,
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
            re.captures(s).map(|cap| {
                [
                    cap[1].parse().expect("Number"),
                    cap[2].parse().expect("Number"),
                    cap[3].parse().expect("Number"),
                    cap[4].parse().expect("Number"),
                ]
            })
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
        .map(|p| p.iter().map(|n| i32::from(*n)).sum::<i32>())
        .sum::<i32>() as SolutionType
}

/*
fn intersect_range(a: &mut [i16; 8], b: &[i16; 8]) {
    for i in 0..4 {
        a[i * 2] = a[i * 2].max(b[i * 2]);
        a[i * 2 + 1] = a[i * 2 + 1].min(b[i * 2 + 1]);
    }
}
*/

fn is_proper_subset(a: &[i16; 8], b: &[i16; 8]) -> bool {
    if a == b {
        return false;
    }
    for i in 0..4 {
        if a[i * 2] < b[i * 2] || a[i * 2 + 1] > b[i * 2 + 1] {
            return false;
        }
    }
    true
}

fn update_ranges(
    rules: &HashMap<String, Vec<Rule>>,
    wfn: &str,
    ranges: &[[i16; 8]],
) -> Vec<[i16; 8]> {
    let wf = rules.get(wfn).expect("Workflow");
    let mut result = vec![];
    let mut ranges = Vec::from(ranges);
    'range: for range in ranges.iter_mut() {
        for rule in wf {
            match (&rule.action, rule.is_less_than) {
                (Action::Accept, true) => {
                    // Rule is V[x] < y => ACCEPT:

                    if range[rule.field as usize * 2] < rule.value {
                        let mut new_range = *range;
                        new_range[rule.field as usize * 2 + 1] =
                            new_range[rule.field as usize * 2 + 1].min(rule.value);
                        result.push(new_range);
                    }

                    if range[rule.field as usize * 2 + 1] <= rule.value {
                        continue 'range;
                    }

                    range[rule.field as usize * 2] = range[rule.field as usize * 2].max(rule.value);
                }
                (Action::Accept, false) => {
                    // Rule is V[x] > y => ACCEPT:

                    if range[rule.field as usize * 2 + 1] > rule.value + 1 {
                        let mut new_range = *range;
                        new_range[rule.field as usize * 2] =
                            new_range[rule.field as usize * 2].max(rule.value + 1);
                        result.push(new_range);
                    }

                    if range[rule.field as usize * 2] >= rule.value {
                        continue 'range;
                    }

                    range[rule.field as usize * 2 + 1] =
                        range[rule.field as usize * 2 + 1].min(rule.value + 1);
                }
                (Action::Reject, true) => {
                    // Rule is V[x] < y => REJECT:
                    if range[rule.field as usize * 2 + 1] <= rule.value {
                        continue 'range;
                    }

                    range[rule.field as usize * 2] = range[rule.field as usize * 2].max(rule.value);
                }
                (Action::Reject, false) => {
                    // Rule is V[x] > y => REJECT:

                    if range[rule.field as usize * 2] >= rule.value {
                        continue 'range;
                    }

                    range[rule.field as usize * 2 + 1] =
                        range[rule.field as usize * 2 + 1].min(rule.value + 1);
                }
                (Action::Goto(ref s), true) => {
                    // Rule is V[x] < y => GOTO(x):

                    if range[rule.field as usize * 2] < rule.value {
                        let mut new_range = *range;
                        new_range[rule.field as usize * 2 + 1] =
                            new_range[rule.field as usize * 2 + 1].min(rule.value);
                        let mut new_ranges = update_ranges(rules, s, &[new_range]);
                        result.append(&mut new_ranges);
                    }

                    if range[rule.field as usize * 2 + 1] <= rule.value {
                        continue 'range;
                    }

                    range[rule.field as usize * 2] = range[rule.field as usize * 2].max(rule.value);
                }
                (Action::Goto(ref s), false) => {
                    // Rule is V[x] > y => GOTO(x):

                    if range[rule.field as usize * 2 + 1] > rule.value + 1 {
                        let mut new_range = *range;
                        new_range[rule.field as usize * 2] =
                            new_range[rule.field as usize * 2].max(rule.value + 1);
                        let mut new_ranges = update_ranges(rules, s, &[new_range]);
                        result.append(&mut new_ranges);
                    }

                    if range[rule.field as usize * 2] >= rule.value {
                        continue 'range;
                    }

                    range[rule.field as usize * 2 + 1] =
                        range[rule.field as usize * 2 + 1].min(rule.value + 1);
                }
            }
        }
    }

    result
        .iter()
        .filter(|&a| !result.iter().any(|b| is_proper_subset(a, b)))
        .copied()
        .collect()
}

#[aoc(day19, part2)]
pub fn solve_part2(data: &InputType) -> SolutionType {
    let ranges = [[1, 4001, 1, 4001, 1, 4001, 1, 4001]];

    let rules = &data.0;

    let ranges = update_ranges(rules, "in", &ranges);

    /*
    for ran in &ranges {
        println!(
            "[{:4}..{:4}, {:4}..{:4}, {:4}..{:4}, {:4}..{:4}]",
            ran[0], ran[1], ran[2], ran[3], ran[4], ran[5], ran[6], ran[7],
        );
    }
    */

    ranges
        .iter()
        .map(|r| {
            (r[1] - r[0]) as SolutionType
                * (r[3] - r[2]) as SolutionType
                * (r[5] - r[4]) as SolutionType
                * (r[7] - r[6]) as SolutionType
        })
        .sum()
}

#[cfg(test)]
mod test {
    /*
        #[test]
        fn test_intersect() {
            let mut a = [0, 2001, 2000, 4001, 1000, 3001, 0, 4001];
            let b = [0, 4001, 0, 4001, 0, 4001, 1000, 3001];

            super::intersect_range(&mut a, &b);
            assert_eq!(a, [0, 2001, 2000, 4001, 1000, 3001, 1000, 3001]);
        }
    */
    #[test]
    fn test_action_accept_less() {
        use super::*;
        let mut rules = HashMap::new();
        rules.insert(
            "in".to_string(),
            Vec::from([
                Rule {
                    field: 0,
                    is_less_than: true,
                    value: 5,
                    action: Action::Accept,
                },
                Rule {
                    field: 1,
                    is_less_than: true,
                    value: 5,
                    action: Action::Accept,
                },
            ]),
        );
        let res = update_ranges(&rules, "in", &[[0, 10, 0, 10, 0, 10, 0, 10]]);
        assert_eq!(
            res,
            Vec::from([[0, 5, 0, 10, 0, 10, 0, 10], [5, 10, 0, 5, 0, 10, 0, 10],])
        );
    }

    #[test]
    fn test_action_accept_greater() {
        use super::*;
        let mut rules = HashMap::new();
        rules.insert(
            "in".to_string(),
            Vec::from([
                Rule {
                    field: 0,
                    is_less_than: false,
                    value: 5,
                    action: Action::Accept,
                },
                Rule {
                    field: 1,
                    is_less_than: true,
                    value: 5,
                    action: Action::Accept,
                },
            ]),
        );
        let res = update_ranges(&rules, "in", &[[0, 10, 0, 10, 0, 10, 0, 10]]);
        assert_eq!(
            res,
            Vec::from([[6, 10, 0, 10, 0, 10, 0, 10], [0, 6, 0, 5, 0, 10, 0, 10],])
        );
    }

    #[test]
    fn test_action_reject_less() {
        use super::*;
        let mut rules = HashMap::new();
        rules.insert(
            "in".to_string(),
            Vec::from([
                Rule {
                    field: 0,
                    is_less_than: true,
                    value: 5,
                    action: Action::Reject,
                },
                Rule {
                    field: 0,
                    is_less_than: true,
                    value: i16::MAX,
                    action: Action::Accept,
                },
            ]),
        );
        let res = update_ranges(&rules, "in", &[[0, 10, 0, 10, 0, 10, 0, 10]]);
        assert_eq!(res, Vec::from([[5, 10, 0, 10, 0, 10, 0, 10]]));
    }

    #[test]
    fn test_action_reject_greater() {
        use super::*;
        let mut rules = HashMap::new();
        rules.insert(
            "in".to_string(),
            Vec::from([
                Rule {
                    field: 0,
                    is_less_than: false,
                    value: 5,
                    action: Action::Reject,
                },
                Rule {
                    field: 0,
                    is_less_than: true,
                    value: i16::MAX,
                    action: Action::Accept,
                },
            ]),
        );
        let res = update_ranges(&rules, "in", &[[0, 10, 0, 10, 0, 10, 0, 10]]);
        assert_eq!(res, Vec::from([[0, 6, 0, 10, 0, 10, 0, 10]]));
    }

    #[test]
    fn test_action_goto_less() {
        use super::*;
        let mut rules = HashMap::new();
        rules.insert(
            "acc".to_string(),
            Vec::from([Rule {
                field: 0,
                is_less_than: true,
                value: 20,
                action: Action::Accept,
            }]),
        );
        rules.insert(
            "in".to_string(),
            Vec::from([
                Rule {
                    field: 0,
                    is_less_than: true,
                    value: 5,
                    action: Action::Goto("acc".to_string()),
                },
                Rule {
                    field: 0,
                    is_less_than: true,
                    value: i16::MAX,
                    action: Action::Reject,
                },
            ]),
        );
        let res = update_ranges(&rules, "in", &[[0, 10, 0, 10, 0, 10, 0, 10]]);
        assert_eq!(res, Vec::from([[0, 5, 0, 10, 0, 10, 0, 10]]));
    }

    #[test]
    fn test_action_goto_greater() {
        use super::*;
        let mut rules = HashMap::new();
        rules.insert(
            "acc".to_string(),
            Vec::from([Rule {
                field: 0,
                is_less_than: true,
                value: 20,
                action: Action::Accept,
            }]),
        );
        rules.insert(
            "in".to_string(),
            Vec::from([
                Rule {
                    field: 0,
                    is_less_than: false,
                    value: 5,
                    action: Action::Goto("acc".to_string()),
                },
                Rule {
                    field: 0,
                    is_less_than: true,
                    value: i16::MAX,
                    action: Action::Reject,
                },
            ]),
        );
        let res = update_ranges(&rules, "in", &[[0, 10, 0, 10, 0, 10, 0, 10]]);
        assert_eq!(res, Vec::from([[6, 10, 0, 10, 0, 10, 0, 10]]));
    }
    #[test]
    fn test_action_accept_middle() {
        use super::*;
        let mut rules = HashMap::new();
        rules.insert(
            "in".to_string(),
            Vec::from([
                Rule {
                    field: 0,
                    is_less_than: true,
                    value: 4,
                    action: Action::Reject,
                },
                Rule {
                    field: 0,
                    is_less_than: true,
                    value: 6,
                    action: Action::Accept,
                },
                Rule {
                    field: 0,
                    is_less_than: false,
                    value: 5,
                    action: Action::Reject,
                },
            ]),
        );
        let res = update_ranges(&rules, "in", &[[0, 10, 0, 10, 0, 10, 0, 10]]);
        assert_eq!(res, Vec::from([[4, 6, 0, 10, 0, 10, 0, 10]]));
    }
}
