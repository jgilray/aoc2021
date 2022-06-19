// AoC 2021 day 19
//
// I just assumed that the problem was correct about 12 common points being enough to align.
// Here is a paper that might provide deeper explanation if you can understand it:
// https://igl.ethz.ch/projects/ARAP/svd_rot.pdf
//
// The approach below is find alignment on x before trying y and z.  There are probably faster
// ways but this does the job for me in 0.3 seconds.

use std::collections::HashSet;

// 3-D location
type Point = [i16; 3];

fn manhattan_dist(a: &Point, b: &Point) -> i16 {
    (a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs()
}

#[derive(Debug, Clone)]
struct Xform {
    xyz_perm: [usize; 3],
    offset: [i16; 3],
    flips: [bool; 3],
}

#[derive(Debug)]
struct Scanner {
    points: Vec<Point>,
    xform: Xform,
}

impl Scanner {
    fn new() -> Self {
        Self {
            points: vec![],
            xform: Xform {
                xyz_perm: [0, 1, 2],
                offset: [0, 0, 0],
                flips: [false, false, false],
            },
        }
    }

    // transform one of the scanner's points based on its xform
    fn xform_point(&self, i: usize) -> Point {
        transformed(&self.points[i], &self.xform)
    }
}

// function that returns the passed p as a new, transformed point
fn transformed(p: &Point, xform: &Xform) -> Point {
    let mut point = [0; 3];

    for i in 0..3 {
        point[i] = p[xform.xyz_perm[i]];
        point[i] *= if xform.flips[i] { -1 } else { 1 };
        point[i] += xform.offset[i];
    }

    point
}

// assuming that the two passed scanners are aligned on x at the passed points,
// complete the alignment if possible, returning the Xform
fn yz_align(
    s1: &Scanner,
    s2: &Scanner,
    s1_idx: usize,
    s2_idx: usize,
    xform: &Xform,
) -> Option<Xform> {
    // Since x is set, there are only two permutations to try
    let perms = [
        xform.xyz_perm,
        [xform.xyz_perm[0], xform.xyz_perm[2], xform.xyz_perm[1]],
    ];

    // There are four flips to try
    let flips = [
        xform.flips,
        [xform.flips[0], true, false],
        [xform.flips[0], false, true],
        [xform.flips[0], true, true],
    ];

    let s1_trans_p = s1.xform_point(s1_idx);

    let mut h: HashSet<Point> = HashSet::new();
    for i in 0..s1.points.len() {
        h.insert(s1.xform_point(i));
    }

    for cand_perm in perms.iter() {
        for cand_flip in flips.iter() {
            let mut tot_xform = Xform {
                xyz_perm: *cand_perm,
                offset: xform.offset,
                flips: *cand_flip,
            };
            let s2_trans_p = transformed(&s2.points[s2_idx], &tot_xform);

            // update y and z offsets
            tot_xform.offset[1] = s1_trans_p[1] - s2_trans_p[1];
            tot_xform.offset[2] = s1_trans_p[2] - s2_trans_p[2];

            // count matches when using tot_xform
            let mut aligned = 0;
            for (idx, s2_p) in s2.points.iter().enumerate() {
                let cand = transformed(s2_p, &tot_xform);
                if h.contains(&cand) {
                    aligned += 1;
                }
                if aligned + s2.points.len() - idx < 12 {
                    break; // impossible to reach 12 aligned points
                }
            }

            if aligned >= 12 {
                return Some(tot_xform);
            }
        }
    }

    None
}

// If can align scanner s1 with with scanner s2 return the Xform that does the job, else return None
fn align(s1: &Scanner, s2: &Scanner) -> Option<Xform> {
    // Hash all the transformed x values to speed up alignment checking
    let mut xhash: HashSet<i16> = HashSet::new();
    for s1_p in &s1.points {
        let s1_trans_p = transformed(s1_p, &s1.xform);
        xhash.insert(s1_trans_p[0]);
    }

    // first align only on x coordinates
    for perm in [[0, 1, 2], [1, 2, 0], [2, 0, 1]].iter() {
        for flip in [[false, false, false], [true, false, false]].iter() {
            for (s2_idx, s2_p) in s2.points.iter().enumerate() {
                if s2.points.len() - s2_idx < 12 {
                    break; // alignment of s1 and s2 not possible
                }
                for (s1_idx, s1_p) in s1.points.iter().enumerate() {
                    if s1.points.len() - s1_idx < 12 {
                        break; // alignment of s1 and s2 not possible
                    }

                    // count the number of points that align with current transformation
                    let mut xform = Xform {
                        xyz_perm: *perm,
                        offset: [0, 0, 0],
                        flips: *flip,
                    };
                    let s1_trans_p = transformed(s1_p, &s1.xform);
                    let s2_trans_p = transformed(s2_p, &xform);

                    let x_offset = s1_trans_p[0] - s2_trans_p[0];
                    xform.offset = [x_offset, 0, 0];

                    let mut num_aligned = 0;
                    for (idx, s2_cand_p) in s2.points.iter().enumerate() {
                        let cand_p = transformed(s2_cand_p, &xform);
                        if xhash.contains(&cand_p[0]) {
                            num_aligned += 1;
                        }

                        if num_aligned + s2.points.len() - idx < 12 {
                            break; // impossible to reach 12 aligned points
                        }
                    }

                    if num_aligned >= 12 {
                        if let Some(tot_xform) = yz_align(s1, s2, s1_idx, s2_idx, &xform) {
                            return Some(tot_xform);
                        }
                    }
                }
            }
        }
    }
    None
}

// function that aligns all the scanners then returns
// (number-of-beacons, largest-manhattan-distance-between-scanners)
fn calculate(vs: &mut Vec<Scanner>) -> Result<(usize, i16), String> {
    // start working on 0th scanner arbitrarily setting it's origin at 0, 0, 0
    let mut dfs = vec![0];
    let mut origins: Vec<Point> = vec![[0, 0, 0]];

    // aligned keeps track of which scanners are aligned
    let mut aligned: HashSet<usize> = HashSet::new();
    aligned.insert(0);

    while !dfs.is_empty() {
        let cand = dfs.pop().unwrap();
        for i in 0..vs.len() {
            if !aligned.contains(&i) {
                if let Some(xform) = align(&vs[cand], &vs[i]) {
                    vs[i].xform = xform.clone();
                    origins.push(transformed(&[0, 0, 0], &xform));
                    dfs.push(i);
                    aligned.insert(i);
                }
            }
        }
    }

    if aligned.len() != vs.len() {
        return Err("Failed to aligned all scanners".to_string());
    }

    // find all unique beacons
    let mut unique_beacons: HashSet<Point> = HashSet::new();
    for scanner in vs {
        for i in 0..scanner.points.len() {
            unique_beacons.insert(scanner.xform_point(i));
        }
    }

    // find the max distance between scanners
    let mut max_dist = 0;
    for i in 0..origins.len() {
        for j in i..origins.len() {
            let dist = manhattan_dist(&origins[i], &origins[j]);
            if dist > max_dist {
                max_dist = dist;
            }
        }
    }

    Ok((unique_beacons.len(), max_dist))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();
    let mut vs: Vec<Scanner> = vec![];
    let mut idx: usize = 0;

    while reader.read_line(&mut inputstr)? != 0 {
        let loc = inputstr.trim();
        if loc == format!("--- scanner {} ---", vs.len()) {
            vs.push(Scanner::new());
            idx = vs.len() - 1;
        } else if !loc.is_empty() {
            let v = loc
                .split(',')
                .map(|s| s.parse::<i16>().unwrap())
                .collect::<Vec<_>>();
            vs[idx].points.push([v[0], v[1], v[2]]);
        }
        inputstr.clear();
    }

    let (num_beacons, max_dist) = calculate(&mut vs)?;
    println!("aoc19a: {}, aoc19b: {}", num_beacons, max_dist);

    Ok(())
}
