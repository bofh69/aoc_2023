// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use super::world::*;
use std::collections::HashMap;
// use rayon::prelude::*;

type SolutionType = usize;

fn bfs(map: &Map, from: Point, max_steps: u32) -> Vec<(Point, u32)> {
    let mut expanded = HashMap::new();
    let mut to_expand = Vec::new();
    to_expand.push((0, from));
    while let Some((steps, pos)) = to_expand.pop() {
        let is_even = steps % 2 == 0;
        if let Some(prev_steps) = expanded.get(&(pos, is_even)) {
            if *prev_steps <= steps {
                continue;
            }
        }
        expanded.insert((pos, is_even), steps);
        if steps >= max_steps {
            continue;
        }
        let steps = steps + 1;
        for (pos, dir, c) in map.neighbors(pos) {
            use Dir::*;
            if matches!(dir, North | South | East | West) && (c == b'.' || c == b'S') {
                to_expand.push((steps, pos));
            }
        }
    }
    expanded
        .iter()
        .filter_map(
            |((pos, even), v)| {
                if *even {
                    Some((*pos, *v))
                } else {
                    None
                }
            },
        )
        .collect()
}

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Map {
    Map::from_string(input)
}

fn _print_map(map: &Map, found: &Vec<(Point, u32)>) {
    map.print_with_overlay(|pos, c| {
        for (pos2, _steps) in found {
            if pos == *pos2 && c != b'S' {
                return Some(b'O');
            }
        }
        None
    });
}

#[aoc(day21, part1)]
pub fn solve_part1(map: &Map) -> SolutionType {
    let start = map.find(b'S')[0];
    let found = bfs(map, start, 64);

    found.len()
}

/*
fn bfs2(map: &Map, from: Point) -> ([(u16, Point); 4], HashMap<u16, usize>) {
    let mut expanded = HashMap::new();
    let mut to_expand = std::collections::BTreeSet::new();
    to_expand.insert((0, from));
    let mut north = None;
    let mut south = None;
    let mut east = None;
    let mut west = None;
    let mut found_per_steps = HashMap::new();

    while let Some((steps, pos)) = to_expand.pop_first() {
        if north.is_some() && south.is_some() && east.is_some() && west.is_some() {
            break;
        }
        if let Some(prev_steps) = expanded.get(&pos) {
            if *prev_steps <= steps {
                continue;
            }
        }
        expanded.insert(pos, steps);

        found_per_steps.insert(steps, expanded.keys().len());

        if pos.y == 0 {
            if let Some((prev, _pos)) = north {
                if prev > steps {
                    north = Some((steps, pos));
                }
            } else {
                north = Some((steps, pos));
            }
        }

        if pos.y == map.get_height() - 1 {
            if let Some((prev, _pos)) = south {
                if prev > steps {
                    south = Some((steps, pos));
                }
            } else {
                south = Some((steps, pos));
            }
        }

        if pos.x == 0 {
            if let Some((prev, _pos)) = west {
                if prev > steps {
                    west = Some((steps, pos));
                }
            } else {
                west = Some((steps, pos));
            }
        }

        if pos.x == map.get_width() - 1 {
            if let Some((prev, _pos)) = east {
                if prev > steps {
                    east = Some((steps, pos));
                }
            } else {
                east = Some((steps, pos));
            }
        }

        let steps = steps + 1;
        for (pos, dir, c) in map.neighbors(pos) {
            use Dir::*;
            if matches!(dir, North | South | East | West) && (c == b'.' || c == b'S') {
                to_expand.insert((steps, pos));
            }
        }
    }

    /*
    map.print_with_overlay(|pos, c| {
        for (pos2, _steps) in &expanded {
            if pos == *pos2 && c != b'S' {
                return Some(b'O');
            }
        }
        None
    });
    */

    ([north.unwrap(), south.unwrap(), east.unwrap(), west.unwrap()], found_per_steps)
}
*/

fn next_gen(map: &mut Map) {
    map.transform(|map, pos, c| {
        if c == b'.' {
            if Some(b'O') == map.get_at(pos.walk(Dir::North)) {
                return b'O';
            }
            if Some(b'O') == map.get_at(pos.walk(Dir::South)) {
                return b'O';
            }
            if Some(b'O') == map.get_at(pos.walk(Dir::East)) {
                return b'O';
            }
            if Some(b'O') == map.get_at(pos.walk(Dir::West)) {
                return b'O';
            }
        } else if c == b'O' {
            return b'.';
        }
        c
    });
}

#[aoc(day21, part2)]
pub fn solve_part2(map: &Map) -> SolutionType {
    const NEW_SIZE: i32 = 9;
    let mut map2 = Map::new(NEW_SIZE * map.get_width(), NEW_SIZE * map.get_height());
    for y in 0..map2.get_height() {
        for x in 0..map2.get_width() {
            let c = map.get_at_unchecked(Point {
                x: x % map.get_width(),
                y: y % map.get_height(),
            });
            map2.set_at(Point { x, y }, c);
        }
    }
    let start = map.find(b'S')[0];
    map2.set_at(
        Point {
            x: (NEW_SIZE / 2) * map.get_width() + start.x,
            y: (NEW_SIZE / 2) * map.get_height() + start.y,
        },
        b'O',
    );
    map2.transform(|_, _, c| if c == b'S' { b'.' } else { c });

    println!("{} {}", map2.get_width(), map2.get_height());

    let mut counts: Vec<Vec<usize>> = Vec::new();
    for _i in 0..=(map.get_width() * 4) {
        let mut row = vec![];
        for ly in 0..NEW_SIZE {
            for lx in 0..NEW_SIZE {
                let mut count = 0;
                for y in 0..map.get_height() {
                    for x in 0..map.get_width() {
                        if map2.get_at_unchecked(Point {
                            x: lx * map.get_width() + x,
                            y: ly * map.get_height() + y,
                        }) == b'O'
                        {
                            count += 1;
                        }
                    }
                }
                row.push(count);
            }
        }
        /*
        if i > 1 {
            print!("{:4} ", i);
            for (j, &v) in row.iter().enumerate() {
                if v > 0 && counts[i - 2][j] == v {
                    print!(" *{:4}", v);
                } else {
                    print!("  {:4}", v);
                }
            }
            println!();
        }
        */
        counts.push(row);
        // map2.print();
        next_gen(&mut map2);
    }

    /*
    // let mut filled_maps = 0;
    for (gen, row) in counts.iter().enumerate() {
        /*
        let new_filled_maps = row.iter().filter(|&r| *r != 0).count();
        if gen != counts.len() - 1 &&
           new_filled_maps == filled_maps {
            continue;
        }
        filled_maps = new_filled_maps;
        */
        let gen = gen as i32;
        if gen % map.get_width() != map.get_width() / 2 {
            continue;
        }
        println!("Gen {}", gen);
        for y in 0..NEW_SIZE {
            for x in 0..NEW_SIZE {

                print!("{:4} ", row[(y * NEW_SIZE + x) as usize]);
            }
            println!();
        }
        println!();
    }
    */

    /*
     *     U
     *    ESF
     *   ESISF
     *  EISISIF
     * LISISISIR
     *  GISISIH
     *   GISIH
     *    GSH
     *     D
     */

    /*
    let target_gen = 458;

    let reminder = target_gen % map.get_width();
    let width = 1 + target_gen / map.get_height();
    let height = 1 + target_gen / map.get_height();

    let middle = NEW_SIZE * (1 + NEW_SIZE) / 2 + NEW_SIZE / 2;
    println!("Reminder {}", reminder);
    println!("Width {}", width);
    println!("Height {}", height);
    println!("Middle {}", middle);

    for y in -height ..= height {
        for x in -width ..= width {
            let l = y.abs() + x.abs();
            if l > (target_gen + map.get_width() - 1)/map.get_width() {
                print!("    0");
            } else if l < target_gen/map.get_width() {
                let even = l % 2;
                print!(" {:4}", counts[ 3 * map.get_width() as usize + reminder as usize][(even + middle) as usize]);
            } else {
                print!(" XXXX");
            }
        }
        println!();
    }
     */

    // let mut offset = 0;

    /*
    let mut count = 0;
    let STEPS: usize = 10;
    for y in 0..
    */

    // There is an empty border around the map

    /*
    let mut map = map.clone();

    for x in 0..map.get_width() {
        map.set_at(
            Point { x, y: 0 },
            map.get_at_unchecked(Point {
                x,
                y: map.get_height() - 2,
            }),
        );
        map.set_at(
            Point {
                x,
                y: map.get_height() - 1,
            },
            map.get_at_unchecked(Point { x, y: 1 }),
        );
    }
    for y in 0..map.get_width() {
        map.set_at(
            Point { x: 0, y },
            map.get_at_unchecked(Point {
                x: map.get_width() - 2,
                y,
            }),
        );
        map.set_at(
            Point {
                x: map.get_width() - 1,
                y,
            },
            map.get_at_unchecked(Point { x: 1, y }),
        );
    }

    map.print();
    */

    /*
    let found_edge = bfs2(&map, start);

    println!("{:?}", found_edge.0);

    let mut found_per_step : Vec<_> = found_edge.1.iter().collect();

    found_per_step.sort();

    println!("{:?}", found_per_step);

    let found_edge = found_edge.0;

    let found_north = bfs2(
        &map,
        Point {
            x: found_edge[0].1.x,
            y: map.get_height() - 1,
        },
    ).0[0];
    let found_south = bfs2(
        &map,
        Point {
            x: found_edge[1].1.x,
            y: 0,
        },
    ).0[1];
    let found_east = bfs2(
        &map,
        Point {
            x: 0,
            y: found_edge[2].1.y,
        },
    ).0[2];
    let found_west = bfs2(
        &map,
        Point {
            x: map.get_width() - 1,
            y: found_edge[3].1.y,
        },
    ).0[3];

    println!("{:?}", (found_north, found_south, found_east, found_west));
     */

    // TODO: Add border
    // Count time to next tile.

    // Finds edge at 65
    // TODO

    counts.len()

    /*
    use std::collections::HashSet;
    let start = map.find(b'S')[0];
    let mut found = [HashSet::new(), HashSet::new()];
    let mut front = Vec::new();
    front.push(start);
    let mut next = Vec::new();
    for gen in 0..=5000 {
        let is_even = gen % 2;
        for &pos in &front {
            if found[is_even].insert(pos) {
                use Dir::*;
                for dir in [North, South, East, West] {
                    let new_pos = pos.walk(dir);
                    let c = map.get_at_unchecked(Point {
                        x: new_pos.x.rem_euclid(map.get_width()),
                        y: new_pos.y.rem_euclid(map.get_height()),
                    });
                    if c == b'.' || c == b'S' {
                        // println!("Inserting {:?}", new_pos);
                        next.push(new_pos);
                    }
                }
            }
        }
        // println!("{:4} {}", gen, found[is_even].len());
        front.clear();
        front.append(&mut next);
    }
    found[0].len()
    */
}
