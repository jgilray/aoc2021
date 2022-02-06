// AoC 2021 day 15

use std::collections::VecDeque;

// the head of a path with total risk and position
struct State {
    cost: u16,
    x: usize,
    y: usize,
}

// function that generates a 2-d vector with the lowest risk total to get to each position
fn bfs_least_risk_path(cave: &[Vec<u16>]) -> Vec<Vec<u16>> {
    let mut retval = vec![vec![u16::MAX; cave[0].len()]; cave.len()];
    let mut fifo = VecDeque::new(); // for BFS

    // add starting position to the fifo
    fifo.push_back(State {
        cost: 0,
        x: 0,
        y: 0,
    });

    // process cave positions in breadth first search manner
    while !fifo.is_empty() {
        let pos = fifo.pop_front().unwrap();
        let ymin = pos.y.saturating_sub(1);
        let ymax = if pos.y >= cave.len() - 1 {
            pos.y
        } else {
            pos.y + 1
        };
        let xmin = pos.x.saturating_sub(1);
        let xmax = if pos.x >= cave.len() - 1 {
            pos.x
        } else {
            pos.x + 1
        };
        for y in ymin..=ymax {
            for x in xmin..=xmax {
                // angled routes are not allowed
                if x == pos.x && y != pos.y || x != pos.x && y == pos.y {
                    let risk = pos.cost + cave[y][x];
                    if risk < retval[y][x] {
                        // found a better path to (x, y), keep going
                        retval[y][x] = risk;
                        fifo.push_back(State { cost: risk, x, y });
                    }
                }
            }
        }
    }

    retval
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

    let path_lengths = bfs_least_risk_path(&cave);
    println!(
        "aoc15a: {}",
        path_lengths[cave.len() - 1][cave[0].len() - 1]
    );

    let path_lengths = bfs_least_risk_path(&large_cave);
    println!(
        "aoc15b: {}",
        path_lengths[large_cave.len() - 1][large_cave[0].len() - 1]
    );

    Ok(())
}
