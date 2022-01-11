// AoC 2021 day 5

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: u16,
    y: u16,
}

impl Point {
    fn new(s: &str) -> Self {
        let mut iter = s.split(',');
        let x = iter.next().unwrap().parse::<u16>().unwrap();
        let y = iter.next().unwrap().parse::<u16>().unwrap();
        Self { x, y }
    }
}

fn main() -> std::io::Result<()> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();
    let mut lines: Vec<(Point, Point)> = vec![];

    while reader.read_line(&mut inputstr)? != 0 {
        let mut iter = inputstr.trim().split(" -> ");
        let from_pt = Point::new(iter.next().unwrap());
        let to_pt = Point::new(iter.next().unwrap());
        lines.push((from_pt, to_pt));

        inputstr.clear();
    }

    let mut hm: HashMap<Point, u16> = HashMap::new();

    // for part one only deal with horizontal or vertical lines
    for l in &lines {
        if l.0.x == l.1.x || l.0.y == l.1.y {
            if l.0.x == l.1.x {
                let ystart = std::cmp::min(l.0.y, l.1.y);
                let yend = std::cmp::max(l.0.y, l.1.y);
                for y in ystart..=yend {
                    let count = hm.entry(Point { x: l.0.x, y }).or_insert(0);
                    *count += 1;
                }
            } else {
                let xstart = std::cmp::min(l.0.x, l.1.x);
                let xend = std::cmp::max(l.0.x, l.1.x);
                for x in xstart..=xend {
                    let count = hm.entry(Point { x, y: l.0.y }).or_insert(0);
                    *count += 1;
                }
            }
        }
    }

    let ans = hm.values().filter(|val| *val >= &2).count();
    println!("aoc5a: {}", ans);

    // for part two add in diagonal lines
    for l in &lines {
        if l.0.x != l.1.x && l.0.y != l.1.y {
            let ystart = std::cmp::min(l.0.y, l.1.y);
            let yend = std::cmp::max(l.0.y, l.1.y);
            let (xstart, xend) = if ystart == l.0.y {
                (l.0.x, l.1.x)
            } else {
                (l.1.x, l.0.x)
            };
            let neg: bool = xstart > xend;
            for incr in 0..=yend - ystart {
                let count = hm
                    .entry(Point {
                        x: if neg { xstart - incr } else { xstart + incr },
                        y: ystart + incr,
                    })
                    .or_insert(0);
                *count += 1;
            }
        }
    }

    let ans = hm.values().filter(|val| *val >= &2).count();
    println!("aoc5b: {}", ans);

    Ok(())
}
