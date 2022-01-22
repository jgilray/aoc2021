// AoC 2021 day 12

use std::collections::HashSet;

#[derive(Debug)]
struct Node {
    name: String,
    neighbors: Vec<usize>,
    reenterable: bool,
}

impl Node {
    fn new(n: &str) -> Self {
        let c1 = n.chars().next().unwrap() as u8;
        let reenterable = (b'A'..=b'Z').contains(&c1);
        Self {
            name: n.to_owned(),
            neighbors: vec![],
            reenterable,
        }
    }
}

// if s is a the name of a Node that already is in nv will return that Node's index
// or adds a new Node and returns its index
fn add_node(s: &str, nv: &mut Vec<Node>) -> usize {
    for (i, n) in nv.iter().enumerate() {
        if n.name == s {
            return i;
        }
    }
    nv.push(Node::new(s));

    nv.len() - 1
}

// returns the number of times the passed path contains the passed idx
fn times_contained(path: &[usize], idx: usize) -> usize {
    let mut retval = 0;
    for p in path {
        if *p == idx {
            retval += 1;
        }
    }

    retval
}

fn main() -> std::io::Result<()> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();
    let mut nodes: Vec<Node> = vec![];

    while reader.read_line(&mut inputstr)? != 0 {
        let mut first_idx = usize::MAX;
        inputstr.trim().split('-').for_each(|s| {
            let idx = add_node(s, &mut nodes);
            if first_idx != usize::MAX {
                if first_idx == idx {
                    panic!("self-connected node: {}", s);
                } else {
                    if nodes[first_idx].neighbors.contains(&idx) {
                        panic!("Nodes already connected: {:?}", (first_idx, idx));
                    }
                    nodes[first_idx].neighbors.push(idx);
                    if nodes[idx].neighbors.contains(&first_idx) {
                        panic!("Nodes already connected: {:?}", (idx, first_idx));
                    }
                    nodes[idx].neighbors.push(first_idx);
                }
            } else {
                first_idx = idx;
            }
        });
        inputstr.clear();
    }

    // pre load paths with the "start" node
    let mut paths: Vec<Vec<usize>> = vec![vec![add_node("start", &mut nodes)]];
    let end_idx = add_node("end", &mut nodes);
    let mut count = 0;

    // part one: BFS from "start" to "end" nodes, counting the number of paths
    while !paths.is_empty() {
        let path = paths.remove(0);
        for n_idx in &nodes[path[path.len() - 1]].neighbors {
            if *n_idx == end_idx {
                count += 1;
            } else if nodes[*n_idx].reenterable || !path.contains(n_idx) {
                let mut newpath = path.clone();
                newpath.push(*n_idx);
                paths.push(newpath);
            }
        }
    }
    println!("aoc12a: {}", count);

    // part two: allow one reentry for each small cave in turn building a HashSet of paths
    let mut hs: HashSet<Vec<usize>> = HashSet::new();
    let mut smallcaves: Vec<usize> = vec![];
    for (i, n) in nodes.iter().enumerate() {
        let c1 = n.name.chars().next().unwrap() as u8;
        if (b'a'..=b'z').contains(&c1) && n.name != "start" && n.name != "end" {
            smallcaves.push(i);
        }
    }

    for sc_idx in &smallcaves {
        paths = vec![vec![add_node("start", &mut nodes)]];
        while !paths.is_empty() {
            let path = paths.remove(0);
            for n_idx in &nodes[path[path.len() - 1]].neighbors {
                if *n_idx == end_idx {
                    hs.insert(path.clone());
                } else if nodes[*n_idx].reenterable
                    || !path.contains(n_idx)
                    || n_idx == sc_idx && times_contained(&path, *n_idx) == 1
                {
                    let mut newpath = path.clone();
                    newpath.push(*n_idx);
                    paths.push(newpath);
                }
            }
        }
    }
    println!("aoc12b: {}", hs.len());

    Ok(())
}
