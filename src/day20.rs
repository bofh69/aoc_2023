// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashMap;
use std::collections::VecDeque;
use ModuleType::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ModuleType {
    Broadcaster(Vec<String>),
    FlipFlop(String, Vec<String>),
    Conjunction(String, Vec<String>),
}

struct Memory<'a> {
    flipflops: HashMap<&'a str, bool>,
    conjunctions: HashMap<&'a str, Vec<(&'a str, bool)>>,
}

const BROADCASTER: &str = "broadcaster";

type InputType = ModuleType;
type SolutionType = usize;

fn modules_to_hashmap(data: &[InputType]) -> HashMap<&str, &ModuleType> {
    data.iter()
        .map(|m| {
            let name = match m {
                FlipFlop(name, _) => name.as_str(),
                Conjunction(name, _) => name.as_str(),
                Broadcaster(_) => BROADCASTER,
            };
            (name, m)
        })
        .collect()
}

fn create_memories<'a>(modules: &'a HashMap<&'a str, &'a ModuleType>) -> Memory<'a> {
    let mut ff_memory = HashMap::new();
    let mut conj_memory = HashMap::new();

    for &module in modules.values() {
        if let FlipFlop(n, _) = module {
            ff_memory.insert(n.as_str(), false);
        }
        if let Conjunction(n, _) = module {
            conj_memory.insert(n.as_str(), Vec::<(&str, bool)>::new());
        }
    }
    for &module in modules.values() {
        let (name, dest) = match module {
            FlipFlop(name, dest) => (name.as_str(), dest),
            Conjunction(name, dest) => (name.as_str(), dest),
            Broadcaster(dest) => (BROADCASTER, dest),
        };
        for dest_name in dest {
            if let Some(Conjunction(_, _)) = modules.get(dest_name.as_str()) {
                let list = conj_memory.get_mut(dest_name.as_str()).expect("conjuction");
                list.push((name, false));
            }
        }
    }
    Memory {
        flipflops: ff_memory,
        conjunctions: conj_memory,
    }
}

fn pulse(
    module: &str,
    is_high: bool,
    memory: &mut Memory,
    modules: &HashMap<&str, &ModuleType>,
) -> (usize, usize, bool) {
    let mut lows = 0;
    let mut highs = 0;

    let mut current_pulses = VecDeque::new();
    current_pulses.push_back(("button", module, is_high));

    let mut found_rx = false;

    while let Some((emitting_name, current_name, is_high)) = current_pulses.pop_front() {
        /*
        println!(
            "{} -{}-> {}",
            emitting_name,
            if is_high { "high" } else { "low" },
            current_name
        );
        */
        if current_name == "rx" && !is_high {
            found_rx = true;
        }
        if is_high {
            highs += 1;
        } else {
            lows += 1;
        }
        if let Some(&current_module) = modules.get(current_name) {
            match current_module {
                Broadcaster(dests) => {
                    for dest in dests {
                        current_pulses.push_back((BROADCASTER, dest, is_high));
                    }
                }
                FlipFlop(_, dests) => {
                    if !is_high {
                        let mem = memory.flipflops.get_mut(current_name).expect("FF memory");
                        *mem = !*mem;
                        for dest in dests {
                            current_pulses.push_back((current_name, dest, *mem));
                        }
                    }
                }
                Conjunction(_, dests) => {
                    let mem = memory
                        .conjunctions
                        .get_mut(current_name)
                        .expect("conj memory");
                    for mem in mem.iter_mut() {
                        if mem.0 == emitting_name {
                            mem.1 = is_high;
                            break;
                        }
                    }
                    let new_pulse = mem.iter().any(|(_, state)| !state);
                    for dest in dests {
                        current_pulses.push_back((current_name, dest, new_pulse));
                    }
                }
            }
        }
    }
    (lows, highs, found_rx)
}

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    input
        .lines()
        .map(|line| {
            let line = line.split_once(" -> ").expect("->");
            let destinations = line.1.split(", ").map(|s| s.to_string()).collect();
            if let Some(name) = line.0.strip_prefix('%') {
                FlipFlop(name.to_string(), destinations)
            } else if let Some(name) = line.0.strip_prefix('&') {
                Conjunction(name.to_string(), destinations)
            } else {
                Broadcaster(destinations)
            }
        })
        .collect()
}

#[aoc(day20, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    let modules = modules_to_hashmap(data);
    let mut memory = create_memories(&modules);

    let (mut lows, mut highs) = (0, 0);
    for _i in 0..1000 {
        let (nl, nh, _) = pulse(BROADCASTER, false, &mut memory, &modules);
        lows += nl;
        highs += nh;
    }

    highs * lows
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

#[aoc(day20, part2)]
pub fn solve_part2(data: &[InputType]) -> u64 {
    /*
     * In my input:
     *
     * rx is only connected to dr, a flip flop.
     *
     * The network implements four counters/clocks, they are connected to dr.
     *
     * By only having one counter connected to dr/rx at a time one can find that counter's
     * period.
     *
     * Then the LCM of the periods give the result.
     *
     * This assumes each counter has an even periodicity.
     */
    let mods_to_remove: Vec<_> = data
        .iter()
        .filter_map(|m| {
            let (name, dests) = match m {
                Broadcaster(dests) => (BROADCASTER, dests),
                FlipFlop(n, dests) => (n.as_str(), dests),
                Conjunction(n, dests) => (n.as_str(), dests),
            };
            for dest in dests {
                if dest == "dr" {
                    return Some(name);
                }
            }
            None
        })
        .collect();

    mods_to_remove
        .iter()
        .map(|&mod_to_keep| {
            let mut modules = modules_to_hashmap(data);
            for &mod_to_remove in &mods_to_remove {
                if mod_to_remove != mod_to_keep {
                    modules.remove(mod_to_remove);
                }
            }

            let mut memory = create_memories(&modules);
            let mut i = 1;
            while let (_, _, false) = pulse(BROADCASTER, false, &mut memory, &modules) {
                i += 1;
            }
            i
        })
        .fold(1, lcm)
}
