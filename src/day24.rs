// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};
use num::*;

// use crate::world::LengthType;

type Length = f64;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Point {
    x: Length,
    y: Length,
    z: Length,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Dir {
    x: Length,
    y: Length,
    z: Length,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct InputType {
    p: Point,
    v: Dir,
}
type SolutionType = usize;

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    // 1, 2, 3 @ -1, 2, -3
    input
        .lines()
        .map(|line| {
            let line: Vec<_> = line
                .split_ascii_whitespace()
                .filter_map(|s| s.trim_end_matches(',').parse().ok())
                .collect();
            InputType {
                p: Point {
                    x: line[0],
                    y: line[1],
                    z: line[2],
                },
                v: Dir {
                    x: line[3],
                    y: line[4],
                    z: line[5],
                },
            }
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

            let p0 = a.p;
            let p1 = b.p;
            let n0 = a.v;
            let n1 = b.v;

            let dx = p1.x - p0.x;
            let dy = p1.y - p0.y;
            let det = n1.x * n0.y - n1.y * n0.x;
            let u_det = dy * n1.x - dx * n1.y;
            let v_det = dy * n0.x - dx * n0.y;
            let u = u_det / det;
            let v = v_det / det;
            if u < Zero::zero() || v < Zero::zero() {
                continue;
            }

            let m0 = n0.y / n0.x;
            let m1 = n1.y / n1.x;
            let b0 = p0.y - m0 * p0.x;
            let b1 = p1.y - m1 * p1.x;
            let x = (b1 - b0) / (m0 - m1);
            let y = m0 * x + b0;

            if x >= min && x <= max && y >= min && y <= max {
                sum += 1;
            }
        }
    }
    sum
}

fn fill_rows_for(m: &mut [f64], h1: &InputType, h2: &InputType) {
    // Px0 * (Vy2 - Vy1) + Py0 * (Vx1 - Vx2) + Vx0 * (Py1 - Py2) + Vy0 * (Px2 - Px1) + Px1 * Vy1 - Vx1 * Py1 - Px2 * Vy2 + Vx2 * Py2 = 0
    m[0] = h2.v.y - h1.v.y; // Px0
    m[1] = h1.v.x - h2.v.x; // Py0
    m[2] = 0.; // Pz0
    m[3] = h1.p.y - h2.p.y; // Vx0
    m[4] = h2.p.x - h1.p.x; // Vy0
    m[5] = 0.; // Vz0
    m[6] = -(h1.p.x * h1.v.y - h1.v.x * h1.p.y - h2.p.x * h2.v.y + h2.v.x * h2.p.y); // result

    // Py0 * (Vz2 - Vz1) + Pz0 * (Vy1 - Vy2) + Vy0 * (Pz1 - Pz2) + Vz0 * (Py2 - Py1) + Py1 * Vz1 - Vy1 * Pz1 - Py2 * Vz2 + Vy2 * Pz2 = 0
    m[7] = 0.; // Px0
    m[7 + 1] = h2.v.z - h1.v.z; // Py0
    m[7 + 2] = h1.v.y - h2.v.y; // Pz0
    m[7 + 3] = 0.; // Vx0
    m[7 + 4] = h1.p.z - h2.p.z; // Vy0
    m[7 + 5] = h2.p.y - h1.p.y; // Vz0
    m[7 + 6] = -(h1.p.y * h1.v.z - h1.v.y * h1.p.z - h2.p.y * h2.v.z + h2.v.y * h2.p.z); // Result

    // Pz0 * (Vx2 - Vx1) + Px0 * (Vz1 - Vz2) + Vz0 * (Px1 - Px2) + Vx0 * (Pz2 - Pz1) + Pz1 * Vx1 - Vz1 * Px1 - Pz2 * Vx2 + Vz2 * Px2 = 0
    m[14] = h1.v.z - h2.v.z; // Px0
    m[14 + 1] = 0.; // Py0
    m[14 + 2] = h2.v.x - h1.v.x; // Pz0
    m[14 + 3] = h2.p.z - h1.p.z; // Vx0
    m[14 + 4] = 0.; // Vy0
    m[14 + 5] = h1.p.x - h2.p.x; // Vz0
    m[14 + 6] = -(h1.p.z * h1.v.x - h1.v.z * h1.p.x - h2.p.z * h2.v.x + h2.v.z * h2.p.x);
    // Result
}

fn find_max_row_in_column(matrix: &[Length], column: usize, row: usize) -> usize {
    let mut max = matrix[row * 7 + column].abs();
    let mut max_row = row;
    for r in row..6 {
        let v = matrix[r * 7 + column].abs();
        if v > max {
            max = v;
            max_row = r;
        }
    }
    max_row
}

fn print_matrix(matrix: &[Length]) {
    for row in 0..6 {
        println!(
            "    {:4} {:4} {:4} {:4} {:4} {:4} {:4}",
            matrix[row * 7],
            matrix[row * 7 + 1],
            matrix[row * 7 + 2],
            matrix[row * 7 + 3],
            matrix[row * 7 + 4],
            matrix[row * 7 + 5],
            matrix[row * 7 + 6]
        );
    }
}

fn swap_rows(matrix: &mut [Length], row1: usize, row2: usize) {
    for column in 0..7 {
        matrix.swap(row1 * 7 + column, row2 * 7 + column);
    }
}

#[aoc(day24, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    // P + T * V = S_P + T * S_V
    // Lös för S_P[0..2], S_V[0..2] T[0..n]
    // Pi + Ti * Vi = S_P + Ti * S_V
    /*
    * Px0 + Vx0 * t1 = Px1 + Vx1 * t1
    * Py0 + Vy0 * t1 = Py1 + Vy1 * t1
    * Pz0 + Vz0 * t1 = Pz1 + Vz1 * t1
    *
    * ---
    *
    * (Px0 - Px1) + (Vx0 - Vx1) * t1 = 0
    * (Py0 - Py1) + (Vy0 - Vy1) * t1 = 0
    * (Pz0 - Pz1) + (Vz0 - Vz1) * t1 = 0
    *
    * ---
    *
    * Lös andra för t1, sätt in i första:
    * (Px0 - Px1) + (Vx0 - Vx1) * t1 = 0
    * (Py1 - Py0) / (Vy0 - Vy1) = t1
    *
    * (Px0 - Px1) + (Vx0 - Vx1) * (Py1 - Py0) / (Vy0 - Vy1) = 0
    *
    * Multiplicera bort divisionen:
    * (Px0 - Px1) * (Vy0 - Vy1) + (Vx0 - Vx1) * (Py1 - Py0) = 0
    * Övriga:
    * (Py0 - Py1) * (Vz0 - Vz1) + (Vy0 - Vy1) * (Pz1 - Pz0) = 0
    * (Pz0 - Pz1) * (Vx0 - Vx1) + (Vz0 - Vz1) * (Px1 - Px0) = 0
    *
    * ----
    *
    * Expandera:
      Px0 * Vy0 - Px0 * Vy1 - Px1 * Vy0 + Px1 * Vy1 + Vx0 * Py1 - Vx0 * Py0 - Vx1 * Py1 + Vx1 * Py0 = 0

      Övriga:
      Py0 * Vz0 - Py0 * Vz1 - Py1 * Vz0 + Py1 * Vz1 + Vy0 * Pz1 - Vy0 * Pz0 - Vy1 * Pz1 + Vy1 * Pz0 = 0
      Pz0 * Vx0 - Pz0 * Vx1 - Pz1 * Vx0 + Pz1 * Vx1 + Vz0 * Px1 - Vz0 * Px0 - Vz1 * Px1 + Vz1 * Px0 = 0

      ----

      Gör samma för hagel 2:
      Px0 * Vy0 - Px0 * Vy2 - Px2 * Vy0 + Px2 * Vy2 + Vx0 * Py2 - Vx0 * Py0 - Vx2 * Py2 + Vx2 * Py0 = 0
      Py0 * Vz0 - Py0 * Vz2 - Py2 * Vz0 + Py2 * Vz2 + Vy0 * Pz2 - Vy0 * Pz0 - Vy2 * Pz2 + Vy2 * Pz0 = 0
      Pz0 * Vx0 - Pz0 * Vx2 - Pz2 * Vx0 + Pz2 * Vx2 + Vz0 * Px2 - Vz0 * Px0 - Vz2 * Px2 + Vz2 * Px0 = 0

      --- Subtrahera ekvationerna för 2 i 1:
      - Px0 * Vy1 - Px1 * Vy0 + Px1 * Vy1 + Vx0 * Py1 - Vx1 * Py1 + Vx1 * Py0 + Px0 * Vy2 + Px2 * Vy0 - Px2 * Vy2 - Vx0 * Py2 + Vx2 * Py2 - Vx2 * Py0 = 0

      Förenkla till:
      Px0 * (Vy2 - Vy1) + Py0 * (Vx1 - Vx2) + Vx0 * (Py1 - Py2) + Vy0 * (Px2 - Px1) + Px1 * Vy1 - Vx1 * Py1 - Px2 * Vy2 + Vx2 * Py2 = 0
      Py0 * (Vz2 - Vz1) + Pz0 * (Vy1 - Vy2) + Vy0 * (Pz1 - Pz2) + Vz0 * (Py2 - Py1) + Py1 * Vz1 - Vy1 * Pz1 - Py2 * Vz2 + Vy2 * Pz2 = 0
      Pz0 * (Vx2 - Vx1) + Px0 * (Vz1 - Vz2) + Vz0 * (Px1 - Px2) + Vx0 * (Pz2 - Pz1) + Pz1 * Vx1 - Vz1 * Px1 - Pz2 * Vx2 + Vz2 * Px2 = 0

    */

    let mut matrix = [0.; 6 * 7];
    fill_rows_for(&mut matrix[0..3 * 7], &data[0], &data[1]);
    fill_rows_for(&mut matrix[3 * 7..], &data[2], &data[3]);

    for column in 0..6 - 1 {
        println!("Column {}", column);
        let max_r = find_max_row_in_column(&matrix, column, column);
        if max_r != column {
            println!("Swapping {} and {}", column, max_r);
            swap_rows(&mut matrix, column, max_r);
            print_matrix(&matrix);
        }
        let vi = dbg!(matrix[column * 7 + column]);
        assert!(vi.abs() > 1e-20);
        for row in column + 1..6 {
            let v = matrix[row * 7 + column];
            if v.abs() > 1e-20 {
                let mult = v / vi;
                matrix[row * 7 + column] = 0.;
                for c2 in column + 1..7 {
                    let v = matrix[column * 7 + c2];
                    matrix[row * 7 + c2] -= mult * v;
                }
            }
        }
    }
    println!("Done");
    print_matrix(&matrix);

    let mut res = [0.; 6];
    for i in 0..6 {
        let i = 5 - i;
        let mut v = matrix[i * 7 + 6];
        for j in i..6 {
            v -= res[j] * matrix[i * 7 + j];
        }
        res[i] = (v / matrix[i * 7 + i]).round();
    }

    println!("{:?}", res);

    (res[0] + res[1] + res[2]).round() as SolutionType
}
