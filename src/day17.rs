// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use super::world::*;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use rayon::prelude::*;

type SolutionType = u16;

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Map {
    Map::from_string(input)
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct CostAndPoint<T: Eq + PartialEq>(SolutionType, T);

impl<T: Eq + PartialEq> Ord for CostAndPoint<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

impl<T: Eq + PartialEq> PartialOrd for CostAndPoint<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.0.cmp(&self.0))
    }
}

type PointData = (u8, Point, Dir);

fn bfs(map: &Map, from: Point, to: Point) -> SolutionType {
    let mut expanded = std::collections::HashMap::new();
    let mut to_expand = std::collections::BinaryHeap::new();
    to_expand.push(CostAndPoint(0, (1, from.walk(Dir::East), Dir::East)));
    to_expand.push(CostAndPoint(0, (1, from.walk(Dir::South), Dir::South)));
    while let Some(CostAndPoint(mut heat_loss, (steps, pos, dir))) = to_expand.pop() {
        if let Some(cost) = expanded.get_mut(&(steps, pos, dir)) {
            if *cost <= heat_loss {
                continue;
            }
            *cost = heat_loss;
        } else {
            expanded.insert((steps, pos, dir), heat_loss);
        }
        heat_loss += SolutionType::from(map.get_at_unchecked(pos) - b'0');
        if to == pos {
            // println!("{} ", expanded.len());
            return heat_loss;
        }
        let dir_left = dir.turn_cardinal_left();
        let pos_left = pos.walk(dir_left);
        if map.is_inside_map(pos_left) {
            to_expand.push(CostAndPoint(heat_loss, (1, pos_left, dir_left)));
        }
        let dir_right = dir.turn_cardinal_right();
        let pos_right = pos.walk(dir_right);
        if map.is_inside_map(pos_right) {
            to_expand.push(CostAndPoint(heat_loss, (1, pos_right, dir_right)));
        }
        if steps < 3 {
            let pos = pos.walk(dir);
            if map.is_inside_map(pos) {
                to_expand.push(CostAndPoint(heat_loss, (steps + 1, pos, dir)));
            }
        }
    }
    0
}

#[aoc(day17, part1)]
pub fn solve_part1(map: &Map) -> SolutionType {
    let goal = Point {
        x: map.get_width() - 1,
        y: map.get_height() - 1,
    };
    bfs(map, Point { x: 0, y: 0 }, goal)
}

// Turn into trait?
fn search_for_node(map: &Map, start: &[PointData], goal: Point) -> SolutionType {
    let mut expanded = std::collections::HashMap::new();
    let mut to_expand = std::collections::BinaryHeap::new();

    for node in start {
        to_expand.push(CostAndPoint(0, (0, *node)));
    }

    while let Some(CostAndPoint(_estimated_cost, (mut ack_cost, node_state))) = to_expand.pop() {
        if let Some(cost) = expanded.get_mut(&node_state) {
            if *cost <= ack_cost {
                continue;
            }
            *cost = ack_cost;
        } else {
            expanded.insert(node_state, ack_cost);
        }
        ack_cost += node_cost(map, &node_state);
        let estimated_cost = ack_cost + node_underestimate_cost_to_goal(&node_state, goal);
        if node_is_at_goal(goal, &node_state) {
            // println!("{} ", expanded.len());
            return ack_cost;
        }
        node_expand(&node_state, map, &mut to_expand, estimated_cost, ack_cost);
    }
    0
}

fn node_is_at_goal(goal: Point, node_state: &PointData) -> bool {
    goal == node_state.1
}

fn node_expand(
    node_state: &PointData,
    map: &Map,
    to_expand: &mut std::collections::BinaryHeap<CostAndPoint<(u16, PointData)>>,
    estimated_cost: u16,
    ack_cost: u16,
) {
    let &(steps, pos, dir) = node_state;
    if steps >= 4 {
        let dir_left = dir.turn_cardinal_left();
        let pos_left = pos.walk(dir_left);
        if map.is_inside_map(pos_left) {
            let new_state = (1, pos_left, dir_left);
            to_expand.push(CostAndPoint(estimated_cost, (ack_cost, new_state)));
        }
        let dir_right = dir.turn_cardinal_right();
        let pos_right = pos.walk(dir_right);
        if map.is_inside_map(pos_right) {
            let new_state = (1, pos_right, dir_right);
            to_expand.push(CostAndPoint(estimated_cost, (ack_cost, new_state)));
        }
    }
    if steps < 10 {
        let pos = pos.walk(dir);
        if map.is_inside_map(pos) {
            let new_state = (steps + 1, pos, dir);
            to_expand.push(CostAndPoint(estimated_cost, (ack_cost, new_state)));
        }
    }
}

fn node_cost(map: &Map, node_state: &PointData) -> u16 {
    SolutionType::from(map.get_at_unchecked(node_state.1) - b'0')
}

fn node_underestimate_cost_to_goal(node_state: &PointData, goal: Point) -> u16 {
    node_state.1.manhattan_distance(goal) as u16
}

#[aoc(day17, part2)]
pub fn solve_part2(map: &Map) -> SolutionType {
    let start = Point { x: 0, y: 0 };
    let start = [
        (1, start.walk(Dir::East), Dir::East),
        (1, start.walk(Dir::South), Dir::South),
    ];
    let goal = Point {
        x: map.get_width() - 1,
        y: map.get_height() - 1,
    };
    search_for_node(map, &start, goal)
}
