// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use super::world::*;
use std::collections::HashSet;
use rayon::prelude::*;

type SolutionType = usize;

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Map {
    Map::from_string(input)
}

fn add_light(moving_lights: &mut HashSet<(Point, Dir)>, to_expand: &mut Vec<(Point, Dir)>, map: &Map, pos: Point, dir: Dir) {
        if map.is_inside_map(pos) && !moving_lights.contains(&(pos, dir)) {
            moving_lights.insert((pos, dir));
            to_expand.push((pos, dir));
        }
}

fn calculate_energize(map: &Map, start: Point, dir: Dir) -> SolutionType {
    use Dir::*;

    let mut moving_lights = HashSet::new();
    let mut to_expand = vec![];
    let mut energized = HashSet::new();

    add_light(&mut moving_lights, &mut to_expand, map, start, dir);

    while let Some((pos, dir)) = to_expand.pop() {
        energized.insert(pos);
        let c = map.get_at_unchecked(pos);
        match (c, dir) {
            (b'-', East | West) | (b'|', North | South) | (b'.', _) => {
                let pos = pos.walk(dir);
                add_light(&mut moving_lights, &mut to_expand, map, pos, dir);
            }
            (b'/', East | West) | (b'\\', North | South) => {
                let dir = dir.turn_left().turn_left();
                let pos = pos.walk(dir);
                add_light(&mut moving_lights, &mut to_expand, map, pos, dir);
            }
            (b'\\', East | West) | (b'/', North | South) => {
                let dir = dir.turn_right().turn_right();
                let pos = pos.walk(dir);
                add_light(&mut moving_lights, &mut to_expand, map, pos, dir);
            }
            (b'-', North | South) | (b'|', East | West) => {
                let dir1 = dir.turn_left().turn_left();
                let dir2 = dir.turn_right().turn_right();
                let pos1 = pos.walk(dir1);
                let pos2 = pos.walk(dir2);
                add_light(&mut moving_lights, &mut to_expand, map, pos1, dir1);
                add_light(&mut moving_lights, &mut to_expand, map, pos2, dir2);
            }
            x => unreachable!("{:?}", x),
        }
    }
    energized.len()
}

#[aoc(day16, part1)]
pub fn solve_part1(map: &Map) -> SolutionType {
    calculate_energize(map, Point { x: 0, y: 0 }, Dir::East)
}

#[aoc(day16, part2)]
pub fn solve_part2(map: &Map) -> SolutionType {
    let e1 = (0..map.get_width())
        .into_par_iter()
        .map(|x| {
            let e1 = calculate_energize(map, Point { x, y: 0 }, Dir::South);
            let e2 = calculate_energize(
                map,
                Point {
                    x,
                    y: map.get_height() - 1,
                },
                Dir::North,
            );
            e1.max(e2)
        })
        .max()
        .expect("Number");
    let e2 = (0..map.get_height())
        .into_par_iter()
        .map(|y| {
            let e1 = calculate_energize(map, Point { x: 0, y }, Dir::East);
            let e2 = calculate_energize(
                map,
                Point {
                    x: map.get_width() - 1,
                    y,
                },
                Dir::West,
            );
            e1.max(e2)
        })
        .max()
        .expect("Number");
    e1.max(e2)
}
