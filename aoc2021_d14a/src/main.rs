// AoC 2021 day 14

use std::collections::HashMap;

// simulate a single set of polymer insertions, returning the next polymer iteration as a vector
// note that in the vector we're just keeping track of the rules used
fn sim(
    pv: &[usize],                         // incoming polymer
    rv: &[(u8, u8, u8)],                  // rules
    rhm: &HashMap<(u8, u8), (u8, usize)>, // rules hash map
    ec: &mut Vec<usize>,                  // element counts
) -> Vec<usize> {
    let mut new_polyvec: Vec<usize> = vec![0; rv.len()];
    for (i, x) in pv.iter().enumerate() {
        if *x > 0 {
            ec[rv[i].1 as usize] += x;
            if let Some((_, idx)) = rhm.get(&(rv[i].0, rv[i].1)) {
                new_polyvec[*idx] += x;
            }
            if let Some((_, idx)) = rhm.get(&(rv[i].1, rv[i].2)) {
                new_polyvec[*idx] += x;
            }
        }
    }
    new_polyvec
}

fn main() -> std::io::Result<()> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();
    let mut collecting_polymer = true;
    let mut rule_count = 0_usize;
    let mut polymer: Vec<u8> = vec![]; // initial polymer
    let mut elem_counts: Vec<usize> = vec![0; 26];
    let mut ruleshm: HashMap<(u8, u8), (u8, usize)> = HashMap::new(); // (left, right), (middle, index)
    let mut rulesvec: Vec<(u8, u8, u8)> = vec![]; // middle

    while reader.read_line(&mut inputstr)? != 0 {
        if inputstr.trim() == "" {
            collecting_polymer = false;
        } else if collecting_polymer {
            for c in inputstr.trim().chars() {
                let elem = c as u8 - b'A';
                polymer.push(elem);
                elem_counts[elem as usize] += 1;
            }
        } else {
            let instr = inputstr.trim().to_string();
            let mut split = instr.split(" -> ");
            let mut lside = split.next().unwrap().chars();
            let middle = split.next().unwrap().chars().last().unwrap() as u8 - b'A';
            let left = lside.next().unwrap() as u8 - b'A';
            let right = lside.next().unwrap() as u8 - b'A';
            ruleshm.insert((left, right), (middle, rule_count));
            rulesvec.push((left, middle, right));
            rule_count += 1;
        }

        inputstr.clear();
    }

    // create a vector that represents the polymer as the number of times each rule is used
    let mut polyvec: Vec<usize> = vec![0; rule_count];
    let mut left = &polymer[0];
    for right in polymer.iter().skip(1) {
        if let Some((_, idx)) = ruleshm.get(&(*left, *right)) {
            polyvec[*idx] += 1;
        }
        left = right;
    }

    // simulate 10 steps of pair insertions for part one
    for _ in 0..10 {
        polyvec = sim(&polyvec, &rulesvec, &ruleshm, &mut elem_counts);
    }

    let ans =
        elem_counts.iter().max().unwrap() - elem_counts.iter().filter(|&&c| c > 0).min().unwrap();
    println!("aoc14a: {}", ans);

    // simulate 30 more steps of pair insertions for part two
    for _ in 10..40 {
        polyvec = sim(&polyvec, &rulesvec, &ruleshm, &mut elem_counts);
    }

    let ans =
        elem_counts.iter().max().unwrap() - elem_counts.iter().filter(|&&c| c > 0).min().unwrap();
    println!("aoc14b: {}", ans);

    Ok(())
}
