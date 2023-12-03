// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

type InputType = Vec<char>;
type SolutionType = usize;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    // 467..114..
    // ...*......
    input.lines().map(|line| line.chars().collect()).collect()
}

fn is_symbol(data: &[InputType], x: isize, y: isize) -> bool {
    if x < 0 {
        return false;
    }
    if y < 0 {
        return false;
    }
    if y >= data.len() as isize {
        return false;
    }
    if x >= data[0].len() as isize {
        return false;
    }
    let c = data[y as usize][x as usize];
    return c != '.' && !c.is_digit(10);
}

#[aoc(day3, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    let mut sum = 0;
    for (y, row) in data.iter().enumerate() {
        let mut num = 0;
        let mut any_symbol = false;
        for (x, c) in row.iter().enumerate() {
            if !c.is_digit(10) {
                if any_symbol {
                    sum += num;
                }
                any_symbol = false;
                num = 0;
            } else {
                num = num * 10 + c.to_digit(10).expect("Number");
                if !any_symbol {
                    for (dx, dy) in [
                        (-1, -1),
                        (-1, 0),
                        (-1, 1),
                        (0, 1),
                        (1, 1),
                        (1, 0),
                        (1, -1),
                        (0, -1),
                    ] {
                        any_symbol = is_symbol(data, x as isize + dx, y as isize + dy);
                        if any_symbol {
                            break;
                        }
                    }
                }
            }
        }
        if any_symbol {
            sum += num;
        }
    }
    sum as usize
}

fn add_number(numbers: &mut Vec<SolutionType>, data: &[InputType], x: isize, y: isize) {
    if x < 0 || x >= data[0].len() as isize {
        return;
    }
    let row = &data[y as usize];
    let mut x = x as usize;
    if !row[x].is_digit(10) {
        return;
    }
    // println!("Number at x={}, y={}, row={:?}", x, y, row);
    while x > 0 && row[x - 1].is_digit(10) {
        x -= 1;
    }
    let mut sum = 0;
    while x < row.len() && row[x].is_digit(10) {
        sum = sum * 10 + row[x].to_digit(10).expect("digit");
        x += 1;
    }
    numbers.push(sum as SolutionType);
}

#[aoc(day3, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    let mut sum = 0;
    for (y, row) in data.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            let x = x as isize;
            let y = y as isize;
            if *c == '*' {
                let mut numbers = vec![];
                add_number(&mut numbers, data, x - 1, y);
                add_number(&mut numbers, data, x + 1, y);
                if y > 0 {
                    if data[(y - 1) as usize][x as usize].is_digit(10) {
                        add_number(&mut numbers, data, x, y - 1);
                    } else {
                        add_number(&mut numbers, data, x - 1, y - 1);
                        add_number(&mut numbers, data, x + 1, y - 1);
                    }
                }
                if y < data.len() as isize - 1 {
                    if data[(y + 1) as usize][x as usize].is_digit(10) {
                        add_number(&mut numbers, data, x, y + 1);
                    } else {
                        add_number(&mut numbers, data, x - 1, y + 1);
                        add_number(&mut numbers, data, x + 1, y + 1);
                    }
                }
                if numbers.len() == 2 {
                    sum += numbers[0] * numbers[1];
                }
            }
        }
    }
    sum
}
