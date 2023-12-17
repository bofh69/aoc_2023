// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use super::world::*;
// use std::collections::HashSet;
use std::collections::HashMap;
// use rayon::prelude::*;

type SolutionType = usize;

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Map {
    Map::from_string(input)
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct CostAndPoint(SolutionType, u8, Point, Dir, HashMap<Point, Dir>);

impl Ord for CostAndPoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for CostAndPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.0.cmp(&self.0))
    }
}

fn bfs(map: &Map, from: Point, to: Point) -> (SolutionType, HashMap<Point, Dir>) {
    let mut expanded = std::collections::HashMap::new();
    let mut to_expand = std::collections::BinaryHeap::new();
    to_expand.push(CostAndPoint(
        0,
        1,
        from.walk(Dir::East),
        Dir::East,
        HashMap::new(),
    ));
    to_expand.push(CostAndPoint(
        0,
        1,
        from.walk(Dir::South),
        Dir::South,
        HashMap::new(),
    ));
    while let Some(CostAndPoint(mut heat_loss, steps, pos, dir, path)) = to_expand.pop() {
        if let Some(&cost) = expanded.get(&(steps, pos, dir)) {
            if cost <= heat_loss {
                continue;
            }
        }
        expanded.insert((steps, pos, dir), heat_loss);
        // println!("Walking from {:?} {:?} ({} steps)", pos, dir, steps);
        heat_loss += SolutionType::from(map.get_at_unchecked(pos) - b'0');
        if to == pos {
            return (heat_loss, path);
        }
        let mut path = path.clone();
        path.insert(pos, dir);
        let dir_left = dir.turn_left().turn_left();
        let pos_left = pos.walk(dir_left);
        if map.is_inside_map(pos_left) {
            let path = path.clone();
            to_expand.push(CostAndPoint(heat_loss, 1, pos_left, dir_left, path));
        }
        let dir_right = dir.turn_right().turn_right();
        let pos_right = pos.walk(dir_right);
        if map.is_inside_map(pos_right) {
            let path = path.clone();
            to_expand.push(CostAndPoint(heat_loss, 1, pos_right, dir_right, path));
        }
        if steps < 3 {
            let pos = pos.walk(dir);
            if map.is_inside_map(pos) {
                let path = path.clone();
                to_expand.push(CostAndPoint(heat_loss, steps + 1, pos, dir, path));
            }
        }
    }
    (0, HashMap::new())
}

#[aoc(day17, part1)]
pub fn solve_part1(map: &Map) -> SolutionType {
    let goal = Point {
        x: map.get_width() - 1,
        y: map.get_height() - 1,
    };
    let (c, _path) = bfs(map, Point { x: 0, y: 0 }, goal);
    /*
    map.print_with_overlay(|pos, _c| {
        path.get(&pos).map(|dir| match dir {
            Dir::North => b'^',
            Dir::South => b'v',
            Dir::East => b'>',
            Dir::West => b'<',
            _ => unreachable!(),
        })
    });
    */
    c
}

fn bfs2(map: &Map, from: Point, to: Point) -> (SolutionType, HashMap<Point, Dir>) {
    let mut expanded = std::collections::HashMap::new();
    let mut to_expand = std::collections::BinaryHeap::new();
    to_expand.push(CostAndPoint(
        0,
        1,
        from.walk(Dir::East),
        Dir::East,
        HashMap::new(),
    ));
    to_expand.push(CostAndPoint(
        0,
        1,
        from.walk(Dir::South),
        Dir::South,
        HashMap::new(),
    ));
    while let Some(CostAndPoint(mut heat_loss, steps, pos, dir, path)) = to_expand.pop() {
        if let Some(&cost) = expanded.get(&(steps, pos, dir)) {
            if cost <= heat_loss {
                continue;
            }
        }
        expanded.insert((steps, pos, dir), heat_loss);
        // println!("Walking from {:?} {:?} ({} steps)", pos, dir, steps);
        heat_loss += SolutionType::from(map.get_at_unchecked(pos) - b'0');
        if to == pos {
            return (heat_loss, path);
        }
        let mut path = path.clone();
        path.insert(pos, dir);
        if steps >= 4 {
            let dir_left = dir.turn_left().turn_left();
            let pos_left = pos.walk(dir_left);
            if map.is_inside_map(pos_left) {
                let path = path.clone();
                to_expand.push(CostAndPoint(heat_loss, 1, pos_left, dir_left, path));
            }
            let dir_right = dir.turn_right().turn_right();
            let pos_right = pos.walk(dir_right);
            if map.is_inside_map(pos_right) {
                let path = path.clone();
                to_expand.push(CostAndPoint(heat_loss, 1, pos_right, dir_right, path));
            }
        }
        if steps < 10 {
            let pos = pos.walk(dir);
            if map.is_inside_map(pos) {
                let path = path.clone();
                to_expand.push(CostAndPoint(heat_loss, steps + 1, pos, dir, path));
            }
        }
    }
    (0, HashMap::new())
}

#[aoc(day17, part2)]
pub fn solve_part2(map: &Map) -> SolutionType {
    let goal = Point {
        x: map.get_width() - 1,
        y: map.get_height() - 1,
    };
    let (c, _path) = bfs2(map, Point { x: 0, y: 0 }, goal);
    /*
    map.print_with_overlay(|pos, _c| {
        path.get(&pos).map(|dir| match dir {
            Dir::North => b'^',
            Dir::South => b'v',
            Dir::East => b'>',
            Dir::West => b'<',
            _ => unreachable!(),
        })
    });
    */
    c
}
