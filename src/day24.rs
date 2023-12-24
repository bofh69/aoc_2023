// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};
use num::*;

type Length = f64;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Point(Length, Length, Length);

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Dir(Length, Length, Length);

type InputType = (Point, Dir);
type SolutionType = usize;

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    // 1, 2, 3 @ -1, 2, -3
    input
        .lines()
        .map(|line| {
            let line: Vec<_> = line
                .split_ascii_whitespace()
                .filter_map(|s| s.trim_end_matches(",").parse().ok())
                .collect();
            (
                Point(line[0], line[1], line[2]),
                Dir(line[3], line[4], line[5]),
            )
        })
        .collect()
}

#[aoc(day24, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    let (min, max);
    if data.len() > 10 {
        min = 200_000_000_000_000f64;
        max = 400_000_000_000_000f64;
    } else {
        min = 7f64;
        max = 27f64;
    }

    let mut sum = 0;
    for i in 0..data.len() {
        for j in (i + 1)..data.len() {
            let a = data[i];
            let b = data[j];

            let p0 = a.0;
            let p1 = b.0;
            let n0 = a.1;
            let n1 = b.1;

            let dx = p1.0 - p0.0;
            let dy = p1.1 - p0.1;
            let det = n1.0 * n0.1 - n1.1 * n0.0;
            let u_det = dy * n1.0 - dx * n1.1;
            let v_det = dy * n0.0 - dx * n0.1;
            let u = u_det / det;
            let v = v_det / det;
            if u < Zero::zero() || v < Zero::zero() {
                continue;
            }

            let m0 = n0.1 / n0.0;
            let m1 = n1.1 / n1.0;
            let b0 = p0.1 - m0 * p0.0;
            let b1 = p1.1 - m1 * p1.0;
            let x = (b1 - b0) / (m0 - m1);
            let y = m0 * x + b0;

            if x >= min && x <= max && y >= min && y <= max {
                sum += 1;
            }
        }
    }
    sum
}

#[aoc(day24, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    data.len()

    // P + T * V = S_P + T * S_V
    // Lös för S_P[0..2], S_V[0..2] T[0..n]
    // Pi + Ti * Vi = S_P + Ti * S_V
    
}
