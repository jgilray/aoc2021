// AoC 2021 day 13

use std::collections::HashSet;

struct Fold {
    loc: isize,
    x_fold: bool,
}

fn main() -> std::io::Result<()> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();
    let mut collecting_points = true;
    let mut hs: HashSet<(isize, isize)> = HashSet::new();
    let mut vf: Vec<Fold> = vec![];

    while reader.read_line(&mut inputstr)? != 0 {
        if inputstr.trim() == "" {
            collecting_points = false;
        } else if collecting_points {
            let instr = inputstr.trim().to_string();
            let mut split = instr.split(',');
            let x: isize = split.next().unwrap().parse().expect("bad x value");
            let y: isize = split.next().unwrap().parse().expect("bad y value");
            hs.insert((x, y));
        } else {
            let instr = inputstr.trim().to_string();
            let mut split = instr.split('=');
            let fold_type = split.next().unwrap().chars().last().expect("bad fold type");
            let loc: isize = split.next().unwrap().parse().expect("bad fold location");
            vf.push(Fold {
                loc,
                x_fold: fold_type == 'x',
            });
        }

        inputstr.clear();
    }

    // fold the paper
    let mut first_fold = true;
    for f in &vf {
        let mut new_hs: HashSet<(isize, isize)> = HashSet::new();
        for (x, y) in hs.iter() {
            if f.x_fold {
                let new_x = if *x > f.loc { 2 * f.loc - *x } else { *x };
                new_hs.insert((new_x, *y));
            } else {
                let new_y = if *y > f.loc { 2 * f.loc - *y } else { *y };
                new_hs.insert((*x, new_y));
            }
        }
        hs = new_hs;

        if first_fold {
            println!("aoc13a: {}", hs.len());
            first_fold = false;
        }
    }

    // after all the folds, display the paper
    let mut smallest_x = isize::MAX;
    let mut largest_x = 0_isize;
    let mut smallest_y = isize::MAX;
    let mut largest_y = 0_isize;

    for (x, y) in hs.iter() {
        if *x < smallest_x {
            smallest_x = *x;
        }
        if *x > largest_x {
            largest_x = *x;
        }
        if *y < smallest_y {
            smallest_y = *y;
        }
        if *y > largest_y {
            largest_y = *y;
        }
    }

    let xlim = (largest_x - smallest_x + 1) as usize;
    let ylim = (largest_y - smallest_y + 1) as usize;

    let mut v: Vec<Vec<char>> = vec![vec![' '; xlim]; ylim];
    for (x, y) in hs.iter() {
        v[(*y - smallest_y) as usize][(*x - smallest_x) as usize] = '#';
    }

    println!("aoc13b:");
    for s in &v {
        println!("{}", s.iter().collect::<String>());
    }

    Ok(())
}
