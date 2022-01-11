// AoC 2021 day 9

// function that returns a Vector of (x, y, val) tuples of the neighbors of the passed coordinate
fn neighbors(x: usize, y: usize, v: &[Vec<u8>]) -> Vec<(usize, usize, u8)> {
    let mut retval: Vec<(usize, usize, u8)> = vec![];

    // build list of "manhattan" neighbors
    let yl = y.saturating_sub(1);
    let yh = if y == v.len() - 1 { y } else { y + 1 };
    let xl = x.saturating_sub(1);
    let xh = if x == v[y].len() - 1 { x } else { x + 1 };
    for yy in yl..=yh {
        for xx in xl..=xh {
            if xx == x || yy == y {
                if xx == x && yy == y {
                    continue;
                } else {
                    retval.push((xx, yy, v[yy][xx]));
                }
            }
        }
    }

    retval
}

fn main() -> std::io::Result<()> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();
    let mut floor: Vec<Vec<u8>> = vec![];

    while reader.read_line(&mut inputstr)? != 0 {
        let mut row: Vec<u8> = vec![];
        for c in inputstr.trim().chars() {
            row.push(c as u8 - b'0');
        }
        floor.push(row);

        inputstr.clear();
    }

    let mut basin_vec: Vec<Vec<(usize, usize)>> = vec![]; // for part two

    // part one - find low points and score their risk
    let mut risk_sum = 0_u32;
    for y in 0..floor.len() {
        for x in 0..floor[y].len() {
            let mut tot = 0;
            let mut lower = 0;
            for (_, _, val) in neighbors(x, y, &floor) {
                tot += 1;
                if floor[y][x] < val {
                    lower += 1;
                } else {
                    break;
                }
            }
            if lower == tot {
                risk_sum += floor[y][x] as u32 + 1;

                // prepare for part two
                basin_vec.push(vec![(x, y)]);
            }
        }
    }

    // part two, basin_size is a vector of the size of each basin
    let mut basin_size: Vec<usize> = vec![];

    // basin_vec contains a vector of the lowest point of each basin
    for bv in basin_vec.iter_mut() {
        let mut basin: Vec<(usize, usize)> = vec![];

        // DFS to build each entire basin in basin
        while !bv.is_empty() {
            let (x, y) = bv.pop().unwrap();
            if !basin.contains(&(x, y)) {
                basin.push((x, y));
                for (xx, yy, val) in neighbors(x, y, &floor) {
                    if val < 9 && !basin.contains(&(xx, yy)) {
                        bv.push((xx, yy));
                    }
                }
            }
        }
        basin_size.push(basin.len());
    }

    basin_size.sort_unstable();
    basin_size.reverse();
    println!(
        "aoc9a: {} aoc9b: {}",
        risk_sum,
        basin_size[0] * basin_size[1] * basin_size[2]
    );

    Ok(())
}
