// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use super::world::*;
use std::collections::{HashMap, HashSet};
use Dir::*;

type SolutionType = usize;

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Map {
    Map::from_string_with_border(input)
}

fn is_walkable(c: u8) -> bool {
    matches!(c, b'.' | b'<' | b'>' | b'^' | b'v')
}

fn is_node(map: &Map, pos: Point, c: u8) -> bool {
    if is_walkable(c) {
        let mut count = 0;
        for dir in [North, South, East, West] {
            let new_pos = pos.walk(dir);
            if Some(c) != map.get_at(new_pos) && is_walkable(c) {
                count += 1;
                if count > 2 {
                    return true;
                }
            }
        }
    }
    false
}

#[aoc(day23, part1)]
pub fn solve_part1(map: &Map) -> SolutionType {
    let start = Point { x: 2, y: 1 };
    let goal = Point {
        x: map.get_width() - 3,
        y: map.get_height() - 2,
    };

    let mut frontier = Vec::new();
    frontier.push((HashSet::new(), start));
    let mut most_steps = 0;
    while let Some((mut steps, pos)) = frontier.pop() {
        if !steps.insert(pos) {
            continue;
        }
        if pos == goal {
            most_steps = most_steps.max(steps.len() - 1);
        }
        for dir in [North, South, East, West] {
            if matches!(
                (map.get_at_unchecked(pos), dir),
                (b'.', _) | (b'^', North) | (b'v', South) | (b'>', East) | (b'<', West)
            ) {
                let new_pos = pos.walk(dir);
                if is_walkable(map.get_at_unchecked(new_pos)) {
                    frontier.push((steps.clone(), new_pos));
                }
            }
        }
    }

    most_steps
}

#[aoc(day23, part2)]
pub fn solve_part2(map: &Map) -> SolutionType {
    let start = Point { x: 2, y: 1 };
    let goal = Point {
        x: map.get_width() - 3,
        y: map.get_height() - 2,
    };

    let mut nodes = vec![];
    for (pos, c) in map.iter() {
        if is_node(map, pos, c) {
            nodes.push(pos);
        }
    }
    let pos_to_node: HashMap<_, _> = nodes.iter().enumerate().map(|(k, &v)| (v, k)).collect();

    let start = *pos_to_node.get(&start).expect("Start node added");
    let goal = *pos_to_node.get(&goal).expect("Goal node added");

    let edges: Vec<_> = nodes
        .iter()
        .map(|&pos| {
            let mut frontier = Vec::new();
            let mut edges = Vec::new();
            for dir in [North, South, East, West] {
                let new_pos = pos.walk(dir);
                if is_walkable(map.get_at_unchecked(new_pos)) {
                    frontier.push((0, pos, new_pos));
                }
            }
            while let Some((mut steps, old_pos, pos)) = frontier.pop() {
                steps += 1;
                if let Some(next_node) = pos_to_node.get(&pos) {
                    edges.push((next_node, steps));
                } else {
                    for dir in [North, South, East, West] {
                        let new_pos = pos.walk(dir);
                        if new_pos != old_pos && is_walkable(map.get_at_unchecked(new_pos)) {
                            frontier.push((steps, pos, new_pos));
                        }
                    }
                }
            }
            edges
        })
        .collect();

    let mut frontier = Vec::new();
    frontier.push((HashSet::new(), 0, start));
    let mut most_steps = 0;
    while let Some((mut visited, steps, node)) = frontier.pop() {
        if !visited.insert(node) {
            continue;
        }
        if node == goal {
            most_steps = most_steps.max(steps);
        }
        for (&new_node, new_steps) in &edges[node] {
            frontier.push((visited.clone(), steps + *new_steps, new_node));
        }
    }

    most_steps
}
