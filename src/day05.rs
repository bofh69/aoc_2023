// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};
// use rayon::prelude::*;
use std::collections::HashSet;
use std::ops::Range;

type Seed = i64;

pub struct MappingData {
    from_kind: String,
    to_kind: String,
    ranges: Vec<(Seed, Seed, Seed)>,
}

type InputType = (Vec<Seed>, Vec<MappingData>);
type SolutionType = Seed;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> InputType {
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    let mut lines = input.lines();

    let seeds = lines
        .next()
        .expect("Seeds line")
        .split_once("seeds: ")
        .expect("Seeds")
        .1
        .split_ascii_whitespace()
        .map(|num| num.parse().expect("Seed number"))
        .collect();

    lines.next().expect("empty line");

    let mut mappings = vec![];

    while let Some(mapping) = lines.next() {
        let names = mapping.split_once(" map:").expect("map").0;
        let mut names = names.split('-');
        let from_kind = names.next().expect("from name").to_string();
        names.next();
        let to_kind = names.next().expect("to name").to_string();
        let mut ranges = vec![];
        for range in lines.by_ref() {
            if range.is_empty() {
                break;
            }
            let mut range = range.split_ascii_whitespace();
            let from_range = range
                .next()
                .expect("from range")
                .parse()
                .expect("From range");
            let to_range = range.next().expect("to range").parse().expect("To range");
            let length = range.next().expect("length").parse().expect("Range length");
            ranges.push((from_range, to_range, length));
        }
        mappings.push(MappingData {
            from_kind,
            to_kind,
            ranges,
        });
    }

    (seeds, mappings)
}

fn translate_number(num: Seed, kind: &str, final_kind: &str, data: &InputType) -> SolutionType {
    let mut num = num;
    let mut kind = kind;

    // println!("\n-------");

    while kind != final_kind {
        for trans in &data.1 {
            if trans.from_kind == kind {
                // println!("\nTranslating from {} {}", trans.from_kind, num);
                // let old_num = num;
                kind = trans.to_kind.as_str();
                for range in &trans.ranges {
                    // println!( "Range dest={}, src={}, length={}", range.0, range.1, range.2);
                    if range.1 <= num && (range.1 + range.2) > num {
                        num = num - range.1 + range.0;
                        break;
                    }
                }
                // println!("Translated from {} {} to {} {}", trans.from_kind, old_num, kind, num);
            }
        }
    }
    num
}

#[aoc(day5, part1)]
pub fn solve_part1(data: &InputType) -> SolutionType {
    data.0
        .iter()
        .map(|seed| translate_number(*seed, "seed", "location", data))
        .min()
        .expect("Minimum number")
}

fn translate_range_with_mapping(
    translated: &mut Vec<Range<Seed>>,
    untranslated: &mut HashSet<Range<Seed>>,
    range: Range<Seed>,
    mapping: &(Seed, Seed, Seed),
) {
    if range.end <= mapping.1 || range.start >= mapping.1 + mapping.2 {
        // Outside range
        untranslated.insert(range);
    } else if range.start < mapping.1 {
        // Starts before
        untranslated.insert(range.start..mapping.1);
        if range.end <= mapping.1 + mapping.2 {
            // Ends inside
            translated.push(mapping.0..(mapping.0 + (range.end - mapping.1)));
        } else {
            // Ends after
            translated.push(mapping.0..(mapping.0 + mapping.2));
            untranslated.insert((mapping.1 + mapping.2)..range.end);
        }
    } else {
        // Starts inside
        if range.end <= mapping.1 + mapping.2 {
            // Ends inside
            translated
                .push((mapping.0 + range.start - mapping.1)..(mapping.0 + (range.end - mapping.1)));
        } else {
            // Ends after
            translated.push((mapping.0 + range.start - mapping.1)..(mapping.0 + mapping.2));
            untranslated.insert((mapping.1 + mapping.2)..range.end);
        }
    }
}

fn translate_range_with_mappings(
    range: Range<Seed>,
    mappings: &Vec<(Seed, Seed, Seed)>,
) -> Vec<Range<Seed>> {
    let mut untranslated = HashSet::from([range]);
    let mut translated = vec![];
    for mapping in mappings {
        let mut next_untranslated = HashSet::new();
        for range in untranslated {
            translate_range_with_mapping(
                &mut translated,
                &mut next_untranslated,
                range.clone(),
                mapping,
            );
        }
        untranslated = next_untranslated;
    }
    // Put untranslated in translated
    for range in untranslated {
        translated.push(range);
    }
    translated
}

fn translate_range(
    range: Range<Seed>,
    mappings: &[MappingData],
    kind: &str,
    final_kind: &str,
) -> Vec<Range<Seed>> {
    let mut kind = kind;
    let mut ranges = vec![range];
    while kind != final_kind {
        for trans in mappings.iter() {
            if trans.from_kind == kind {
                kind = trans.to_kind.as_str();
                let mut next_ranges = HashSet::new();
                for range in ranges {
                    for new_range in translate_range_with_mappings(range, &trans.ranges).iter() {
                        next_ranges.insert(new_range.clone());
                    }
                }
                ranges = vec![];
                for range in next_ranges {
                    ranges.push(range);
                }
            }
        }
    }
    ranges
}

#[aoc(day5, part2)]
pub fn solve_part2(data: &InputType) -> SolutionType {
    // Should really translate ranges
    let mut numbers = data.0.iter();
    let mut location = SolutionType::MAX;
    while let Some(start) = numbers.next() {
        let size = numbers.next().expect("seed size");
        let range = *start..(start + size);
        let new_range = translate_range(range, &data.1, "seed", "location");
        for range in new_range {
            if range.start < location {
                location = range.start;
            }
        }
    }
    location
}

#[cfg(test)]
mod test {
    use super::translate_range_with_mapping;
    use super::translate_range_with_mappings;
    use std::collections::HashSet;

    #[test]
    fn translate_range_before() {
        let mut untranslated = HashSet::new();
        let mut translated = vec![];
        let range = 30..40;
        let mapping = (20, 40, 10);
        translate_range_with_mapping(&mut translated, &mut untranslated, range.clone(), &mapping);
        assert!(translated.is_empty());
        assert_eq!(untranslated, HashSet::from([range]));
    }

    #[test]
    fn translate_range_after() {
        let mut untranslated = HashSet::new();
        let mut translated = vec![];
        let range = 50..60;
        let mapping = (20, 40, 10);
        translate_range_with_mapping(&mut translated, &mut untranslated, range.clone(), &mapping);
        assert!(translated.is_empty());
        assert_eq!(untranslated, HashSet::from([range]));
    }

    #[test]
    fn translate_range_starts_before_ends_inside() {
        let mut untranslated = HashSet::new();
        let mut translated = vec![];
        let range = 35..45;
        let mapping = (20, 40, 10);
        translate_range_with_mapping(&mut translated, &mut untranslated, range.clone(), &mapping);
        assert_eq!(untranslated, HashSet::from([35..40]));
        assert_eq!(translated, vec![20..25]);
    }

    #[test]
    fn translate_range_starts_before_ends_after() {
        let mut untranslated = HashSet::new();
        let mut translated = vec![];
        let range = 35..55;
        let mapping = (20, 40, 10);
        translate_range_with_mapping(&mut translated, &mut untranslated, range.clone(), &mapping);
        assert_eq!(untranslated, HashSet::from([35..40, 50..55]));
        assert_eq!(translated, vec![20..30]);
    }

    #[test]
    fn translate_range_starts_inside_ends_after() {
        let mut untranslated = HashSet::new();
        let mut translated = vec![];
        let range = 45..55;
        let mapping = (20, 40, 10);
        translate_range_with_mapping(&mut translated, &mut untranslated, range.clone(), &mapping);
        assert_eq!(untranslated, HashSet::from([50..55]));
        assert_eq!(translated, vec![25..30]);
    }

    #[test]
    fn translate_range_is_inside() {
        let mut untranslated = HashSet::new();
        let mut translated = vec![];
        let range = 42..48;
        let mapping = (20, 40, 10);
        translate_range_with_mapping(&mut translated, &mut untranslated, range.clone(), &mapping);
        assert!(untranslated.is_empty());
        assert_eq!(translated, vec![22..28]);
    }

    #[test]
    fn translate_range_with_mappings_inside() {
        let range = 82..83;
        let mappings = vec![(50, 98, 2), (52, 50, 48)];
        let result = translate_range_with_mappings(range, &mappings);
        assert_eq!(result, vec![84..85]);
    }
}
