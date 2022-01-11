// AoC 2021 day 8

use std::collections::HashMap;

// function that returns a String -> u32 HashMap decoding the passed inputs
fn decode_inputs(v: &[Vec<char>]) -> HashMap<Vec<char>, u32> {
    let mut retval: HashMap<Vec<char>, u32> = HashMap::new();
    let mut vc: Vec<Vec<char>> = vec![vec![]; 10];

    // first get those unique by length
    for s in v {
        let mut ss = s.clone();
        ss.sort_unstable();
        match s.len() {
            2 => {
                retval.insert(ss, 1);
                vc[1] = s.clone();
            }
            3 => {
                retval.insert(ss, 7);
                vc[7] = s.clone();
            }
            4 => {
                retval.insert(ss, 4);
                vc[4] = s.clone();
            }
            7 => {
                retval.insert(ss, 8);
                vc[8] = s.clone();
            }
            _ => {}
        }
    }

    // figure out those of length 5 and 6 based upon those already determined above
    for s in v {
        let mut ss = s.clone();
        ss.sort_unstable();
        match s.len() {
            5 => {
                let seg1count = vc[1].iter().filter(|seg| s.contains(seg)).count();
                if seg1count == 2 {
                    retval.insert(ss, 3);
                } else {
                    let seg4count = vc[4].iter().filter(|seg| s.contains(seg)).count();
                    if seg4count == 2 {
                        retval.insert(ss, 2);
                    } else {
                        retval.insert(ss, 5);
                    }
                }
            }
            6 => {
                let seg1count = vc[1].iter().filter(|seg| s.contains(seg)).count();
                if seg1count == 1 {
                    retval.insert(ss, 6);
                } else {
                    let seg4count = vc[4].iter().filter(|seg| s.contains(seg)).count();
                    if seg4count == 3 {
                        retval.insert(ss, 0);
                    } else {
                        retval.insert(ss, 9);
                    }
                }
            }
            _ => {}
        }
    }

    retval
}

fn main() -> std::io::Result<()> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();
    let mut count = 0; // for part one
    let mut sum = 0; // for part two

    while reader.read_line(&mut inputstr)? != 0 {
        let iter = inputstr.trim().split(" | ");
        let mut collecting_input = true;
        let mut decoder: HashMap<Vec<char>, u32> = HashMap::new();
        let mut val = 0;

        for s in iter {
            let sv = s
                .split(' ')
                .map(|s| s.chars().collect::<Vec<char>>())
                .collect::<Vec<_>>();

            if collecting_input {
                decoder = decode_inputs(&sv);
            } else {
                for out in &sv {
                    let mut nout = out.clone();
                    nout.sort_unstable();
                    let len = out.len();
                    if len == 2 || len == 3 || len == 4 || len == 7 {
                        count += 1;
                    }
                    val *= 10;
                    val += decoder.get(&nout).unwrap();
                }
            }

            collecting_input = false;
        }

        sum += val;
        inputstr.clear();
    }

    println!("aoc8a: {} aoc8b: {}", count, sum);

    Ok(())
}
