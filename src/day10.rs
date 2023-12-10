// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use super::world::*;
use std::collections::HashSet;

type SolutionType = i32;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Map {
    Map::from_string(input)
}

fn is_up(c: u8) -> bool {
    matches!(c, b'S' | b'|' | b'L' | b'J')
}

fn is_down(c: u8) -> bool {
    matches!(c, b'S' | b'|' | b'F' | b'7')
}

fn is_left(c: u8) -> bool {
    matches!(c, b'S' | b'-' | b'J' | b'7')
}

fn is_right(c: u8) -> bool {
    matches!(c, b'S' | b'-' | b'F' | b'L')
}

fn find_exits(map: &Map, from: Point) -> [Point; 2] {
    let mut result = [from; 2];
    let mut idx = 0;
    let from_c = map.get_at(from);
    for (pos, dir, c) in map.neighbors(from) {
        if match (dir, c) {
            (Dir::North, b'|') => is_up(from_c),
            (Dir::North, b'F') => is_up(from_c),
            (Dir::North, b'7') => is_up(from_c),
            (Dir::North, b'S') => is_up(from_c),
            (Dir::South, b'|') => is_down(from_c),
            (Dir::South, b'L') => is_down(from_c),
            (Dir::South, b'J') => is_down(from_c),
            (Dir::South, b'S') => is_down(from_c),
            (Dir::West, b'-') => is_left(from_c),
            (Dir::West, b'L') => is_left(from_c),
            (Dir::West, b'F') => is_left(from_c),
            (Dir::West, b'S') => is_left(from_c),
            (Dir::East, b'-') => is_right(from_c),
            (Dir::East, b'J') => is_right(from_c),
            (Dir::East, b'7') => is_right(from_c),
            (Dir::East, b'S') => is_right(from_c),
            _ => false,
        } {
            result[idx] = pos;
            idx += 1;
        }
    }
    for pos in result {
        if pos == from {
            map.print();
            map.print_with_overlay(|pos, _| {
                if pos == result[0] {
                    Some(b'A')
                } else if pos == result[1] {
                    Some(b'B')
                } else {
                    None
                }
            });
            panic!("Couldn't move from {:?}; {:?}", from, result);
        }
    }
    result
}

#[aoc(day10, part1)]
pub fn solve_part1(map: &Map) -> SolutionType {
    let mut start = Point { x: 0, y: 0 };
    //TODO: map.find
    for (pos, c) in map.iter() {
        if c == b'S' {
            start = pos;
            break;
        }
    }
    let mut from = [start; 2];
    let mut curr = find_exits(map, start);
    let mut distance = 1;
    'main: loop {
        for i in 0..2 {
            let exits = find_exits(map, curr[i]);
            if exits[0] == from[i] {
                from[i] = curr[i];
                curr[i] = exits[1];
            } else {
                from[i] = curr[i];
                curr[i] = exits[0];
            }
            if i == 1 {
                distance += 1;
            }
            if curr[0] == curr[1] {
                break 'main;
            }
        }
    }
    distance
}

#[aoc(day10, part2)]
pub fn solve_part2(map: &Map) -> SolutionType {
    let mut start = Point { x: 0, y: 0 };
    //TODO: map.find
    for (pos, c) in map.iter() {
        if c == b'S' {
            start = pos;
            break;
        }
    }
    let mut from = start;
    let mut curr = find_exits(map, start)[0];
    let mut loop_segs = HashSet::new();
    loop_segs.insert(start);
    loop {
        loop_segs.insert(curr);
        let exits = find_exits(map, curr);
        if exits[0] == from {
            from = curr;
            curr = exits[1];
        } else {
            from = curr;
            curr = exits[0];
        }
        if curr == start {
            break;
        }
    }

    // Tripple the size of the map:
    let map_width = map.get_width() * 3;
    let map_height = map.get_height() * 3;

    let mut map2 = Map::new(map_width, map_height);
    for (pos, c) in map.iter() {
        let s = 3;
        match c {
            b'O' => {
                for i in 0..s {
                    map2.set_at(
                        Point {
                            x: pos.x * s + i,
                            y: pos.y * s,
                        },
                        c,
                    );
                    map2.set_at(
                        Point {
                            x: pos.x * s + i,
                            y: pos.y * s + 1,
                        },
                        c,
                    );
                    map2.set_at(
                        Point {
                            x: pos.x * s + i,
                            y: pos.y * s + 2,
                        },
                        c,
                    );
                }
            }
            b'|' => {
                map2.set_at(
                    Point {
                        x: pos.x * s + 1,
                        y: pos.y * s,
                    },
                    c,
                );
                map2.set_at(
                    Point {
                        x: pos.x * s + 1,
                        y: pos.y * s + 1,
                    },
                    c,
                );
                map2.set_at(
                    Point {
                        x: pos.x * s + 1,
                        y: pos.y * s + 2,
                    },
                    c,
                );
            }
            b'-' => {
                map2.set_at(
                    Point {
                        x: pos.x * s,
                        y: pos.y * s + 1,
                    },
                    c,
                );
                map2.set_at(
                    Point {
                        x: pos.x * s + 1,
                        y: pos.y * s + 1,
                    },
                    c,
                );
                map2.set_at(
                    Point {
                        x: pos.x * s + 2,
                        y: pos.y * s + 1,
                    },
                    c,
                );
            }
            b'S' => {
                map2.set_at(
                    Point {
                        x: pos.x * s + 1,
                        y: pos.y * s + 1,
                    },
                    c,
                );
                map2.set_at(
                    Point {
                        x: pos.x * s + 1,
                        y: pos.y * s,
                    },
                    b'|',
                );
                map2.set_at(
                    Point {
                        x: pos.x * s + 1,
                        y: pos.y * s + 2,
                    },
                    b'|',
                );
                map2.set_at(
                    Point {
                        x: pos.x * s,
                        y: pos.y * s + 1,
                    },
                    b'-',
                );
                map2.set_at(
                    Point {
                        x: pos.x * s + 2,
                        y: pos.y * s + 1,
                    },
                    b'-',
                );
            }
            b'F' => {
                map2.set_at(
                    Point {
                        x: pos.x * s + 1,
                        y: pos.y * s + 1,
                    },
                    c,
                );
                map2.set_at(
                    Point {
                        x: pos.x * s + 1,
                        y: pos.y * s + 2,
                    },
                    b'|',
                );
                map2.set_at(
                    Point {
                        x: pos.x * s + 2,
                        y: pos.y * s + 1,
                    },
                    b'-',
                );
            }
            b'J' => {
                map2.set_at(
                    Point {
                        x: pos.x * s + 1,
                        y: pos.y * s + 1,
                    },
                    c,
                );
                map2.set_at(
                    Point {
                        x: pos.x * s + 1,
                        y: pos.y * s,
                    },
                    b'|',
                );
                map2.set_at(
                    Point {
                        x: pos.x * s,
                        y: pos.y * s + 1,
                    },
                    b'-',
                );
            }
            b'7' => {
                map2.set_at(
                    Point {
                        x: pos.x * s + 1,
                        y: pos.y * s + 1,
                    },
                    c,
                );
                map2.set_at(
                    Point {
                        x: pos.x * s + 1,
                        y: pos.y * s + 2,
                    },
                    b'|',
                );
                map2.set_at(
                    Point {
                        x: pos.x * s,
                        y: pos.y * s + 1,
                    },
                    b'-',
                );
            }
            b'L' => {
                map2.set_at(
                    Point {
                        x: pos.x * s + 1,
                        y: pos.y * s + 1,
                    },
                    c,
                );
                map2.set_at(
                    Point {
                        x: pos.x * s + 1,
                        y: pos.y * s,
                    },
                    b'|',
                );
                map2.set_at(
                    Point {
                        x: pos.x * s + 2,
                        y: pos.y * s + 1,
                    },
                    b'-',
                );
            }
            b'.' => {}
            _ => panic!("Unkown char {}", c as char),
        }
    }

    // Mark the real path here, could just as have been done in the first transform
    map2.transform(|_map, pos, c| {
        let pos2 = Point {
            x: pos.x / 3,
            y: pos.y / 3,
        };
        if c != b'.' && loop_segs.contains(&pos2) {
            b'*'
        } else {
            b'.'
        }
    });

    // This assumes (0, 0) is connected everywhere outside,
    // which was the case with my input.
    map2.flood_cardinal_with(Point { x: 0, y: 0 }, &mut |_p, c| {
        if c != b'O' && c != b'*' {
            Some(b'O')
        } else {
            None
        }
    });

    // map2.print();

    i32::try_from(
        map.iter()
            .filter(|(pos, _c)| {
                let pos2 = Point {
                    x: pos.x * 3,
                    y: pos.y * 3,
                };
                let c = map2.get_at(pos2);
                c != b'O' && !loop_segs.contains(pos)
            })
            .count(),
    )
    .expect("count within range")
}
