#! /usr/bin/env nix-shell
/*
#! nix-shell -p rust-script cargo -i rust-script
*/

use std::collections::VecDeque;
use std::io::stdin;
use std::process::exit;

// Contains the id of each node that it connects to
#[derive(Debug, Clone)]
struct Edge(char, char);

// Holds the state. Which node we are currently on and the unvisited edges.
#[derive(Debug)]
struct State {
    cur_node: char,
    edges: Vec<Edge>,
    // node ids in order of visitation
    history: Vec<char>,
}

fn main() {
    let edges: Vec<_> = stdin()
        .lines()
        .filter_map(|l| {
            let line = l.ok()?;
            let (a, b) = line.trim().split_once(' ')?;
            Some(Edge(a.chars().next()?, b.chars().next()?))
        })
        .collect();

    let mut nodes: Vec<_> = edges.iter().flat_map(|Edge(a, b)| [a, b]).collect();
    nodes.sort();
    nodes.dedup();

    let mut to_solve: VecDeque<_> = nodes
        .into_iter()
        .copied()
        .map(|n| State {
            cur_node: n,
            edges: edges.clone(),
            history: vec![n],
        })
        .collect();

    let mut done = false;
    let mut solution = Vec::new();

    while !done {
        if to_solve.is_empty() {
            break;
        }

        let state = to_solve.pop_front().unwrap();

        let new_states: Vec<_> = state
            .edges
            .iter()
            // Find edges adjacent to this node
            .filter(|Edge(a, b)| *a == state.cur_node || *b == state.cur_node)
            .map(|Edge(a, b)| {
                // Find other end of edge
                let next_node = if *a == state.cur_node { b } else { a };

                let mut edges = state.edges.clone();
                let mut history = state.history.clone();

                // remove current edge from unvisited
                edges.retain(|Edge(aa, bb)| !(a == aa && b == bb));
                // add current node to history
                history.push(*next_node);

                if edges.is_empty() {
                    done = true;
                    solution = history.clone();
                }

                State {
                    cur_node: *next_node,
                    edges,
                    history,
                }
            })
            .collect();

        if !new_states.is_empty() {
            // add new states to queue
            to_solve.extend(new_states);
        }
    }

    if solution.is_empty() {
        println!("Couldn't find solution");
        exit(1);
    } else {
        println!("Solution:");
        for c in solution {
            println!("         {c}");
        }
    }
}
