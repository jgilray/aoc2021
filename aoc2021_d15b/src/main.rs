// AoC 2021 day 15 (a faster way to solve the problem, using petgraph)
// performance: d15a: 8.9s, d15b: 0.15s

use petgraph::algo::dijkstra;
use petgraph::graph::{Graph, NodeIndex};

// returns the path length (total risk) from upper left to lower right of v using petgraph
fn shortest_path(v: &[Vec<u16>]) -> u16 {
    let mut cave_graph: Graph<(usize, usize), u16> = Graph::default();
    let mut entrance: NodeIndex = NodeIndex::default();
    let mut target: NodeIndex = NodeIndex::default();

    // add Nodes to graph.  Use nv to keep track of the NodeIndexs - there is no 
    // function such as cave_graph.get_nodeindex((y, x))
    let mut nv: Vec<Vec<NodeIndex>> = vec![];
    for y in 0..v.len() {
        let mut nrow: Vec<NodeIndex> = vec![];
        for x in 0..v[0].len() {
            let node = cave_graph.add_node((x, y));
            if x == 0 && y == 0 {
                entrance = node;
            } else if x == v[0].len() - 1 && y == v.len() - 1 {
                target = node;
            }
            nrow.push(node)
        }
        nv.push(nrow);
    }

    // add edges to graph
    for y in 0..v.len() {
        for x in 0..v[0].len() {
            let ymin = y.saturating_sub(1);
            let ymax = if y >= v.len() - 1 { y } else { y + 1 };
            let xmin = x.saturating_sub(1);
            let xmax = if x >= v.len() - 1 { x } else { x + 1 };
            for yy in ymin..=ymax {
                for xx in xmin..=xmax {
                    // angled routes are not allowed
                    if xx == x && yy != y || xx != x && yy == y {
                        cave_graph.update_edge(nv[yy][xx], nv[y][x], v[y][x]);
                    }
                }
            }
        }
    }

    let res = dijkstra(&cave_graph, entrance, None, |e| *e.weight());
    let ans = res.get(&target).unwrap();
    *ans
}

fn main() -> std::io::Result<()> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();
    let mut cave: Vec<Vec<u16>> = vec![]; // 2-d risk array from input
    let mut large_cave: Vec<Vec<u16>> = vec![]; // 5x 2-d risk array from input

    while reader.read_line(&mut inputstr)? != 0 {
        let mut row: Vec<u16> = vec![];
        for c in inputstr.trim().chars() {
            row.push((c as u8 - b'0') as u16);
        }
        cave.push(row.clone());

        // for part two
        for incr in 0..4 {
            for n in &cave[cave.len() - 1] {
                row.push((n + incr) % 9 + 1);
            }
        }
        large_cave.push(row);

        inputstr.clear();
    }

    // for part two
    let lcave = large_cave.clone();
    for incr in 0..4 {
        for row in &lcave {
            let mut lrow: Vec<u16> = vec![];
            for n in row {
                lrow.push((n + incr) % 9 + 1);
            }
            large_cave.push(lrow);
        }
    }

    println!("aoc15a: {}", shortest_path(&cave));
    println!("aoc15b: {}", shortest_path(&large_cave));

    Ok(())
}
