// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use super::world::*;
// use rayon::prelude::*;

type SolutionType = usize;

fn plots_after_steps(start: Point, map: &Map, steps: i32) -> usize {
    use std::collections::HashSet;
    let mut found = [HashSet::new(), HashSet::new()];
    let mut front = Vec::new();
    front.push(start);
    let mut next = Vec::new();
    for gen in 0..=steps {
        let is_even = gen % 2;
        for &pos in &front {
            if found[is_even as usize].insert(pos) {
                use Dir::*;
                for dir in [North, South, East, West] {
                    let new_pos = pos.walk(dir);
                    let c = map.get_at_unchecked(Point {
                        x: new_pos.x.rem_euclid(map.get_width()),
                        y: new_pos.y.rem_euclid(map.get_height()),
                    });
                    if c == b'.' || c == b'S' {
                        next.push(new_pos);
                    }
                }
            }
        }
        front.clear();
        front.append(&mut next);
    }
    found[steps as usize % 2]
        .iter()
        .filter(|&pos| map.is_inside_map(*pos))
        .count()
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

    plots_after_steps(start, map, 64)
}

#[aoc(day21, part2)]
pub fn solve_part2(map: &Map) -> SolutionType {
    let gen = 26501365;

    // After a lot of off by one errors while trying to derive the formula below,
    // I cheated and used the formula from here:
    // https://www.reddit.com/r/adventofcode/comments/18o4y0m/2023_day_21_part_2_algebraic_solution_using_only/
    //
    // It also had off by one errors compared to my solution, so in the end I had to adjust the steps for
    // even, odd and t.
    //
    // To validate, I simulated 327 steps on an 7 * 7 maps, recorded the number of plots on each submap
    // and compared that with the result from this formula.

    let w = map.get_width();
    assert_eq!(w, map.get_height());
    assert_eq!((gen - w / 2) % w, 0);

    let n = (gen - w / 2) / w;

    assert_eq!(n % 2, 0);

    let start = Point { x: w / 2, y: w / 2 };

    let even = plots_after_steps(start, map, 3 * w - 1);
    let odd = plots_after_steps(start, map, 3 * w);

    let steps_a = (3 * w - 3) / 2;
    let a = [
        plots_after_steps(Point { x: w - 1, y: w - 1 }, map, steps_a),
        plots_after_steps(Point { x: 0, y: w - 1 }, map, steps_a),
        plots_after_steps(Point { x: 0, y: 0 }, map, steps_a),
        plots_after_steps(Point { x: w - 1, y: 0 }, map, steps_a),
    ];
    let steps_b = (w - 3) / 2;
    let b = [
        plots_after_steps(Point { x: w - 1, y: w - 1 }, map, steps_b),
        plots_after_steps(Point { x: 0, y: w - 1 }, map, steps_b),
        plots_after_steps(Point { x: 0, y: 0 }, map, steps_b),
        plots_after_steps(Point { x: w - 1, y: 0 }, map, steps_b),
    ];

    let steps_t = w - 1;
    let t = [
        plots_after_steps(
            Point {
                x: w - 1,
                y: start.y,
            },
            map,
            steps_t,
        ),
        plots_after_steps(
            Point {
                x: start.x,
                y: w - 1,
            },
            map,
            steps_t,
        ),
        plots_after_steps(Point { x: 0, y: start.y }, map, steps_t),
        plots_after_steps(Point { x: start.x, y: 0 }, map, steps_t),
    ];

    ((n - 1) as SolutionType * (n - 1) as SolutionType) * odd
        + n as SolutionType * n as SolutionType * even
        + (n - 1) as SolutionType * (a[0] + a[1] + a[2] + a[3])
        + n as SolutionType * (b[0] + b[1] + b[2] + b[3])
        + (t[0] + t[1] + t[2] + t[3])
}
