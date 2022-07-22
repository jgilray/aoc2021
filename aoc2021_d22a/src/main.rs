// AoC 2021 day 22
//
// part 1 done brute force with a hashset
// part 2 algorithm:
//   keep a vector of "on" cuboids (ons)
//   for each new cuboid, find the intersection with each of the cuboids in ons
//   remove the intersecting region, breaking the block in ons into (up to) 6 new cuboids
//   if the new block is "on" then add it to ons

use regex::Regex;
use std::cmp;
use std::collections::HashSet;

// custom error type
#[derive(Debug)]
struct Error {
    details: String,
}

impl Error {
    fn new(msg: &str) -> Error {
        Error {
            details: msg.to_string(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.details
    }
}

// cuboid struct
#[derive(Debug, Clone, Copy)]
struct Cuboid {
    x: i32, // min corner
    y: i32,
    z: i32,
    xm: i32, // max corner
    ym: i32,
    zm: i32,
}

impl Cuboid {
    fn new(x: i32, y: i32, z: i32, xm: i32, ym: i32, zm: i32) -> Cuboid {
        Cuboid {
            x,
            y,
            z,
            xm,
            ym,
            zm,
        }
    }

    fn intersect(&self, c: &Cuboid) -> bool {
        let xlow: i32 = cmp::max(self.x, c.x);
        let xhigh: i32 = cmp::min(self.xm, c.xm);
        let ylow: i32 = cmp::max(self.y, c.y);
        let yhigh: i32 = cmp::min(self.ym, c.ym);
        let zlow: i32 = cmp::max(self.z, c.z);
        let zhigh: i32 = cmp::min(self.zm, c.zm);
        xhigh >= xlow && yhigh >= ylow && zhigh >= zlow
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();
    let mut hs: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut ons: Vec<Cuboid> = vec![];
    let re =
        Regex::new(r"^([[:alpha:]]+) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)")
            .unwrap();

    while reader.read_line(&mut inputstr)? != 0 {
        if re.is_match(&inputstr) {
            let caps = re.captures(&inputstr).unwrap();
            let on_off = caps.get(1).map_or("bad command", |m| m.as_str());
            let x1 = caps
                .get(2)
                .map_or(0, |m| m.as_str().parse::<i32>().unwrap());
            let x2 = caps
                .get(3)
                .map_or(0, |m| m.as_str().parse::<i32>().unwrap());
            let y1 = caps
                .get(4)
                .map_or(0, |m| m.as_str().parse::<i32>().unwrap());
            let y2 = caps
                .get(5)
                .map_or(0, |m| m.as_str().parse::<i32>().unwrap());
            let z1 = caps
                .get(6)
                .map_or(0, |m| m.as_str().parse::<i32>().unwrap());
            let z2 = caps
                .get(7)
                .map_or(0, |m| m.as_str().parse::<i32>().unwrap());

            // part 1, limited to the "initialization" region of the reactor
            let xlower = if x1 < -50 { -50 } else { x1 };
            let xupper = if x2 > 50 { 50 } else { x2 };
            let ylower = if y1 < -50 { -50 } else { y1 };
            let yupper = if y2 > 50 { 50 } else { y2 };
            let zlower = if z1 < -50 { -50 } else { z1 };
            let zupper = if z2 > 50 { 50 } else { z2 };

            if xlower <= xupper && ylower <= yupper && zlower <= zupper {
                for x in xlower..=xupper {
                    for y in ylower..=yupper {
                        for z in zlower..=zupper {
                            if on_off == "on" {
                                hs.insert((x, y, z));
                            } else if on_off == "off" {
                                if hs.contains(&(x, y, z)) {
                                    hs.remove(&(x, y, z));
                                }
                            } else {
                                return Err(Box::new(Error::new("Error: bad command line found")));
                            }
                        }
                    }
                }
            }

            // part 2 (see algorithm above)
            let cur = Cuboid::new(x1, y1, z1, x2, y2, z2);
            let mut next_ons: Vec<Cuboid> = vec![];
            //dbg!(ons.clone());
            for c in &ons {
                if !c.intersect(&cur) {
                    // put c back on list
                    next_ons.push(*c);
                } else {
                    let mut oc = *c;
                    // axis-by-axis, create sub-blocks of c, x-axis first
                    if oc.x <= cur.xm && cur.xm <= oc.xm {
                        next_ons.push(Cuboid::new(cur.xm + 1, oc.y, oc.z, oc.xm, oc.ym, oc.zm));
                        oc = Cuboid::new(oc.x, oc.y, oc.z, cur.xm, oc.ym, oc.zm);
                    }
                    if oc.x <= cur.x && cur.x <= oc.xm {
                        next_ons.push(Cuboid::new(oc.x, oc.y, oc.z, cur.x - 1, oc.ym, oc.zm));
                        oc = Cuboid::new(cur.x, oc.y, oc.z, oc.xm, oc.ym, oc.zm);
                    }

                    // y-axis
                    if oc.y <= cur.ym && cur.ym <= oc.ym {
                        next_ons.push(Cuboid::new(oc.x, cur.ym + 1, oc.z, oc.xm, oc.ym, oc.zm));
                        oc = Cuboid::new(oc.x, oc.y, oc.z, oc.xm, cur.ym, oc.zm);
                    }
                    if oc.y <= cur.y && cur.y <= oc.ym {
                        next_ons.push(Cuboid::new(oc.x, oc.y, oc.z, oc.xm, cur.y - 1, oc.zm));
                        oc = Cuboid::new(oc.x, cur.y, oc.z, oc.xm, oc.ym, oc.zm);
                    }

                    // z-axis
                    if oc.z <= cur.zm && cur.zm <= oc.zm {
                        next_ons.push(Cuboid::new(oc.x, oc.y, cur.zm + 1, oc.xm, oc.ym, oc.zm));
                        oc = Cuboid::new(oc.x, oc.y, oc.z, oc.xm, oc.ym, cur.zm);
                    }
                    if oc.z <= cur.z && cur.z <= oc.zm {
                        next_ons.push(Cuboid::new(oc.x, oc.y, oc.z, oc.xm, oc.ym, cur.z - 1));
                    }
                }
            }

            // insert cuboid if on and reset ons list
            if on_off == "on" {
                next_ons.push(cur);
            }
            ons = next_ons;
        }
        inputstr.clear();
    }

    // count volume of cuboids in ons list
    let mut volume = 0;
    for c in &ons {
        volume += (c.xm - c.x + 1) as u64 * (c.ym - c.y + 1) as u64 * (c.zm - c.z + 1) as u64;
    }

    println!("aoc22a: {}, aoc22b: {}", hs.len(), volume);

    Ok(())
}
