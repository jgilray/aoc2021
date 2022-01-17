// AoC 2021 day 11

// increment all locations, return true if they were all zeros prior to incrementing them
fn increment_all(v: &mut [Vec<u8>]) -> bool {
    let mut all_zeroes = true;
    for r in 0..v.len() {
        for c in 0..v[r].len() {
            v[r][c] += 1;

            if v[r][c] > 1 {
                all_zeroes = false;
            }
        }
    }

    all_zeroes
}

// make a pass through all octopuses counting the number that flash
fn flash(v: &mut [Vec<u8>]) -> usize {
    let mut retval = 0;
    for r in 0..v.len() {
        for c in 0..v[r].len() {
            if v[r][c] > 9 {
                retval += 1; // count the flash
                v[r][c] = 0;

                // propagate the flash
                let llr = r.saturating_sub(1);
                let ulr = if r < v.len() - 1 { r + 1 } else { r };
                let llc = c.saturating_sub(1);
                let ulc = if c < v[r].len() - 1 { c + 1 } else { c };
                for rr in llr..=ulr {
                    for cc in llc..=ulc {
                        if v[rr][cc] != 0 {
                            v[rr][cc] += 1;
                        }
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
    let mut vals: Vec<Vec<u8>> = vec![];

    while reader.read_line(&mut inputstr)? != 0 {
        let mut val: Vec<u8> = vec![];
        for c in inputstr.trim_end().chars() {
            val.push(c as u8 - b'0');
        }
        vals.push(val);

        inputstr.clear();
    }

    let mut total_flashes = 0;
    for step in 0.. {
        if step == 100 {
            println!("aoc11a: {}", total_flashes);
        }

        if increment_all(&mut vals) {
            println!("aoc11b: {}", step);
            break;
        }

        let mut new_flashes = flash(&mut vals);
        while new_flashes > 0 {
            total_flashes += new_flashes;
            new_flashes = flash(&mut vals);
        }
    }

    Ok(())
}
