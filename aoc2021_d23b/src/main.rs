// AoC 2021 day 23
//
// I did this one manually (pencil and paper), but wanted to have some placeholder code so
// so I adapted the following from code that was on Reddit

use std::collections::HashMap;

// location on the map
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(y: i32, x: i32) -> Self {
        Self { y, x }
    }
}

// height of the rooms
const HEIGHT: i32 = 4;

// types of Amphipods
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Type {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Type {
    // must be sorted from greatest depth to least depth
    fn sorted_pos(&self) -> Vec<Point> {
        match self {
            Type::Amber => (0..HEIGHT).rev().map(|n| Point::new(2 + n, 3)).collect(),
            Type::Bronze => (0..HEIGHT).rev().map(|n| Point::new(2 + n, 5)).collect(),
            Type::Copper => (0..HEIGHT).rev().map(|n| Point::new(2 + n, 7)).collect(),
            Type::Desert => (0..HEIGHT).rev().map(|n| Point::new(2 + n, 9)).collect(),
        }
    }

    fn cost(&self) -> i32 {
        match self {
            Type::Amber => 1,
            Type::Bronze => 10,
            Type::Copper => 100,
            Type::Desert => 1000,
        }
    }
}

// state of an Amphipod - with color and location
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Amphipod {
    typ: Type,
    pos: Point,
}

// all the hallway locations where an Amphipod can rest
fn all_hallway_eps() -> Vec<Point> {
    vec![
        Point::new(1, 1),
        Point::new(1, 2),
        Point::new(1, 4),
        Point::new(1, 6),
        Point::new(1, 8),
        Point::new(1, 10),
        Point::new(1, 11),
    ]
}

fn is_hallway_space(pos: &Point) -> bool {
    pos.y == 1 && 1 <= pos.x && pos.x <= 11
}

fn has_path_vert(src_row: i32, dst_row: i32, col: i32, occupied: &HashMap<Point, Type>) -> bool {
    let mut row = src_row;
    let inc = (dst_row - src_row).signum();
    while row != dst_row {
        row += inc;
        if occupied.contains_key(&Point::new(row, col)) {
            return false;
        }
    }
    true
}

fn has_path_horz(src_col: i32, dst_col: i32, row: i32, occupied: &HashMap<Point, Type>) -> bool {
    let mut col = src_col;
    let inc = (dst_col - src_col).signum();
    while col != dst_col {
        col += inc;
        if occupied.contains_key(&Point::new(row, col)) {
            return false;
        }
    }
    true
}

fn get_distance(src: &Point, dst: &Point) -> i32 {
    // room-to-room path
    if src.y > 1 && dst.y > 1 {
        return manhattan(src, &Point::new(1, src.x)) + manhattan(&Point::new(1, src.x), dst);
    }

    manhattan(src, dst)
}

// returns true if there is a path from src to dst
fn has_path(src: &Point, dst: &Point, occupied: &HashMap<Point, Type>) -> bool {
    if src == dst {
        return true;
    }

    // room-to-room path
    if src.y > 1 && dst.y > 1 {
        return has_path_vert(src.y, 1, src.x, occupied)
            && has_path_horz(src.x, dst.x, 1, occupied)
            && has_path_vert(1, dst.y, dst.x, occupied);
    }

    // hallway-to-hallway path
    if src.y == 1 && dst.y == 1 {
        return has_path_horz(src.x, dst.x, 1, occupied);
    }

    // hallway-to-room path
    if src.y == 1 {
        return has_path_horz(src.x, dst.x, 1, occupied)
            && has_path_vert(1, dst.y, dst.x, occupied);
    }

    // room-to-hallway path
    has_path_vert(src.y, 1, src.x, occupied) && has_path_horz(src.x, dst.x, 1, occupied)
}

// given and Amphipod and a HashMap of where all Amphipods are, 
// this function figures out where the passed Amphipod intends to go
fn get_intended_position(apod: &Amphipod, map: &HashMap<Point, Type>) -> Point {
    for dst in apod.typ.sorted_pos() {
        if dst == apod.pos {
            return dst;
        }

        match map.get(&dst) {
            None => return dst,
            Some(&other) => {
                if other != apod.typ {
                    return dst;
                }
            }
        }
    }

    unreachable!()
}

// returns a vector of (Amphipod id number, location it intends to go to)
fn get_next_moves(src: &[Amphipod]) -> Vec<(usize, Point)> {
    let mut res = Vec::new();

    // map is a hash of Amphipod locations to types
    let map = src.iter().fold(HashMap::new(), |mut accum, a| {
        accum.insert(a.pos, a.typ);
        accum
    });

    for (i, apod) in src.iter().enumerate() {
        // println!("getting next moves for {:?}", apod);
        let intended_pos = get_intended_position(apod, &map);
        if intended_pos == apod.pos {
            continue;
        }

        if has_path(&apod.pos, &intended_pos, &map) {
            res.push((i, intended_pos));
            continue;
        }

        if is_hallway_space(&apod.pos) {
            continue;
        }

        // apod is in the incorrect room, can't move to its intended room, and
        // therefore can only move into the hallway.
        for dst in all_hallway_eps() {
            if !has_path(&apod.pos, &dst, &map) {
                continue;
            }
            res.push((i, dst));
        }
    }

    res
}

fn is_finished(state: &[Amphipod]) -> bool {
    state.iter().all(|a| a.typ.sorted_pos().contains(&a.pos))
}

fn manhattan(src: &Point, dst: &Point) -> i32 {
    (dst.x - src.x).abs() + (dst.y - src.y).abs()
}

fn parse(src: &str) -> Vec<Amphipod> {
    let mut res = Vec::new();
    for (i, line) in src.trim().lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                'A' => res.push(Amphipod {
                    typ: Type::Amber,
                    pos: Point::new(i as i32, j as i32),
                }),
                'B' => res.push(Amphipod {
                    typ: Type::Bronze,
                    pos: Point::new(i as i32, j as i32),
                }),
                'C' => res.push(Amphipod {
                    typ: Type::Copper,
                    pos: Point::new(i as i32, j as i32),
                }),
                'D' => res.push(Amphipod {
                    typ: Type::Desert,
                    pos: Point::new(i as i32, j as i32),
                }),
                _ => (),
            }
        }
    }

    res.sort();
    res
}

// recursive function that finds the best cost to reach the finished state
// cache is a HashMap of a state vector to the cost to reach that state
fn best_cost(state: &Vec<Amphipod>, cost: i32, cache: &mut HashMap<Vec<Amphipod>, i32>) -> i32 {
    if is_finished(state) {
        return cost;
    }

    let mut best = i32::MAX;

    // DFS where cache is used to make sure that only the lowest cost next_states are saved
    for (id, next_pos) in get_next_moves(state) {
        let apod = state[id];
        let mut next_state = state.clone();
        next_state[id] = Amphipod {
            typ: apod.typ,
            pos: next_pos,
        };
        next_state.sort();

        let next_cost = cost + get_distance(&apod.pos, &next_pos) * apod.typ.cost();
        if let Some(&prev_cost) = cache.get(&next_state) {
            if prev_cost <= next_cost {
                continue;
            }
        }

        cache.insert(next_state.clone(), next_cost);
        best = best.min(best_cost(&next_state, next_cost, cache));
    }

    best
}

fn main() {
    let src = r"
#############
#...........#
###D#A#C#C###
  #D#C#B#A#
  #D#B#A#C#
  #D#A#B#B#
  #########";

    let start = parse(src);
    println!("{}", best_cost(&start, 0, &mut HashMap::new()));
}
