// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::{HashMap, HashSet};

type InputType = (String, Vec<String>);
type SolutionType = usize;

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    input
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(": ").expect(": ");
            let to: Vec<_> = to.split_ascii_whitespace().map(|s| s.to_string()).collect();
            (from.to_owned(), to)
        })
        .collect()
}

fn make_edges<'a>(data: &'a [InputType], cuts: &[&str]) -> HashMap<&'a str, Vec<&'a str>> {
    let mut edges: HashMap<&str, Vec<&str>> = HashMap::new();
    for (from_node, nodes) in data.iter() {
        'nodes: for to_node in nodes {
            for i in 0..cuts.len() / 2 {
                if from_node == cuts[i * 2] && to_node == cuts[i * 2 + 1] {
                    // println!("Cutting from {} to {}", cuts[i*2 + 0], cuts[i*2 + 1]);
                    continue 'nodes;
                }
            }
            if let Some(v) = edges.get_mut(&from_node.as_str()) {
                v.push(to_node);
            } else {
                edges.insert(from_node, Vec::from([to_node.as_str()]));
            }
            if let Some(v) = edges.get_mut(&to_node.as_str()) {
                v.push(from_node);
            } else {
                edges.insert(to_node, Vec::from([from_node.as_str()]));
            }
        }
    }
    edges
}

fn count_size(edges: &HashMap<&str, Vec<&str>>, start: &str) -> (SolutionType, SolutionType) {
    let mut expanded = HashSet::new();
    let mut frontier = Vec::from([start]);
    while let Some(edge) = frontier.pop() {
        if expanded.insert(edge) {
            for &edge in edges.get(edge).expect("Member") {
                frontier.push(edge);
            }
        }
    }
    let n_in = expanded.len();
    let n_out = edges.keys().count() - n_in;
    (n_in, n_out)
}

/*

fn _find_mostly_external<'a>(
    edges: &'a HashMap<&str, Vec<&str>>,
    a: &HashSet<&str>,
) -> Option<&'a str> {
    let mut record_node = None;
    let mut record_cost = 0;
    for &node in a.iter() {
        let mut internal = -1;
        let mut external = -1;
        for other in edges.get(&node).expect("nodes").iter() {
            if a.contains(other) {
                internal += 1;
            } else {
                external += 1;
            }
        }
        if external > internal {
            // Just to get the right lifetime:
            let node = edges.get_key_value(node).expect("Node");
            return Some(node.0);
        }
        let cost = external - internal;
        if cost > record_cost {
            record_cost = cost;
            record_node = Some(node);
        }
    }
    if record_node.is_some() {
        let node = edges.get_key_value(record_node.unwrap()).expect("Node");
        return Some(node.0);
    } else {
        None
    }
}

fn calc_d<'a>(
    out: &mut HashMap<&'a str, i32>,
    a: &'a HashSet<&str>,
    edges: &HashMap<&str, Vec<&str>>,
) {
    for &edge in a {
        let mut d = 0;
        for &other in edges.get(&edge).expect("other edge") {
            if a.contains(&other) {
                d -= 1;
            } else {
                d += 1;
            }
        }
        out.insert(edge, d);
    }
}

fn c(a: &str, b: &str, edges: &HashMap<&str, Vec<&str>>) -> i32 {
    let nodes = edges.get(a).expect("a node");
    for &node in nodes {
        if node == b {
            return 1;
        }
    }
    0
}

fn kernighan_lin(edges: &HashMap<&str, Vec<&str>>) -> (SolutionType, SolutionType) {
    let _nodes : HashMap<_, _> = edges
        .iter()
        .enumerate()
        .map(|(num, node)| (*node.0, num))
        .collect();

    let edges = edges.

    /* Initial partition: */
    let mut a = HashSet::new();
    let mut b = HashSet::new();
    for (i, &node) in edges.keys().enumerate() {
        if i % 2 == 0 {
            a.insert(node);
        } else {
            b.insert(node);
        }
    }

    // loop {
        let mut d_a = HashMap::new();
        let mut d_b = HashMap::new();
        calc_d(&mut d_a, &a, &edges);
        calc_d(&mut d_b, &b, &edges);
        let mut g = 0;
        let mut from_a = "none";
        let mut from_b = "none";
        for &a_node in &a {
            for &b_node in &b {
                let val = d_a.get(a_node).unwrap() + d_b.get(b_node).unwrap()
                    - 2 * c(a_node, b_node, &edges);
                if val > g {
                    g = val;
                    from_a = a_node;
                    from_b = b_node;
                }
            }
        }
    // }
    (0, 0)
}
*/

#[aoc(day25, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    /*
    let edges = make_edges(data, &[]);

    let (n_in, n_out) = kernighan_lin(&edges);
    dbg!(&(n_in, n_out));
    n_in * n_out
    */

    // TODO: Find cuts

    // Cuts found by visualizing with xdot...
    let cuts = Vec::from(["nvg", "vfj", "sqh", "jbz", "fch", "fvh"]);
    let edges = make_edges(data, &cuts);
    let start = cuts[0];

    /*
    // Make graphviz graph:
    println!();
    println!("strict graph {{");
    for (from, nodes) in edges {
        for node in nodes {
            println!("{} -- {}[tooltip=\"{} to {}\"]", from, node, from, node);
        }
    }
    println!("}}");
    println!();
    */

    let (n_in, n_out) = count_size(&edges, start);
    // dbg!(&(n_in, n_out));

    n_in * n_out
}
